use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use futures::{sink::SinkExt, stream::StreamExt};
use rand::{rngs::OsRng, RngCore};
use serde::Deserialize;
use serde_json::{json, Value};
use crate::{errors::AppError, AppState};

const WS_TICKET_TTL_SECS: usize = 30;

/// Atomically fetch-and-delete: returns the value if the key existed, and
/// removes it in the same round-trip so a ticket can only ever be redeemed once.
const CONSUME_TICKET_SCRIPT: &str = r#"
local v = redis.call('GET', KEYS[1])
if v then redis.call('DEL', KEYS[1]) end
return v
"#;

/// GET /api/ws-ticket — mint a short-lived, single-use ticket for the WS handshake.
/// Browsers can't attach an Authorization header when opening a WebSocket, so the
/// admin token can't be passed directly without landing in the URL (and from there,
/// proxy/access logs and browser history). This endpoint sits behind
/// `require_admin_token` like the rest of the API, so only a caller who already
/// holds the real token can mint a ticket — and the ticket itself is worthless
/// after ~30s or after first use.
pub async fn issue_ticket(State(state): State<AppState>) -> Result<Json<Value>, AppError> {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    let ticket = bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>();

    let mut conn = state.redis_pool.get().await
        .map_err(|e| AppError::InternalError(format!("Redis unavailable: {e}")))?;

    deadpool_redis::redis::cmd("SET")
        .arg(format!("devpulse:ws_ticket:{}", ticket))
        .arg(1)
        .arg("EX")
        .arg(WS_TICKET_TTL_SECS)
        .query_async::<()>(&mut *conn)
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to store WS ticket: {e}")))?;

    Ok(Json(json!({ "ticket": ticket, "expires_in": WS_TICKET_TTL_SECS })))
}

#[derive(Deserialize)]
pub struct WsQuery {
    ticket: Option<String>,
}

/// WebSocket handler — validates a one-time ticket (minted via `issue_ticket`) via query param.
/// - In production (ADMIN_TOKEN set): requires ?ticket=<ticket-from-/api/ws-ticket>
/// - In development (ADMIN_TOKEN empty): allows unauthenticated access
pub async fn ws_handler(
    ws:            WebSocketUpgrade,
    Query(params): Query<WsQuery>,
    State(state):  State<AppState>,
) -> Response {
    let admin_token_set = std::env::var("ADMIN_TOKEN").ok().filter(|t| !t.is_empty()).is_some();

    if admin_token_set {
        let ticket = match params.ticket.as_deref() {
            Some(t) if !t.is_empty() => t.to_string(),
            _ => {
                tracing::warn!("WS: rejected connection — missing ticket");
                return (StatusCode::UNAUTHORIZED, "Missing ticket").into_response();
            }
        };

        let valid = match state.redis_pool.get().await {
            Ok(mut conn) => {
                deadpool_redis::redis::cmd("EVAL")
                    .arg(CONSUME_TICKET_SCRIPT)
                    .arg(1)
                    .arg(format!("devpulse:ws_ticket:{}", ticket))
                    .query_async::<Option<String>>(&mut *conn)
                    .await
                    .unwrap_or(None)
                    .is_some()
            }
            Err(_) => false,
        };

        if !valid {
            tracing::warn!("WS: rejected connection — invalid or expired ticket");
            return (StatusCode::UNAUTHORIZED, "Invalid or expired ticket").into_response();
        }
        tracing::debug!("WS: authenticated connection accepted");
    } else {
        tracing::warn!("WS: accepting unauthenticated connection (ADMIN_TOKEN not set — development mode)");
    }

    let redis_url = state.redis_url.clone();
    ws.on_upgrade(move |socket| handle_socket(socket, redis_url))
}

async fn handle_socket(socket: WebSocket, redis_url: String) {
    let (mut sender, mut receiver) = socket.split();

    // Create a dedicated Redis pub/sub connection (not from the pool)
    let client = match deadpool_redis::redis::Client::open(redis_url.as_str()) {
        Ok(c)  => c,
        Err(e) => {
            tracing::error!("WS: failed to open Redis client: {}", e);
            return;
        }
    };
    let mut pubsub = match client.get_async_pubsub().await {
        Ok(p)  => p,
        Err(e) => {
            tracing::error!("WS: failed to get pubsub connection: {}", e);
            return;
        }
    };
    if let Err(e) = pubsub.subscribe("devpulse:events").await {
        tracing::error!("WS: subscribe failed: {}", e);
        return;
    }
    let mut msg_stream = pubsub.into_on_message();

    tracing::info!("🔌 WebSocket client connected");

    let _ = sender.send(Message::Text(
        serde_json::json!({
            "type":    "connected",
            "message": "DevPulse live feed ready 🚀"
        }).to_string().into()
    )).await;

    loop {
        tokio::select! {
            // New event from Redis pub/sub → push to browser
            Some(redis_msg) = msg_stream.next() => {
                let payload: String = match redis_msg.get_payload() {
                    Ok(p)  => p,
                    Err(_) => continue,
                };
                if sender.send(Message::Text(payload.into())).await.is_err() {
                    break;
                }
            }

            // Message from browser
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Ping(p))) => {
                        let _ = sender.send(Message::Pong(p)).await;
                    }
                    Some(Ok(Message::Close(_))) => {
                        tracing::info!("🔌 WebSocket client disconnected (clean)");
                        break;
                    }
                    None => {
                        tracing::info!("🔌 WebSocket client disconnected (connection lost)");
                        break;
                    }
                    _ => continue,
                }
            }
        }
    }
}
