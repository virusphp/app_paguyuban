use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Invoice {
    pub id: Uuid,
    pub org_id: Uuid,
    pub member_id: Uuid,
    pub fee_plan_id: Uuid,
    pub period_month: NaiveDate,
    pub amount_idr: i64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}
