use axum::{Json, Router, extract::State, routing::post};

use crate::{
    error::AppError,
    infrastructure::jwt,
    presentation::proto::blog::{AuthResponse, LoginRequest, RegisterRequest},
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> anyhow::Result<(), AppError> {
    state
        .user_service
        .register(&payload.email, &payload.password, &payload.username)
        .await?;

    Ok(())
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
