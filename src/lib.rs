pub mod listener;

use crate::listener::listen_handler;
use anyhow::Result;
use axum::Router;
use axum::handler::Handler;
use axum::routing::{MethodRouter, any};
use std::collections::HashMap;
use std::sync::mpsc::channel;
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Runtime};

const DEFAULT_PORT: usize = 12345;

pub struct AxumState<R: Runtime> {
    // This can be used to access state managed by tauri
    pub app_handle: AppHandle<R>,
    pub events: Vec<String>,
}

impl<R: Runtime> Clone for AxumState<R> {
    fn clone(&self) -> Self {
        Self {
            app_handle: self.app_handle.clone(),
            events: self.events.clone(),
        }
    }
}

pub struct Builder<R: Runtime> {
    routes: HashMap<String, MethodRouter<AxumState<R>>>,
    events: Vec<String>,
    port: usize,
}

impl<R: Runtime> Builder<R> {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            events: Vec::new(),
            port: DEFAULT_PORT,
        }
    }

    pub fn routes(
        self,
        routes: impl IntoIterator<Item = (String, MethodRouter<AxumState<R>>)>,
    ) -> Self {
        Self {
            routes: routes.into_iter().collect(),
            ..self
        }
    }

    pub fn events(self, events: impl IntoIterator<Item = String>) -> Self {
        Self {
            events: events.into_iter().collect(),
            ..self
        }
    }
    pub fn port(self, port: usize) -> Self {
        Self { port, ..self }
    }
}

impl<R: Runtime> Builder<R> {
    pub fn build(self) -> Result<TauriPlugin<R>> {
        Ok(tauri::plugin::Builder::<R>::new("tauri-plugin-dapis")
            .setup(move |app_handle, api| {
                let state: AxumState<R> = AxumState {
                    app_handle: (*app_handle).clone(),
                    events: self.events.clone(),
                };

                let mut axum_router = Router::new();
                for (path, router) in &self.routes {
                    axum_router = axum_router.route(path, router.to_owned());
                }
                // Register /{event_name} websocket endpoint to stream all listened events
                axum_router = axum_router.route("/{event_name}", any(listen_handler));

                let axum_router = axum_router.with_state(state);

                let (error_tx, error_rx) = channel();
                tokio::spawn(async move {
                    let listener =
                        tokio::net::TcpListener::bind(format!("127.0.0.1:{}", self.port)).await;

                    match listener {
                        Ok(listener) => {
                            let _ = error_tx.send(Ok(()));
                            axum::serve(listener, axum_router)
                                .await
                                .expect("axum::serve will never return an error")
                        }
                        Err(err) => {
                            let _ = error_tx.send(Err(err));
                        }
                    }
                });

                if let Err(e) = error_rx.recv() {
                    Err(e.into())
                } else {
                    Ok(())
                }
            })
            .build())
    }
}
