use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct EmptyResponse {}

#[derive(Deserialize)]
pub struct ServerError {
    pub message: String,
}

#[derive(Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
}

#[derive(Serialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct UpdatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct PostResponse {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct PostListResponse {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub posts: Vec<PostResponse>,
}
