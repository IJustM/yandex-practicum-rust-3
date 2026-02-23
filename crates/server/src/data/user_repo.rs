use sqlx::PgPool;

use crate::{application::user_service::UserRepository, domain::user::User, error::AppError};

pub struct SqlxUserRepository {
    pool: PgPool,
}

impl SqlxUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for SqlxUserRepository {
    async fn create(&self, user: User) -> anyhow::Result<(), AppError> {
        let User {
            id,
            email,
            username,
            password_hash,
            created_at: _,
        } = user;

        let res = sqlx::query!(
            r#"
                INSERT INTO users (id, email, username, password_hash)
                VALUES ($1, $2, $3, $4)
            "#,
            id,
            email,
            username,
            password_hash
        )
        .execute(&self.pool)
        .await;

        match res {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                Err(AppError::Conflict("email already exist".to_string()))
            }
            Err(e) => {
                tracing::error!("SQL create user error: {:?}", e);
                Err(AppError::Db)
            }
        }
    }

    async fn find_by_email(&self, email: &str) -> anyhow::Result<User, AppError> {
        let res = sqlx::query_as!(
            User,
            r#"
                SELECT id, email, username, password_hash, created_at
                FROM users
                WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await;

        match res {
            Ok(Some(user)) => Ok(user),
            Ok(None) => {
                return Err(AppError::Unauthorized("user not found".to_string()));
            }
            Err(_) => {
                return Err(AppError::Db);
            }
        }
    }
}
