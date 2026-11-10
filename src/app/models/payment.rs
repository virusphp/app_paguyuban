use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Payment {
    pub id: Uuid,
    pub org_id: Uuid,
    pub invoice_id: Uuid,
    pub amount_idr: i64,
    pub method: String,
    pub paid_on: NaiveDate,
    pub confirmed_by: Option<Uuid>,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
}
