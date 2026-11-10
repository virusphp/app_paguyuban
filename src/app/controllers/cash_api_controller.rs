use axum::{
    extract::Query,
    response::{IntoResponse, Json, Response},
};
use chrono::{Datelike, Local, NaiveDate};
use lumina::prelude::Request;
use serde::Deserialize;
use serde_json::json;

use crate::app::services::{AuthContext, ViewService};

#[derive(Debug, Deserialize)]
pub struct ApiCashQuery {
    pub year: Option<i32>,
}

pub struct CashApiController;

impl CashApiController {
    /// Endpoint Mobile SvelteKit PWA: GET /api/v1/kas/ringkasan
    pub async fn summary(req: Request, Query(params): Query<ApiCashQuery>) -> Response {
        let auth =
            match AuthContext::from_request(&req).await {
                Ok(ctx) => ctx,
                Err(_) => return (
                    axum::http::StatusCode::UNAUTHORIZED,
                    Json(json!({ "success": false, "error": "Sesi tidak valid atau belum login" })),
                )
                    .into_response(),
            };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(_) => {
                return (
                    axum::http::StatusCode::FORBIDDEN,
                    Json(json!({ "success": false, "error": "Belum memilih organisasi aktif" })),
                )
                    .into_response()
            }
        };

        let current_year = params.year.unwrap_or_else(|| Local::now().year());
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => {
                return (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "success": false, "error": e.to_string() })),
                )
                    .into_response()
            }
        };

        let start_date = NaiveDate::from_ymd_opt(current_year, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(current_year, 12, 31).unwrap();

        // 1. Total Pemasukan Tahun Ini
        let total_income: i64 = sqlx::query_scalar(
            r#"
            SELECT COALESCE(SUM(amount_idr), 0)::BIGINT
            FROM transactions
            WHERE org_id = $1 AND kind = 'masuk' AND occurred_on >= $2 AND occurred_on <= $3
            "#,
        )
        .bind(org_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

        // 2. Total Pengeluaran Tahun Ini
        let total_expense: i64 = sqlx::query_scalar(
            r#"
            SELECT COALESCE(SUM(amount_idr), 0)::BIGINT
            FROM transactions
            WHERE org_id = $1 AND kind = 'keluar' AND occurred_on >= $2 AND occurred_on <= $3
            "#,
        )
        .bind(org_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

        // 3. Saldo Kas Kumulatif (Seluruh Waktu)
        let total_all_income: i64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(amount_idr), 0)::BIGINT FROM transactions WHERE org_id = $1 AND kind = 'masuk'",
        )
        .bind(org_id)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

        let total_all_expense: i64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(amount_idr), 0)::BIGINT FROM transactions WHERE org_id = $1 AND kind = 'keluar'",
        )
        .bind(org_id)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

        let current_balance = total_all_income - total_all_expense;

        // 4. Jumlah Donasi Barang
        let in_kind_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM in_kind_donations WHERE org_id = $1")
                .bind(org_id)
                .fetch_one(&pool)
                .await
                .unwrap_or(0);

        (
            axum::http::StatusCode::OK,
            Json(json!({
                "success": true,
                "data": {
                    "org_name": auth.current_org_name.unwrap_or_else(|| "Paguyuban".to_string()),
                    "year": current_year,
                    "current_balance_idr": current_balance,
                    "current_balance_formatted": ViewService::format_rupiah(current_balance),
                    "total_income_idr": total_income,
                    "total_income_formatted": ViewService::format_rupiah(total_income),
                    "total_expense_idr": total_expense,
                    "total_expense_formatted": ViewService::format_rupiah(total_expense),
                    "in_kind_items_count": in_kind_count
                }
            })),
        )
            .into_response()
    }
}
