use pbjson_types::Empty;
use tonic::{Request, Response, Status};

use crate::{
    error::AppError,
    infrastructure::jwt,
    presentation::grpc::blog::{
        AuthResponse, LoginRequest, RegisterRequest, user_service_server::UserService,
    },
    state::AppState,
};

#[derive(Clone)]
pub struct UserServiceImpl {
    state: AppState,
}

impl UserServiceImpl {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> anyhow::Result<Response<Empty>, Status> {
        let payload = request.into_inner();

        self.state
            .user_service
            .register(&payload.email, &payload.password, &payload.username)
            .await?;

        Ok(Response::new(Empty {}))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> anyhow::Result<Response<AuthResponse>, Status> {
        let payload = request.into_inner();

        let user = self
            .state
            .user_service
            .login(&payload.email, &payload.password)
            .await?;

        let access_token = jwt::generate_jwt(&self.state.config.jwt_secret, &user.id)
            .map_err(|_| AppError::Internal("jwt error".to_string()))?;

        Ok(Response::new(AuthResponse { access_token }))
    }
}
