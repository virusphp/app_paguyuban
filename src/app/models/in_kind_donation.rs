use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct InKindDonation {
    pub id: Uuid,
    pub org_id: Uuid,
    pub event_id: Option<Uuid>,
    pub donor_name: String,
    pub item: String,
    pub quantity_text: String,
    pub created_at: DateTime<Utc>,
}
