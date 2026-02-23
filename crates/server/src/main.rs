mod data;
mod domain;
mod presentation;

use axum::{Router, serve};
use server::{
    db::{create_config, create_db},
    infrastructure,
    state::AppState,
};
use tokio::net::TcpListener;

use crate::presentation::routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    infrastructure::tracing::init_tracing();

    let config = create_config();

    let pool = create_db(&config).await;

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("starting server on {}", addr);

    let listener = TcpListener::bind(addr).await.expect("bind listener error");

    let app = Router::<AppState>::new()
        .merge(routes::router())
        .layer(infrastructure::cors::cors(&config.cors_origin));

    let state = AppState { pool, config };
    let app = app.with_state(state);

    serve(listener, app).await.expect("serve error");

    Ok(())
}
