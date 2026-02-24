use tonic::async_trait;
use uuid::Uuid;

use crate::{BlogClient, Post, PostList, error::BlogClientError};

#[derive(Default)]
pub struct GrpcClient {
    addr: String,
    token: Option<String>,
}

impl GrpcClient {
    pub fn new(addr: String) -> Self {
        Self { addr, token: None }
    }
}

#[async_trait]
impl BlogClient for GrpcClient {
    async fn register(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> anyhow::Result<(), BlogClientError> {
        Ok(())
    }

    async fn login(&self, email: &str, password: &str) -> anyhow::Result<()> {
        Ok(())
    }

    async fn create_post(&self, title: &str, content: &str) -> anyhow::Result<Post> {
        Ok(Post::default())
    }

    async fn get_post(&self, id: &Uuid) -> anyhow::Result<Post> {
        Ok(Post::default())
    }

    async fn update_post(&self, id: &Uuid, title: &str, content: &str) -> anyhow::Result<Post> {
        Ok(Post::default())
    }

    async fn delete_post(&self, id: &Uuid) -> anyhow::Result<()> {
        Ok(())
    }

    async fn list_posts(&self, limit: i64, offset: i64) -> anyhow::Result<PostList> {
        Ok(PostList::default())
    }
}
