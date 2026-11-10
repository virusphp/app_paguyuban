use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Transaction {
    pub id: Uuid,
    pub org_id: Uuid,
    pub event_id: Option<Uuid>,
    pub kind: String,     // 'masuk', 'keluar'
    pub category: String, // 'saldo_awal', 'iuran', 'iuran_panitia', 'donasi', 'lelang', 'konsumsi', 'hadiah', 'perlengkapan', 'operasional', 'lainnya'
    pub description: String,
    pub amount_idr: i64,
    pub occurred_on: NaiveDate,
    pub donor_name: Option<String>,
    pub source_payment_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}
