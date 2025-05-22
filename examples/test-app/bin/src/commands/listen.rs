use axum::Json;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::{IntoResponse, Response};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{Listener, Runtime};
use tauri_plugin_dapis::AxumState;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]

// TODO: implement event-target filtering
pub struct ListenInput {
    pub event_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListenOutput {
    pub id: u32,
    pub payload: String,
}

impl IntoResponse for ListenOutput {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

pub async fn listen_handler<R: Runtime>(
    ws: WebSocketUpgrade,
    State(state): State<AxumState<R>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_listen_socket(socket, state))
}

async fn handle_listen_socket<R: Runtime>(mut socket: WebSocket, state: AxumState<R>) {
    let (sender, mut receiver) = socket.split();
    let sender = Arc::new(Mutex::new(sender));

    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(text) = message {
            dbg!(format!("Received message: {}", text));
            // Deserialize input json text into ListenInput type
            let payload = serde_json::from_str::<ListenInput>(&text);
            if let Ok(input) = payload {
                // Register an event listener that will forward the event to the websocket
                let sender = sender.clone();
                state.app_handle.listen_any(input.event_name, move |event| {
                    let output = ListenOutput {
                        id: event.id(),
                        payload: event.payload().to_string(),
                    };
                    let json = serde_json::to_string_pretty(&output).unwrap();
                    let sender = sender.clone();
                    tokio::spawn(async move {
                        let _ = sender.lock().await.send(Message::Text(json.into())).await;
                    });
                });
            }
        }
    }
}
