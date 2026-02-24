use tonic::async_trait;

use crate::{grpc_client::GrpcClient, http_client::HttpClient};

pub mod grpc_client;
pub mod http_client;

#[async_trait]
trait Client: Send + Sync {
    async fn register(&self, username: &str, email: &str, password: &str) -> anyhow::Result<()>;
}

pub enum Transport {
    Http(String),
    Grpc(String),
}

pub struct BlogClient {
    client: Box<dyn Client>,
    token: Option<String>,
}

impl BlogClient {
    pub fn new(transport: Transport) -> Self {
        let client: Box<dyn Client> = match transport {
            Transport::Http(addr) => Box::new(HttpClient::new(addr)),
            Transport::Grpc(addr) => Box::new(GrpcClient::new(addr)),
        };

        Self {
            client,
            token: None,
        }
    }

    pub fn set_token(&mut self, token: &str) {
        self.token = Some(token.to_string());
    }

    pub fn get_token(&self) -> Option<String> {
        self.token.clone()
    }

    pub async fn register(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> anyhow::Result<()> {
        self.client.register(username, email, password).await?;
        Ok(())
    }
}
