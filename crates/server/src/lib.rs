pub mod error;
pub mod infrastructure;
pub mod state;

pub mod db {
    use sqlx::{Pool, Postgres};

    use crate::infrastructure::{self, config::Config};

    pub fn create_config() -> Config {
        let config = infrastructure::config::Config::from_env().expect("invalid config");
        config
    }

    pub async fn create_db(config: &Config) -> Pool<Postgres> {
        let pool = infrastructure::db::create_db(&config.database_url).await;

        infrastructure::migrate::run(&pool)
            .await
            .expect("migrations failed");

        pool
    }
}
