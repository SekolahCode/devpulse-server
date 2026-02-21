use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use crate::AppState;

pub async fn ws_handler(
    ws:           WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.event_tx.subscribe();  // subscribe to broadcast

    tracing::info!("🔌 WebSocket client connected");

    // Welcome message
    let _ = sender.send(Message::Text(
        serde_json::json!({
            "type":    "connected",
            "message": "DevPulse live feed ready 🚀"
        }).to_string().into()
    )).await;

    loop {
        tokio::select! {
            // New event from worker → push to browser
            Ok(data) = rx.recv() => {
                if sender.send(Message::Text(data.into())).await.is_err() {
                    break;  // client disconnected
                }
            }

            // Message from browser
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Ping(p))) => {
                        let _ = sender.send(Message::Pong(p)).await;
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => continue,
                }
            }
        }
    }

    tracing::info!("🔌 WebSocket client disconnected");
}
