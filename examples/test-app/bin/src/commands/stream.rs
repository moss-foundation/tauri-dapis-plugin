use axum::Json;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::{IntoResponse, Response};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::ipc::{Channel, InvokeResponseBody};
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamInput {
    pub upper_limit: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamEvent {
    pub current: i32,
}

impl IntoResponse for StreamEvent {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[tauri::command(async)]
pub async fn stream(input: StreamInput, channel: Channel<StreamEvent>) {
    stream_impl(input, channel).await
}

pub async fn stream_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_stream_socket)
}

async fn handle_stream_socket(mut socket: WebSocket) {
    let (mut sender, mut receiver) = socket.split();
    let sender = Arc::new(Mutex::new(sender));

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            dbg!(format!("Received message: {}", text));
            // Deserialize input json text into StreamInput type
            let payload = serde_json::from_str::<StreamInput>(&text);
            if let Ok(input) = payload {
                // Create a channel that will send the StreamEvent back to the websocket
                let sender = sender.clone();
                let channel = Channel::<StreamEvent>::new(move |event| {
                    dbg!(&event);
                    match event {
                        InvokeResponseBody::Json(json) => {
                            let sender = sender.clone();
                            tokio::spawn(async move {
                                let _ = sender.lock().await.send(Message::Text(json.into())).await;
                            });
                            Ok(())
                        }
                        InvokeResponseBody::Raw(raw) => {
                            dbg!(raw);
                            Ok(())
                        }
                    }
                });

                stream_impl(input, channel).await;
            }
        }
    }
}

async fn stream_impl(input: StreamInput, channel: Channel<StreamEvent>) {
    for i in 0..input.upper_limit {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        let _ = channel.send(StreamEvent { current: i });
        dbg!(format!("Sending {} on channel", i));
    }
}
