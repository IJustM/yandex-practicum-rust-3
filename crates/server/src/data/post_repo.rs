use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    application::post_service::PostRepository,
    domain::post::{Post, PostList},
    error::AppError,
};

pub struct SqlxPostRepository {
    pool: PgPool,
}

impl SqlxPostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl PostRepository for SqlxPostRepository {
    async fn create(&self, post: Post) -> anyhow::Result<(), AppError> {
        let Post {
            id,
            author_id,
            title,
            content,
            ..
        } = post;

        let res = sqlx::query!(
            r#"
                INSERT INTO posts (id, author_id, title, content)
                VALUES ($1, $2, $3, $4)
            "#,
            id,
            author_id,
            title,
            content,
        )
        .execute(&self.pool)
        .await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("SQL create post error: {:?}", e);
                Err(AppError::Db)
            }
        }
    }

    async fn get_by_id(&self, id: &Uuid) -> anyhow::Result<Post, AppError> {
        let res = sqlx::query_as!(
            Post,
            r#"
                SELECT id, author_id, title, content, created_at
                FROM posts
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await;

        match res {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(AppError::Unauthorized("post not found".to_string())),
            Err(_) => Err(AppError::Db),
        }
    }

    async fn update(&self, post: Post) -> anyhow::Result<(), AppError> {
        let Post {
            id, title, content, ..
        } = post;

        let res = sqlx::query!(
            r#"
                UPDATE posts
                SET title = $1,
                    content = $2
                WHERE id = $3
            "#,
            title,
            content,
            id
        )
        .execute(&self.pool)
        .await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("SQL update post error: {:?}", e);
                Err(AppError::Db)
            }
        }
    }

    async fn remove(&self, id: &Uuid) -> anyhow::Result<(), AppError> {
        let res = sqlx::query!(
            r#"
                DELETE FROM posts
                WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("SQL delete post error: {:?}", e);
                Err(AppError::Db)
            }
        }
    }

    async fn list(&self, limit: i64, offset: i64) -> anyhow::Result<PostList, AppError> {
        let posts = sqlx::query_as!(
            Post,
            r#"
                SELECT id, author_id, title, content, created_at
                FROM posts
                ORDER BY created_at DESC
                LIMIT $1
                OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await;

        let total = sqlx::query_scalar!(
            r#"
                SELECT COUNT(*) as "count!"
                FROM posts
            "#
        )
        .fetch_one(&self.pool)
        .await;

        match posts {
            Ok(posts) => match total {
                Ok(total) => Ok(PostList {
                    posts,
                    total,
                    limit,
                    offset,
                }),
                Err(e) => {
                    tracing::error!("SQL list post error: {:?}", e);
                    Err(AppError::Db)
                }
            },
            Err(e) => {
                tracing::error!("SQL list post error: {:?}", e);
                Err(AppError::Db)
            }
        }
    }
}
