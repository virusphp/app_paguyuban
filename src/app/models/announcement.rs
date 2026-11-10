use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Announcement {
    pub id: Uuid,
    pub org_id: Uuid,
    pub title: String,
    pub body: String,
    pub pinned: bool,
    pub published_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}
