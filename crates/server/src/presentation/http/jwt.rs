use axum::{extract::FromRequestParts, http::header};

use crate::{
    error::AppError,
    infrastructure::jwt::{Claims, verify_jwt},
    state::AppState,
};

pub struct AuthUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(header::AUTHORIZATION)
            .ok_or(AppError::Unauthorized(
                "missing authorization header".to_string(),
            ))?;

        let header = header
            .to_str()
            .map_err(|_| AppError::Unauthorized("invalid header".to_string()))?;

        let claims = verify_jwt(&state.config.jwt_secret, header)?;

        Ok(AuthUser(claims))
    }
}
