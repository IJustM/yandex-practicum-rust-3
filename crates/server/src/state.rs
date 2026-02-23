use std::sync::Arc;

use crate::{
    application::user_service::UserService, data::user_repo::SqxlUserRepository,
    infrastructure::config::Config,
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub user_service: Arc<UserService<SqxlUserRepository>>,
}
