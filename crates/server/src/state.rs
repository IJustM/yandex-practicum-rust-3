use std::sync::Arc;

use crate::{
    application::{post_service::PostService, user_service::UserService},
    data::{post_repo::SqlxPostRepository, user_repo::SqlxUserRepository},
    infrastructure::config::Config,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub user_service: Arc<UserService<SqlxUserRepository>>,
    pub post_service: Arc<PostService<SqlxPostRepository>>,
}
