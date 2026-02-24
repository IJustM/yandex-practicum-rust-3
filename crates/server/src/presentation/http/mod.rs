pub mod error;
pub mod jwt;
pub mod post_router;
pub mod user_router;

use axum::{Router, serve};
use serde::Serialize;
use tokio::net::TcpListener;

use crate::{
    infrastructure::{config::Config, cors::cors},
    state::AppState,
};

pub async fn run_http(state: AppState) -> anyhow::Result<()> {
    let Config {
        host,
        port_http,
        cors_origin,
        ..
    } = state.config.as_ref();
    let addr = format!("{}:{}", host, port_http);
    tracing::info!("starting http server on {}", addr);

    let listener = TcpListener::bind(addr).await.expect("bind listener error");

    let app = Router::<AppState>::new()
        .merge(user_router::router())
        .merge(post_router::router())
        .layer(cors(cors_origin));

    let app = app.with_state(state);

    serve(listener, app).await.expect("serve error");

    Ok(())
}

#[derive(Serialize)]
pub struct EmptyResponse {}
