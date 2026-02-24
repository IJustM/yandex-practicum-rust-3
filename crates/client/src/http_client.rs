use tonic::async_trait;

use crate::Client;

#[derive(Default)]
pub struct HttpClient {
    addr: String,
}

impl HttpClient {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
}

#[async_trait]
impl Client for HttpClient {
    async fn register(&self, username: &str, email: &str, password: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
