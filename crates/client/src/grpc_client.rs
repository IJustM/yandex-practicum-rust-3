use time::{Duration, OffsetDateTime};
use tonic::{Request, async_trait, transport::Channel};
use uuid::Uuid;

use crate::{
    AuthResponse, BlogClient, Post, PostList,
    error::BlogClientError,
    grpc_client::blog::{
        CreatePostRequest, DeletePostRequest, GetPostRequest, ListPostsRequest, LoginRequest,
        RegisterRequest, UpdatePostRequest, post_service_client::PostServiceClient,
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
            .map_err(BlogClientError::InternalRgpcTransport)?;
        let post_service = PostServiceClient::connect(addr.clone())
            .await
            .map_err(BlogClientError::InternalRgpcTransport)?;

        Ok(Self {
            token: None,
            user_service,
            post_service,
        })
    }

    fn add_token_to_req<T>(
        &self,
        mut req: Request<T>,
    ) -> anyhow::Result<Request<T>, BlogClientError> {
        let authorization = format!("Bearer {}", self.token.clone().unwrap_or("".to_string()));

        req.metadata_mut().insert(
            "authorization",
            authorization
                .parse()
                .map_err(|_| BlogClientError::Internal("add authorization error".to_string()))?,
        );

        Ok(req)
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

        let _ = self
            .user_service
            .register(req)
            .await
            .map_err(BlogClientError::InternalRgpcStatus)?;

        Ok(())
    }

    async fn login(
        &mut self,
        email: &str,
        password: &str,
    ) -> anyhow::Result<AuthResponse, BlogClientError> {
        let req = Request::new(LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        });

        let res = self
            .user_service
            .login(req)
            .await
            .map_err(BlogClientError::InternalRgpcStatus)?
            .into_inner();

        Ok(res.into())
    }

    async fn create_post(
        &mut self,
        title: &str,
        content: &str,
    ) -> anyhow::Result<Post, BlogClientError> {
        let req = self.add_token_to_req(Request::new(CreatePostRequest {
            title: title.to_string(),
            content: content.to_string(),
        }))?;

        let res = self
            .post_service
            .create_post(req)
            .await
            .map_err(BlogClientError::InternalRgpcStatus)?
            .into_inner();

        Ok(to_post(res)?)
    }

    async fn get_post(&mut self, id: &Uuid) -> anyhow::Result<Post, BlogClientError> {
        let req = Request::new(GetPostRequest { id: id.to_string() });

        let res = self
            .post_service
            .get_post(req)
            .await
            .map_err(BlogClientError::InternalRgpcStatus)?
            .into_inner();

        Ok(to_post(res)?)
    }

    async fn update_post(
        &mut self,
        id: &Uuid,
        title: &str,
        content: &str,
    ) -> anyhow::Result<Post, BlogClientError> {
        let req = self.add_token_to_req(Request::new(UpdatePostRequest {
            id: id.to_string(),
            title: title.to_string(),
            content: content.to_string(),
        }))?;

        let res = self
            .post_service
            .update_post(req)
            .await
            .map_err(BlogClientError::InternalRgpcStatus)?
            .into_inner();

        Ok(to_post(res)?)
    }

    async fn delete_post(&mut self, id: &Uuid) -> anyhow::Result<(), BlogClientError> {
        let req = self.add_token_to_req(Request::new(DeletePostRequest { id: id.to_string() }))?;

        self.post_service
            .delete_post(req)
            .await
            .map_err(BlogClientError::InternalRgpcStatus)?;

        Ok(())
    }

    async fn list_posts(
        &mut self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> anyhow::Result<PostList, BlogClientError> {
        let req = Request::new(ListPostsRequest { limit, offset });

        let res = self
            .post_service
            .list_posts(req)
            .await
            .map_err(BlogClientError::InternalRgpcStatus)?
            .into_inner();

        let posts: Result<Vec<_>, _> = res.posts.into_iter().map(to_post).collect();

        Ok(PostList {
            total: res.total,
            limit: res.limit,
            offset: res.offset,
            posts: posts?,
        })
    }
}

impl From<blog::AuthResponse> for AuthResponse {
    fn from(auth_response: blog::AuthResponse) -> Self {
        Self {
            access_token: auth_response.access_token,
        }
    }
}

fn to_uuid(value: &str) -> anyhow::Result<Uuid, BlogClientError> {
    match Uuid::parse_str(value) {
        Ok(uuid) => Ok(uuid),
        Err(_) => Err(BlogClientError::Internal("parse error to uuid".to_string())),
    }
}

fn to_post(post: blog::Post) -> anyhow::Result<Post, BlogClientError> {
    let created_at = post
        .created_at
        .and_then(|t| {
            OffsetDateTime::from_unix_timestamp(t.seconds)
                .ok()
                .and_then(|dt| dt.checked_add(Duration::nanoseconds(t.nanos as i64)))
        })
        .ok_or(BlogClientError::Internal(
            "create_at parse error".to_string(),
        ))?;

    Ok(Post {
        id: to_uuid(&post.id)?,
        author_id: to_uuid(&post.author_id)?,
        title: post.title,
        content: post.content,
        created_at,
    })
}
