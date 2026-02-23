mod application;
mod data;
mod domain;
mod error;
mod infrastructure;
mod presentation;
mod state;

use std::sync::Arc;

use crate::{
    application::{post_service::PostService, user_service::UserService},
    data::{post_repo::SqlxPostRepository, user_repo::SqlxUserRepository},
    presentation::http::run_http,
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

    let config = Arc::new(config);
    let user_service = Arc::new(UserService::new(SqlxUserRepository::new(pool.clone())));
    let post_service = Arc::new(PostService::new(SqlxPostRepository::new(pool.clone())));
    let state = AppState {
        config,
        user_service,
        post_service,
    };

    tokio::select! {
        res = run_http(state.clone()) => {
            if let Err(e) = res {
                tracing::error!("http failed: {}", e);
                return Err(e);
            }
        },
    };

    Ok(())
}
