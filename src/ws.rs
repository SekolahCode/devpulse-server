use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use crate::AppState;

#[derive(Deserialize)]
pub struct WsQuery {
    token: Option<String>,
}

/// WebSocket handler — requires the same ADMIN_TOKEN as the REST API.
/// Clients pass it as a query param: /ws?token=<ADMIN_TOKEN>
pub async fn ws_handler(
    ws:            WebSocketUpgrade,
    Query(params): Query<WsQuery>,
    State(state):  State<AppState>,
) -> Response {
    // Validate token
    let expected = match std::env::var("ADMIN_TOKEN").ok().filter(|t| !t.is_empty()) {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, "Server has no ADMIN_TOKEN configured").into_response(),
    };

    let provided = params.token.unwrap_or_default();
    if provided != expected {
        return (StatusCode::UNAUTHORIZED, "Invalid or missing token").into_response();
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
