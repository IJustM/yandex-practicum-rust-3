use uuid::Uuid;

use crate::error::AppError;

pub fn to_uuid(value: &str) -> anyhow::Result<Uuid, AppError> {
    match Uuid::parse_str(value) {
        Ok(uuid) => Ok(uuid),
        Err(_) => Err(AppError::BadRequest("parse error to uuid".to_string())),
    }
}
