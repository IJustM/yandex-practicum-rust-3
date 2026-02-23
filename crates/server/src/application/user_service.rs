use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::{domain::user::User, error::AppError};

pub trait UserRepository {
    async fn create(&self, user: User) -> anyhow::Result<(), AppError>;
    async fn find_by_email(&self, email: &str) -> anyhow::Result<User, AppError>;
}

pub struct UserService<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn register(
        &self,
        email: &str,
        password: &str,
        username: &str,
    ) -> anyhow::Result<(), AppError> {
        let email = email.trim().to_lowercase();
        let username = username.trim().to_string();

        if email.is_empty() || password.len() < 6 {
            return Err(AppError::BadRequest(
                "invalid email or password".to_string(),
            ));
        }

        tracing::info!("register user = {}", email);

        let password_hash =
            hash_password(password).map_err(|_| AppError::Internal("hash error".to_string()))?;

        let id = uuid::Uuid::now_v7();
        self.repo
            .create(User {
                id,
                email: email.to_string(),
                username: username.to_string(),
                password_hash,
                created_at: None,
            })
            .await?;

        Ok(())
    }

    pub async fn login(&self, email: &str, password: &str) -> anyhow::Result<User, AppError> {
        let email = email.trim().to_lowercase();

        tracing::info!("login user = {}", &email);

        let user = self.repo.find_by_email(&email).await?;

        let ok = verify_password(password, &user.password_hash)
            .map_err(|_| AppError::Internal("verify error".to_string()))?;

        if !ok {
            return Err(AppError::Unauthorized("not correct password".to_string()));
        }

        Ok(user)
    }
}

fn hash_password(plain: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(plain.as_bytes(), &salt)?.to_string();
    Ok(hash)
}

fn verify_password(plain: &str, hash: &str) -> anyhow::Result<bool> {
    let parsed = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(plain.as_bytes(), &parsed).is_ok())
}
