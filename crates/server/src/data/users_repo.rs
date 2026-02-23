use server::state::AppState;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    #[allow(dead_code)]
    pub username: String,
    pub password_hash: String,
    #[allow(dead_code)]
    pub created_at: OffsetDateTime,
}

pub async fn create_user(
    state: &AppState,
    id: &Uuid,
    email: &str,
    username: &str,
    password_hash: &str,
) -> anyhow::Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, username, password_hash)
        VALUES ($1, $2, $3, $4)
        "#,
        id,
        email,
        username,
        password_hash
    )
    .execute(&state.pool)
    .await?;

    Ok(())
}

pub async fn find_by_email(
    state: &AppState,
    email: &str,
) -> anyhow::Result<Option<UserRow>, sqlx::Error> {
    let user = sqlx::query_as!(
        UserRow,
        r#"
        SELECT id, email, username, password_hash, created_at
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(&state.pool)
    .await?;

    Ok(user)
}
