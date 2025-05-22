use axum::Json;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use tauri::{Emitter, Runtime};
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
    match state.app_handle.emit(&payload.event_name, payload.payload) {
        Ok(_) => EmitOutput {
            status: "ok".into(),
        },
        Err(err) => EmitOutput {
            status: "err".into(),
        },
    }
}
