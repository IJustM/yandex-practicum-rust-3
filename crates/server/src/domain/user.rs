use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    #[allow(dead_code)]
    pub username: String,
    #[allow(dead_code)]
    pub created_at: OffsetDateTime,
}
