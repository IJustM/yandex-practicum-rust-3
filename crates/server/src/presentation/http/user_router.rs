use axum::{Json, Router, extract::State, routing::post};
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError, infrastructure::jwt, presentation::http::EmptyResponse, state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> anyhow::Result<Json<EmptyResponse>, AppError> {
    state
        .user_service
        .register(&payload.email, &payload.password, &payload.username)
        .await?;

    Ok(Json(EmptyResponse {}))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> anyhow::Result<Json<AuthResponse>, AppError> {
    let user = state
        .user_service
        .login(&payload.email, &payload.password)
        .await?;

    let access_token = jwt::generate_jwt(&state.config.jwt_secret, &user.id)
        .map_err(|_| AppError::Internal("jwt error".to_string()))?;

    Ok(Json(AuthResponse { access_token }))
}

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
    username: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    access_token: String,
}
