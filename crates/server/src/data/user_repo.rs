use sqlx::PgPool;

use crate::{
    application::user_service::{NewUser, UserRepository},
    domain::user::User,
    error::AppError,
};

pub struct SqxlUserRepository {
    pool: PgPool,
}

impl SqxlUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for SqxlUserRepository {
    async fn create(&self, user: NewUser) -> anyhow::Result<(), AppError> {
        let id = user.id;
        let email = user.email;
        let username = user.username;
        let password_hash = user.password_hash;

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
                tracing::error!("SQL create_user error: {:?}", e);
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
