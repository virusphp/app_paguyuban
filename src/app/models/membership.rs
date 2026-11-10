#![allow(dead_code)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Membership {
    pub id: Uuid,
    pub org_id: Uuid,
    pub user_id: Uuid,
    pub role: String, // 'admin', 'bendahara', 'sekretaris', 'anggota'
    pub created_at: DateTime<Utc>,
}
