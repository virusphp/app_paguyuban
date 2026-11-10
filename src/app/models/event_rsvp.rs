use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct EventRsvp {
    pub id: Uuid,
    pub org_id: Uuid,
    pub event_id: Uuid,
    pub member_id: Uuid,
    pub answer: String, // 'hadir', 'tidak'
    pub created_at: DateTime<Utc>,
}
