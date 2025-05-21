use crate::commands::greet::greet_handler;
use crate::commands::set_state::set_state_handler;
use crate::commands::stream::stream_handler;
use axum::Router;
use axum::routing::{any, post};
use tauri::AppHandle;

#[derive(Clone)]
pub struct AxumState {
    pub app_handle: AppHandle,
}

pub async fn start_server(app_handle: AppHandle) {
    let state = AxumState { app_handle };

    let router = Router::new()
        .route("/greet", post(greet_handler))
        .route("/set-state", post(set_state_handler))
        .route("/stream", any(stream_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    dbg!(listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
