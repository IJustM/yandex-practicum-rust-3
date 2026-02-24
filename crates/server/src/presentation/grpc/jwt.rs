use tonic::Request;

use crate::{
    error::AppError,
    infrastructure::jwt::{self, Claims},
    state::AppState,
};

pub fn get_claims<T>(state: &AppState, request: &Request<T>) -> anyhow::Result<Claims, AppError> {
    let res = request.metadata().get("authorization");

    match res {
        Some(header) => {
            let header = header.to_str().unwrap_or("");
            let claims = jwt::verify_jwt(&state.config.jwt_secret, header)?;
            Ok(claims)
        }
        None => Err(AppError::Unauthorized(
            "authorization key not found".to_string(),
        )),
    }
}
