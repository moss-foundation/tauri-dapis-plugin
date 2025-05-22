use axum::Json;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{Listener, Runtime};
use tokio::sync::Mutex;

use crate::AxumState;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListenOutput {
    pub payload: String,
}

impl IntoResponse for ListenOutput {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

pub async fn listen_handler<R: Runtime>(
    ws: WebSocketUpgrade,
    Path(event_name): Path<String>,
    State(state): State<AxumState<R>>,
) -> Response {
    if state.events.contains(&event_name) {
        ws.on_upgrade(move |socket| handle_listen_socket(socket, state, event_name))
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

async fn handle_listen_socket<R: Runtime>(
    mut socket: WebSocket,
    state: AxumState<R>,
    event_name: String,
) {
    let (sender, mut receiver) = socket.split();
    let sender = Arc::new(Mutex::new(sender));

    // Start streaming the listened events to the websocket once a connection establishes
    let listener_id = state.app_handle.listen_any(event_name, move |event| {
        let output = ListenOutput {
            payload: event.payload().to_string(),
        };
        let json =
            serde_json::to_string_pretty(&output).expect("ListenOutput should be serializable");
        let sender = sender.clone();
        tokio::spawn(async move { sender.lock().await.send(Message::Text(json.into())).await });
    });

    while let Some(Ok(message)) = receiver.next().await {
        // Keep the connection alive
    }

    // Deregister the event listener when connection ends
    dbg!("Client disconnected, unlistening: ", listener_id);
    state.app_handle.unlisten(listener_id)
}
