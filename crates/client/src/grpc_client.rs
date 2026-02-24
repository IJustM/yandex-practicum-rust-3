use tonic::async_trait;

use crate::Client;

#[derive(Default)]
pub struct GrpcClient {
    addr: String,
}

impl GrpcClient {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
}

#[async_trait]
impl Client for GrpcClient {
    async fn register(&self, username: &str, email: &str, password: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
