use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{domain::post::Post, error::AppError, infrastructure::jwt::AuthUser, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/posts", post(create))
        .route("/api/posts/{id}", get(get_by_id))
        .route("/api/posts/{id}", put(update))
        .route("/api/posts/{id}", delete(remove))
}

async fn create(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateRequest>,
) -> anyhow::Result<Json<PostResponse>, AppError> {
    let post = state
        .post_service
        .create(&payload.title, &payload.content, &claims.sub)
        .await?;

    Ok(Json(post.into()))
}

async fn get_by_id(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> anyhow::Result<Json<PostResponse>, AppError> {
    let post = state.post_service.get_by_id(&id).await?;

    Ok(Json(post.into()))
}

async fn update(
    AuthUser(claims): AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateRequest>,
) -> anyhow::Result<Json<PostResponse>, AppError> {
    let post = state
        .post_service
        .update(&id, &payload.title, &payload.content, &claims.sub)
        .await?;

    Ok(Json(post.into()))
}

async fn remove(
    AuthUser(claims): AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> anyhow::Result<(), AppError> {
    state.post_service.remove(&id, &claims.sub).await?;

    Ok(())
}

#[derive(Deserialize)]
struct CreateRequest {
    title: String,
    content: String,
}

#[derive(Deserialize)]
struct UpdateRequest {
    title: String,
    content: String,
}

#[derive(Serialize)]
struct PostResponse {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        let Post {
            id,
            author_id,
            title,
            content,
            created_at,
        } = post;

        PostResponse {
            id,
            author_id,
            title,
            content,
            created_at: created_at.unwrap_or_else(|| OffsetDateTime::now_utc()),
        }
    }
}
