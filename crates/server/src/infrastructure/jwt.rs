use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
}

pub fn generate_jwt(secret: &str, user_id: &Uuid) -> anyhow::Result<String> {
    let now = OffsetDateTime::now_utc();
    let exp = now + Duration::hours(24);

    let claims = Claims {
        sub: *user_id,
        exp: exp.unix_timestamp(),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("encode error");

    Ok(token)
}

pub fn verify_jwt(secret: &str, header: &str) -> anyhow::Result<Claims, AppError> {
    if !header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized("invalid scheme".to_string()));
    }

    let token = &header[7..];

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized("invalid jwt token".to_string()))?;

    Ok(data.claims)
}
