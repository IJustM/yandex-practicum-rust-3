use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: Option<OffsetDateTime>,
}
