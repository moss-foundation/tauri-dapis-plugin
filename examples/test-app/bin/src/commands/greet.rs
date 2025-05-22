use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GreetInput {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GreetOutput {
    pub message: String,
}

impl IntoResponse for GreetOutput {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[tauri::command(async)]
pub async fn greet(input: GreetInput) -> GreetOutput {
    greet_impl(input).await
}

pub async fn greet_handler(Json(payload): Json<GreetInput>) -> GreetOutput {
    greet_impl(payload).await
}

async fn greet_impl(input: GreetInput) -> GreetOutput {
    GreetOutput {
        message: format!("Hello, {}! You've been greeted from Rust!", input.name),
    }
}
