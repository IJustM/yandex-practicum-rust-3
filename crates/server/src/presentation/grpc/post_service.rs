use pbjson_types::Empty;
use tonic::{Request, Response, Status};

use crate::{
    domain,
    presentation::grpc::{
        blog::{
            CreatePostRequest, DeletePostRequest, GetPostRequest, ListPostsRequest,
            ListPostsResponse, Post, UpdatePostRequest, post_service_server::PostService,
        },
        jwt::get_claims,
        utils::to_uuid,
    },
    state::AppState,
};

#[derive(Clone)]
pub struct PostServiceImpl {
    state: AppState,
}

impl PostServiceImpl {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl PostService for PostServiceImpl {
    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> anyhow::Result<Response<Post>, Status> {
        let claims = get_claims(&self.state, &request)?;
        let payload = request.into_inner();

        let post = self
            .state
            .post_service
            .create(&payload.title, &payload.content, &claims.sub)
            .await?;

        Ok(Response::new(post.into()))
    }

    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> anyhow::Result<Response<Post>, Status> {
        let payload = request.into_inner();

        let post = self
            .state
            .post_service
            .get_by_id(&to_uuid(&payload.id)?)
            .await?;

        Ok(Response::new(post.into()))
    }

    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>,
    ) -> anyhow::Result<Response<Post>, Status> {
        let claims = get_claims(&self.state, &request)?;
        let payload = request.into_inner();

        let post = self
            .state
            .post_service
            .update(
                &to_uuid(&payload.id)?,
                &payload.title,
                &payload.content,
                &claims.sub,
            )
            .await?;

        Ok(Response::new(post.into()))
    }

    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> anyhow::Result<Response<Empty>, Status> {
        let claims = get_claims(&self.state, &request)?;
        let payload = request.into_inner();

        self.state
            .post_service
            .remove(&to_uuid(&payload.id)?, &claims.sub)
            .await?;

        Ok(Response::new(Empty {}))
    }

    async fn list_posts(
        &self,
        request: Request<ListPostsRequest>,
    ) -> anyhow::Result<Response<ListPostsResponse>, Status> {
        let payload = request.into_inner();

        let list = self
            .state
            .post_service
            .list(payload.limit, payload.offset)
            .await?;

        Ok(Response::new(ListPostsResponse {
            posts: list.posts.into_iter().map(|p| p.into()).collect(),
            total: list.total,
            limit: list.limit,
            offset: list.offset,
        }))
    }
}

impl From<domain::post::Post> for Post {
    fn from(post: domain::post::Post) -> Self {
        Post {
            id: post.id.to_string(),
            author_id: post.author_id.to_string(),
            title: post.title,
            content: post.content,
            created_at: post.created_at.map(|v| pbjson_types::Timestamp {
                seconds: v.unix_timestamp(),
                nanos: v.nanosecond() as i32,
            }),
        }
    }
}
