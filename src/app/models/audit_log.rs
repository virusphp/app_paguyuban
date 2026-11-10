#![allow(dead_code)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct AuditLog {
    pub id: Uuid,
    pub org_id: Option<Uuid>,
    pub actor_user_id: Option<Uuid>,
    pub action: String,
    pub entity: String,
    pub entity_id: Option<String>,
    pub meta: Option<serde_json::Value>,
    pub at: DateTime<Utc>,
}
