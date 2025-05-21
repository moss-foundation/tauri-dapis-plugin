pub mod commands;
mod server;

use std::sync::Mutex;

use crate::commands::greet::greet;
use crate::commands::set_state::set_state;
use crate::commands::stream::stream;
use crate::server::start_server;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use tauri::Manager;

pub async fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(String::from("Initial State")));
            tokio::spawn(start_server(app.handle().clone()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, set_state, stream])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
