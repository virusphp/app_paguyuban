use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Event {
    pub id: Uuid,
    pub org_id: Uuid,
    pub title: String,
    pub starts_at: DateTime<Utc>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: String, // 'aktif', 'selesai', 'batal'
    pub created_at: DateTime<Utc>,
}
