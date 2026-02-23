use uuid::Uuid;

use crate::{domain::post::Post, error::AppError};

pub trait PostRepository {
    async fn create(&self, post: Post) -> anyhow::Result<(), AppError>;
    async fn get_by_id(&self, id: &Uuid) -> anyhow::Result<Post, AppError>;
    async fn update(&self, post: Post) -> anyhow::Result<(), AppError>;
    async fn remove(&self, id: &Uuid) -> anyhow::Result<(), AppError>;
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
        tracing::info!("get post by id {}", id);

        let post = self.repo.get_by_id(id).await?;

        Ok(post)
    }

    pub async fn update(
        &self,
        id: &Uuid,
        title: &str,
        content: &str,
        user_id: &Uuid,
    ) -> anyhow::Result<Post, AppError> {
        tracing::info!("update post {}", id);

        let mut post = self.verify_post_author(id, user_id).await?;

        let id = post.id;
        post.title = title.to_string();
        post.content = content.to_string();

        self.repo.update(post).await?;

        let post = self.get_by_id(&id).await?;

        Ok(post)
    }

    pub async fn remove(&self, id: &Uuid, user_id: &Uuid) -> anyhow::Result<(), AppError> {
        tracing::info!("remove post {}", id);

        let _ = self.verify_post_author(id, user_id).await?;

        self.repo.remove(&id).await?;

        Ok(())
    }

    async fn verify_post_author(
        &self,
        id: &Uuid,
        user_id: &Uuid,
    ) -> anyhow::Result<Post, AppError> {
        tracing::info!("verify author post {} for user {}", id, user_id);

        let post = self.get_by_id(id).await?;

        if post.author_id != *user_id {
            return Err(AppError::Internal("you are not author".to_string()));
        }

        Ok(post)
    }
}
