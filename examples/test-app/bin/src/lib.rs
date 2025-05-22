pub mod commands;

use std::string::ToString;
use std::sync::Mutex;

use crate::commands::greet::{greet, greet_handler};
use crate::commands::set_state::{set_state, set_state_handler};
use crate::commands::stream::{stream, stream_handler};
use axum::response::IntoResponse;
use axum::routing::{any, post};
use serde::{Deserialize, Serialize};
use tauri::Manager;

pub async fn run() {
    let routes = vec![
        ("/greet".to_string(), post(greet_handler)),
        ("/set-state".to_string(), post(set_state_handler)),
        ("/stream".to_string(), any(stream_handler)),
    ];
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(String::from("Initial State")));
            Ok(())
        })
        .plugin(
            tauri_plugin_dapis::Builder::new()
                .routes(routes)
                .port(12345)
                .build()
                .expect("Unable to initialize tauri_plugin_dapis"),
        )
        .invoke_handler(tauri::generate_handler![greet, set_state, stream])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
