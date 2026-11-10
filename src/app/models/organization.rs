#![allow(dead_code)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub r#type: String, // 'paguyuban', 'rw', 'rt'
    pub parent_id: Option<Uuid>,
    pub status: String, // 'aktif', 'menunggu', 'nonaktif'
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
