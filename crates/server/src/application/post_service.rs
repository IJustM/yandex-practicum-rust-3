use uuid::Uuid;

use crate::{domain::post::Post, error::AppError};

pub trait PostRepository {
    async fn create(&self, post: Post) -> anyhow::Result<(), AppError>;
    async fn get_by_id(&self, id: &Uuid) -> anyhow::Result<Post, AppError>;
}

pub struct PostService<R: PostRepository> {
    repo: R,
}

impl<R: PostRepository> PostService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create(
        &self,
        title: &str,
        content: &str,
        author_id: &Uuid,
    ) -> anyhow::Result<Post, AppError> {
        if title.is_empty() || content.is_empty() {
            return Err(AppError::BadRequest("invalid title or content".to_string()));
        }

        tracing::info!("user {} create post", author_id);

        let id = Uuid::now_v7();
        self.repo
            .create(Post {
                id,
                author_id: author_id.clone(),
                title: title.to_string(),
                content: content.to_string(),
                created_at: None,
            })
            .await?;

        let post = self.get_by_id(&id).await?;

        Ok(post)
    }

    pub async fn get_by_id(&self, id: &Uuid) -> anyhow::Result<Post, AppError> {
        tracing::info!("get post {}", id);

        let post = self.repo.get_by_id(id).await?;

        Ok(post)
    }
}
