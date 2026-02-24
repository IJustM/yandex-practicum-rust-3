use std::collections::HashMap;

use reqwest::{Client, Error, RequestBuilder, Response};
use serde::{Deserialize, de::DeserializeOwned};
use tonic::async_trait;
use uuid::Uuid;

use crate::{BlogClient, EmptyResponse, Post, PostList, error::BlogClientError};

#[derive(Default)]
pub struct HttpClient {
    addr: String,
    token: Option<String>,
    client: Client,
}

impl HttpClient {
    pub fn new(addr: String) -> Self {
        Self {
            addr,
            token: None,
            client: Client::new(),
        }
    }

    fn url(&self, path: &str) -> String {
        let res = format!("{}{}", self.addr, path);
        println!("{}", res);
        res
    }

    async fn send_req<Data>(&self, req: RequestBuilder) -> anyhow::Result<Data, BlogClientError>
    where
        Data: DeserializeOwned,
    {
        let res = req.send().await.map_err(|e| BlogClientError::Internal(e))?;

        if !res.status().is_success() {
            let message = res.text().await.map_err(|e| BlogClientError::Internal(e))?;
            return Err(BlogClientError::InvalidRequest(message));
        }

        let data = res
            .json::<Data>()
            .await
            .map_err(|e| BlogClientError::Internal(e))?;

        Ok(data)
    }
}

#[async_trait]
impl BlogClient for HttpClient {
    async fn register(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> anyhow::Result<(), BlogClientError> {
        let req = self
            .client
            .post(self.url("/api/auth/register"))
            .json(&HashMap::from([
                ("email", email),
                ("password", password),
                ("username", username),
            ]));

        let _ = self.send_req::<EmptyResponse>(req).await?;

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
