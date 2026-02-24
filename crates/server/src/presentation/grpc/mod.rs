pub mod error;
pub mod jwt;
pub mod post_service;
pub mod user_service;
pub mod utils;

use tonic::transport::Server;

use crate::{
    infrastructure::config::Config,
    presentation::grpc::{
        blog::{post_service_server::PostServiceServer, user_service_server::UserServiceServer},
        post_service::PostServiceImpl,
        user_service::UserServiceImpl,
    },
    state::AppState,
};

pub mod blog {
    tonic::include_proto!("blog");
}

pub async fn run_grpc(state: AppState) -> anyhow::Result<()> {
    let Config {
        host, port_grpc, ..
    } = state.config.as_ref();
    let addr = format!("{}:{}", host, port_grpc);
    tracing::info!("starting grpc server on {}", addr);

    let user_service = UserServiceServer::new(UserServiceImpl::new(state.clone()));
    let post_service = PostServiceServer::new(PostServiceImpl::new(state.clone()));

    Server::builder()
        .add_service(user_service)
        .add_service(post_service)
        .serve(addr.parse()?)
        .await?;

    Ok(())
}
