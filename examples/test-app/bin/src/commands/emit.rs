use axum::Json;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Runtime};
use tauri_plugin_dapis::AxumState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmitInput {
    pub event_name: String,
    pub payload: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmitOutput {
    status: String,
}

impl IntoResponse for EmitOutput {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

pub async fn emit_handler<R: Runtime>(
    State(state): State<AxumState<R>>,
    Json(payload): Json<EmitInput>,
) -> EmitOutput {
    emit_impl(state.app_handle.clone(), payload).await
}

#[tauri::command(async)]
pub async fn emit<R: Runtime>(app_handle: AppHandle<R>, emit_input: EmitInput) -> EmitOutput {
    emit_impl(app_handle, emit_input).await
}

pub async fn emit_impl<R: Runtime>(app_handle: AppHandle<R>, emit_input: EmitInput) -> EmitOutput {
    match app_handle.emit(&emit_input.event_name, emit_input.payload) {
        Ok(_) => EmitOutput {
            status: "ok".into(),
        },
        Err(err) => EmitOutput {
            status: "err".into(),
        },
    }
}
