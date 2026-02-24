pub mod error;
pub mod user_service;

use tonic::transport::Server;

use crate::{
    infrastructure::config::Config,
    presentation::grpc::{
        blog::user_service_server::UserServiceServer, user_service::UserServiceImpl,
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

    Server::builder()
        .add_service(user_service)
        .serve(addr.parse()?)
        .await?;

    Ok(())
}
