use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Member {
    pub id: Uuid,
    pub org_id: Uuid,
    pub household_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub full_name: String,
    pub phone: Option<String>,
    pub status: String,
    pub joined_on: NaiveDate,
    pub created_at: DateTime<Utc>,
}
