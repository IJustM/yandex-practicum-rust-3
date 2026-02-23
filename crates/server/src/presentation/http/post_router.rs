use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post, put},
};
use uuid::Uuid;

use crate::{
    domain,
    error::AppError,
    infrastructure::jwt::AuthUser,
    presentation::proto::{
        blog::{CreatePostRequest, Post},
        parse_proto_timestamp,
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/post", post(create))
        .route("/api/post/{id}", get(get_by_id))
        .route("/api/post/{id}", put(update))
}

async fn create(
    AuthUser(claims): AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> anyhow::Result<Json<Post>, AppError> {
    let post = state
        .post_service
        .create(&payload.title, &payload.content, &claims.sub)
        .await?;

    Ok(Json(post.into()))
}

async fn get_by_id(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> anyhow::Result<Json<Post>, AppError> {
    let post = state.post_service.get_by_id(&id).await?;

    Ok(Json(post.into()))
}

async fn update(
    AuthUser(claims): AuthUser,
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> anyhow::Result<Json<Post>, AppError> {
    let post = state
        .post_service
        .update(&id, &payload.title, &payload.content, &claims.sub)
        .await?;

    Ok(Json(post.into()))
}

impl From<domain::post::Post> for Post {
    fn from(post: domain::post::Post) -> Self {
        Post {
            id: post.id.to_string(),
            author_id: post.author_id.to_string(),
            title: post.title,
            content: post.content,
            created_at: parse_proto_timestamp(post.created_at),
        }
    }
}
