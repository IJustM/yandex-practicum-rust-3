use reqwest::Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlogClientError {
    #[error("Internal: {0}")]
    Internal(Error),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}
