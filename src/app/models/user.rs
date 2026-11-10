#![allow(dead_code)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

//                            USER MODEL
// Model akun pengguna aplikasi paguyuban sesuai skema PostgreSQL PLAN.md.
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub password_hash: String,
    pub is_superadmin: bool,
    pub status: String,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
