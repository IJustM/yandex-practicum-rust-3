mod application;
mod data;
mod domain;
mod error;
mod infrastructure;
mod presentation;
mod state;

use std::sync::Arc;

use axum::{Router, serve};
use tokio::net::TcpListener;

use crate::{
    application::{post_service::PostService, user_service::UserService},
    data::{post_repo::SqlxPostRepository, user_repo::SqlxUserRepository},
    presentation::http::{post_router, user_router},
    state::AppState,
};

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
        .merge(user_router::router())
        .merge(post_router::router())
        .layer(infrastructure::cors::cors(&config.cors_origin));

    let config = Arc::new(config);
    let user_service = Arc::new(UserService::new(SqlxUserRepository::new(pool.clone())));
    let post_service = Arc::new(PostService::new(SqlxPostRepository::new(pool.clone())));
    let state = AppState {
        config,
        user_service,
        post_service,
    };
    let app = app.with_state(state);

    serve(listener, app).await.expect("serve error");

    Ok(())
}
