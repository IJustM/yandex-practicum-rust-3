use axum::{Json, Router, extract::State, routing::post};
use server::{error::AppError, state::AppState};
use tracing::error;

use crate::{
    data::users_repo,
    infrastructure::security::{self},
    presentation::proto,
};

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<proto::blog::RegisterRequest>,
) -> anyhow::Result<(), AppError> {
    let email = payload.email.trim().to_lowercase();
    let username = payload.username.trim().to_string();
    let password = payload.password;
    if email.is_empty() || password.len() < 6 {
        return Err(AppError::BadRequest(
            "invalid email or password".to_string(),
        ));
    }

    tracing::info!("register user = {}", email);

    let user_id = uuid::Uuid::now_v7();
    let password_hash = security::hash_password(&password)
        .map_err(|_| AppError::Internal("hash error".to_string()))?;

    let res = users_repo::create_user(&state, &user_id, &email, &username, &password_hash).await;

    match res {
        Ok(_) => Ok(()),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            Err(AppError::Conflict("email already exist".to_string()))
        }
        Err(e) => {
            error!("SQL create_user error: {:?}", e);
            Err(AppError::Db)
        }
    }
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<proto::blog::LoginRequest>,
) -> anyhow::Result<Json<proto::blog::AuthResponse>, AppError> {
    let email = payload.email.trim().to_lowercase();
    let password = payload.password;

    let user = match users_repo::find_by_email(&state, &email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(AppError::Unauthorized("user not found".to_string()));
        }
        Err(_) => {
            return Err(AppError::Db);
        }
    };

    let ok = security::verify_password(&password, &user.password_hash)
        .map_err(|_| AppError::Internal("verify error".to_string()))?;

    if !ok {
        return Err(AppError::Unauthorized("not correct password".to_string()));
    }

    let access_token = security::generate_jwt(&state.config.jwt_secret, &user.id, &user.email)
        .map_err(|_| AppError::Internal("jwt error".to_string()))?;

    Ok(Json(proto::blog::AuthResponse { access_token }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
