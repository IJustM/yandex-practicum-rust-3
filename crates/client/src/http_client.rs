use std::collections::HashMap;

use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, de::DeserializeOwned};
use tonic::async_trait;
use uuid::Uuid;

use crate::{AuthResponse, BlogClient, EmptyResponse, Post, PostList, error::BlogClientError};

pub struct HttpClient {
    addr: String,
    token: Option<String>,
    client: Client,
}

#[derive(Deserialize)]
struct ErrorMessage {
    message: String,
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
        format!("{}{}", self.addr, path)
    }

    async fn send_req<Data>(&self, req: RequestBuilder) -> anyhow::Result<Data, BlogClientError>
    where
        Data: DeserializeOwned,
    {
        let res = req
            .send()
            .await
            .map_err(|e| BlogClientError::InternalHttp(e))?;

        if !res.status().is_success() {
            let bytes = res
                .bytes()
                .await
                .map_err(|e| BlogClientError::InternalHttp(e))?;

            return Err(BlogClientError::InvalidRequest(
                match serde_json::from_slice::<ErrorMessage>(&bytes) {
                    Ok(e) => e.message,
                    Err(_) => String::from_utf8_lossy(&bytes).to_string(),
                },
            ));
        }

        let data = res
            .json::<Data>()
            .await
            .map_err(|e| BlogClientError::InternalHttp(e))?;

        Ok(data)
    }
}

#[async_trait]
impl BlogClient for HttpClient {
    fn set_token(&mut self, token: &str) {
        self.token = Some(token.to_string());
    }

    async fn register(
        &mut self,
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

    async fn login(
        &mut self,
        email: &str,
        password: &str,
    ) -> anyhow::Result<AuthResponse, BlogClientError> {
        let req = self
            .client
            .post(self.url("/api/auth/login"))
            .json(&HashMap::from([("email", email), ("password", password)]));

        let res = self.send_req::<AuthResponse>(req).await?;

        Ok(res)
    }

    async fn create_post(
        &mut self,
        title: &str,
        content: &str,
    ) -> anyhow::Result<Post, BlogClientError> {
        todo!()
    }

    async fn get_post(&mut self, id: &Uuid) -> anyhow::Result<Post, BlogClientError> {
        todo!()
    }

    async fn update_post(
        &mut self,
        id: &Uuid,
        title: &str,
        content: &str,
    ) -> anyhow::Result<Post, BlogClientError> {
        todo!()
    }

    async fn delete_post(&mut self, id: &Uuid) -> anyhow::Result<(), BlogClientError> {
        todo!()
    }

    async fn list_posts(
        &mut self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> anyhow::Result<PostList, BlogClientError> {
        todo!()
    }
}
