use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_dapis::AxumState;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetStateInput {
    state: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetStateOutput {
    old_state: String,
}

impl IntoResponse for SetStateOutput {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[tauri::command(async)]
pub async fn set_state<R: Runtime>(
    app_handle: AppHandle<R>,
    input: SetStateInput,
) -> SetStateOutput {
    set_state_impl(app_handle, input).await
}

pub async fn set_state_handler<R: Runtime>(
    State(state): State<AxumState<R>>,
    Json(payload): Json<SetStateInput>,
) -> SetStateOutput {
    set_state_impl(state.app_handle.clone(), payload).await
}

async fn set_state_impl<R: Runtime>(
    app_handle: AppHandle<R>,
    input: SetStateInput,
) -> SetStateOutput {
    let state = app_handle.state::<Mutex<String>>();
    let mut state_lock = state.lock().unwrap();
    let old_state = state_lock.clone();
    *state_lock = input.state;
    SetStateOutput { old_state }
}
