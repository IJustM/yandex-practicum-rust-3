mod error;
mod grpc_client;
mod http_client;

use std::fmt;

use serde::Deserialize;
use time::OffsetDateTime;
use tonic::async_trait;
use uuid::Uuid;

use crate::{error::BlogClientError, grpc_client::GrpcClient, http_client::HttpClient};

#[async_trait]
trait BlogClient: Send + Sync {
    // token
    fn set_token(&mut self, token: &str) -> ();
    // user
    async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> anyhow::Result<(), BlogClientError>;
    async fn login(
        &mut self,
        email: &str,
        password: &str,
    ) -> anyhow::Result<AuthResponse, BlogClientError>;
    // post
    async fn create_post(
        &mut self,
        title: &str,
        content: &str,
    ) -> anyhow::Result<Post, BlogClientError>;
    async fn get_post(&mut self, id: &Uuid) -> anyhow::Result<Post, BlogClientError>;
    async fn update_post(
        &mut self,
        id: &Uuid,
        title: &str,
        content: &str,
    ) -> anyhow::Result<Post, BlogClientError>;
    async fn delete_post(&mut self, id: &Uuid) -> anyhow::Result<(), BlogClientError>;
    async fn list_posts(
        &mut self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> anyhow::Result<PostList, BlogClientError>;
}

#[derive(Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
}

impl fmt::Display for Post {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "post(id={}, author_id={}, title={}, content={}, created_at={})",
            self.id, self.author_id, self.title, self.content, self.created_at
        )
    }
}

#[derive(Deserialize)]
pub struct PostList {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub posts: Vec<Post>,
}

impl fmt::Display for PostList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "post_list(total={}, limit={}, offset={},\n{})",
            self.total,
            self.limit,
            self.offset,
            self.posts
                .iter()
                .map(|post| post.to_string())
                .collect::<Vec<String>>()
                .join(",\n"),
        )
    }
}

#[derive(Deserialize)]
pub struct EmptyResponse {}

#[derive(Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
}

pub enum Transport {
    Http(String),
    Grpc(String),
}

pub struct BlogClientImpl {
    client: Box<dyn BlogClient>,
}

impl BlogClientImpl {
    pub async fn new(transport: Transport) -> anyhow::Result<Self, BlogClientError> {
        let client: Box<dyn BlogClient> = match transport {
            Transport::Http(addr) => Box::new(HttpClient::new(addr)),
            Transport::Grpc(addr) => Box::new(GrpcClient::new(addr).await?),
        };

        Ok(Self { client })
    }

    pub fn set_token(&mut self, token: &str) {
        self.client.set_token(token);
    }

    pub async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> anyhow::Result<(), BlogClientError> {
        self.client.register(username, email, password).await
    }

    pub async fn login(
        &mut self,
        email: &str,
        password: &str,
    ) -> anyhow::Result<AuthResponse, BlogClientError> {
        self.client.login(email, password).await
    }

    pub async fn create_post(
        &mut self,
        title: &str,
        content: &str,
    ) -> anyhow::Result<Post, BlogClientError> {
        self.client.create_post(title, content).await
    }

    pub async fn get_post(&mut self, id: &Uuid) -> anyhow::Result<Post, BlogClientError> {
        self.client.get_post(id).await
    }

    pub async fn update_post(
        &mut self,
        id: &Uuid,
        title: &str,
        content: &str,
    ) -> anyhow::Result<Post, BlogClientError> {
        self.client.update_post(id, title, content).await
    }

    pub async fn delete_post(&mut self, id: &Uuid) -> anyhow::Result<(), BlogClientError> {
        self.client.delete_post(id).await
    }

    pub async fn list_posts(
        &mut self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> anyhow::Result<PostList, BlogClientError> {
        self.client.list_posts(limit, offset).await
    }
}
