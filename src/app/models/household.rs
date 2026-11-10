use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Household {
    pub id: Uuid,
    pub org_id: Uuid,
    pub house_no: String,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}
