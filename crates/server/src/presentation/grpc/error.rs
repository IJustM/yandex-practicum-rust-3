use tonic::Status;

use crate::error::AppError;

impl From<AppError> for Status {
    fn from(app_error: AppError) -> Self {
        match app_error {
            AppError::BadRequest(message) => Status::invalid_argument(message),
            AppError::Conflict(message) => Status::already_exists(message),
            AppError::Unauthorized(message) => Status::unauthenticated(message),
            AppError::Internal(message) => Status::internal(message),
            AppError::Db => Status::internal("db error"),
        }
    }
}
