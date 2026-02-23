mod data;
mod domain;
mod error;
mod infrastructure;
mod presentation;
mod state;

use axum::{Router, serve};
use tokio::net::TcpListener;

use crate::{presentation::routes, state::AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    infrastructure::tracing::init_tracing();

    let config = infrastructure::config::Config::from_env().expect("invalid config");

    let pool = infrastructure::db::create_db(&config.database_url).await;

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("migrations failed");

    let addr = format!("{}:{}", config.host, config.port_http);
    tracing::info!("starting server on {}", addr);

    let listener = TcpListener::bind(addr).await.expect("bind listener error");

    let app = Router::<AppState>::new()
        .merge(routes::users::router())
        .layer(infrastructure::cors::cors(&config.cors_origin));

    let state = AppState { pool, config };
    let app = app.with_state(state);

    serve(listener, app).await.expect("serve error");

    Ok(())
}
