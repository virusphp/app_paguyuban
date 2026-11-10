use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct FeePlan {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub amount_idr: i64,
    pub period: String,
    pub active_from: NaiveDate,
    pub active_to: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}
