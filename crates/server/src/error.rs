pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    Conflict(String),
    Internal(String),
    Db,
}
