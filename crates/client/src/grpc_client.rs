use tonic::{Request, async_trait, transport::Channel};
use uuid::Uuid;

use crate::{
    BlogClient, Post, PostList,
    error::BlogClientError,
    grpc_client::blog::{
        RegisterRequest, post_service_client::PostServiceClient,
        user_service_client::UserServiceClient,
    },
};

mod blog {
    tonic::include_proto!("blog");
}

pub struct GrpcClient {
    token: Option<String>,
    user_service: UserServiceClient<Channel>,
    post_service: PostServiceClient<Channel>,
}

impl GrpcClient {
    pub async fn new(addr: String) -> anyhow::Result<Self, BlogClientError> {
        let user_service = UserServiceClient::connect(addr.clone())
            .await
            .map_err(|e| BlogClientError::InternalRgpcTransport(e))?;
        let post_service = PostServiceClient::connect(addr.clone())
            .await
            .map_err(|e| BlogClientError::InternalRgpcTransport(e))?;

        Ok(Self {
            token: None,
            user_service,
            post_service,
        })
    }
}

#[async_trait]
impl BlogClient for GrpcClient {
    fn set_token(&mut self, token: &str) {
        self.token = Some(token.to_string());
    }

    async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> anyhow::Result<(), BlogClientError> {
        let req = Request::new(RegisterRequest {
            email: email.to_string(),
            password: password.to_string(),
            username: username.to_string(),
        });

        self.user_service
            .register(req)
            .await
            .map_err(|e| BlogClientError::InternalRgpcStatus(e))?;

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
