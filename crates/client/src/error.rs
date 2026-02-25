#[derive(Debug, thiserror::Error)]
pub enum BlogClientError {
    #[error("Internal: {0}")]
    Internal(String),

    #[error("Internal http: {0}")]
    InternalHttp(reqwest::Error),

    #[error("Internal grpc transport: {0}")]
    InternalRgpcTransport(tonic::transport::Error),

    #[error("Internal grpc status: {0}")]
    InternalRgpcStatus(tonic::Status),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}
