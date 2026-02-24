pub mod error;
pub mod grpc_client;
pub mod http_client;

use serde::Deserialize;
use time::OffsetDateTime;
use tonic::async_trait;
use uuid::Uuid;

use crate::{error::BlogClientError, grpc_client::GrpcClient, http_client::HttpClient};

#[async_trait]
trait BlogClient: Send + Sync {
    // user
    async fn register(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> anyhow::Result<(), BlogClientError>;
    async fn login(&self, email: &str, password: &str) -> anyhow::Result<()>;
    // post
    async fn create_post(&self, title: &str, content: &str) -> anyhow::Result<Post>;
    async fn get_post(&self, id: &Uuid) -> anyhow::Result<Post>;
    async fn update_post(&self, id: &Uuid, title: &str, content: &str) -> anyhow::Result<Post>;
    async fn delete_post(&self, id: &Uuid) -> anyhow::Result<()>;
    async fn list_posts(&self, limit: i64, offset: i64) -> anyhow::Result<PostList>;
}

#[derive(Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: OffsetDateTime,
}

impl Default for Post {
    fn default() -> Self {
        Self {
            id: Uuid::now_v7(),
            author_id: Uuid::now_v7(),
            title: "".to_string(),
            content: "".to_string(),
            created_at: OffsetDateTime::now_utc(),
        }
    }
}

#[derive(Deserialize)]
pub struct PostList {
    pub posts: Vec<Post>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Deserialize)]
pub struct EmptyResponse {}

impl Default for PostList {
    fn default() -> Self {
        Self {
            posts: vec![],
            total: 0,
            limit: 0,
            offset: 0,
        }
    }
}

pub enum Transport {
    Http(String),
    Grpc(String),
}

pub struct BlogClientImpl {
    client: Box<dyn BlogClient>,
}

impl BlogClientImpl {
    pub fn new(transport: Transport) -> Self {
        let client: Box<dyn BlogClient> = match transport {
            Transport::Http(addr) => Box::new(HttpClient::new(addr)),
            Transport::Grpc(addr) => Box::new(GrpcClient::new(addr)),
        };

        Self { client }
    }

    pub async fn register(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> anyhow::Result<(), BlogClientError> {
        self.client.register(username, email, password).await
    }

    pub async fn login(&mut self, email: &str, password: &str) -> anyhow::Result<()> {
        self.client.login(email, password).await
    }

    pub async fn create_post(&self, title: &str, content: &str) -> anyhow::Result<Post> {
        self.client.create_post(title, content).await
    }

    pub async fn get_post(&self, id: &Uuid) -> anyhow::Result<Post> {
        self.client.get_post(id).await
    }

    pub async fn update_post(&self, id: &Uuid, title: &str, content: &str) -> anyhow::Result<Post> {
        self.client.update_post(id, title, content).await
    }

    pub async fn delete_post(&self, id: &Uuid) -> anyhow::Result<()> {
        self.client.delete_post(id).await
    }

    pub async fn list_posts(&self, limit: i64, offset: i64) -> anyhow::Result<PostList> {
        self.client.list_posts(limit, offset).await
    }
}
