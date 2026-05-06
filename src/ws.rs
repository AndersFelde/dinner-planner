use serde::{Deserialize, Serialize};

/// Broadcast message emitted whenever a day-ingredient's bought state changes.
/// Sent over WebSocket to all connected clients.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IngredientUpdate {
    pub day_id: i32,
    pub ingredient_id: i32,
    pub bought: bool,
}

#[cfg(feature = "ssr")]
pub mod server {
    use super::IngredientUpdate;
    use axum::extract::ws::{Message, WebSocket};
    use axum::extract::{Extension, WebSocketUpgrade};
    use axum::response::IntoResponse;
    use futures::{SinkExt, StreamExt};
    use tokio::sync::broadcast;

    /// Clone-able sender handle. Each clone sends to the same channel.
    pub type BroadcastTx = broadcast::Sender<IngredientUpdate>;

    pub fn create_channel() -> BroadcastTx {
        broadcast::channel(256).0
    }

    /// Axum handler — upgrades HTTP to WebSocket, then streams broadcast messages.
    pub async fn ws_handler(
        ws: WebSocketUpgrade,
        Extension(tx): Extension<BroadcastTx>,
    ) -> impl IntoResponse {
        ws.on_upgrade(move |socket| handle_socket(socket, tx))
    }

    async fn handle_socket(socket: WebSocket, tx: BroadcastTx) {
        let mut rx = tx.subscribe();
        let (mut sink, mut stream) = socket.split();

        loop {
            tokio::select! {
                // Forward broadcast updates to this client
                broadcast_msg = rx.recv() => {
                    match broadcast_msg {
                        Ok(update) => {
                            match serde_json::to_string(&update) {
                                Ok(json) => {
                                    if sink.send(Message::Text(json.into())).await.is_err() {
                                        break; // client disconnected
                                    }
                                }
                                Err(e) => {
                                    leptos::logging::error!("WS serialize error: {e}");
                                }
                            }
                        }
                        Err(broadcast::error::RecvError::Lagged(n)) => {
                            // Channel overflowed; skip missed messages and keep going
                            leptos::logging::warn!("WS receiver lagged by {n} messages");
                            continue;
                        }
                        Err(broadcast::error::RecvError::Closed) => break,
                    }
                }
                // Watch for client disconnect; we don't process client→server messages
                client_msg = stream.next() => {
                    if client_msg.is_none() {
                        break;
                    }
                    // Ignore any incoming client frames
                }
            }
        }

        let _ = sink.close().await;
    }
}
