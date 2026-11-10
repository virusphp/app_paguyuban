use axum::{
    extract::Query,
    response::{IntoResponse, Json, Response},
};
use chrono::{Datelike, Local, NaiveDate};
use lumina::prelude::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::app::services::{AuthContext, ViewService};

#[derive(Debug, Deserialize)]
pub struct ApiFeeQuery {
    pub year: Option<i32>,
}

#[derive(Serialize)]
pub struct MemberFeeSummaryDto {
    pub id: Uuid,
    pub full_name: String,
    pub house_no: String,
}

#[derive(Serialize)]
pub struct InvoiceItemDto {
    pub id: Uuid,
    pub plan_name: String,
    pub period_month: String,
    pub amount_idr: i64,
    pub amount_formatted: String,
    pub status: String,
    pub paid_on: Option<String>,
    pub method: Option<String>,
}

pub struct FeeApiController;

impl FeeApiController {
    /// Endpoint Mobile SvelteKit: GET /api/v1/iuran
    pub async fn my_invoices(req: Request, Query(params): Query<ApiFeeQuery>) -> Response {
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

        // 1. Cari profil member warga terkait user_id
        #[derive(sqlx::FromRow)]
        struct MemberRecord {
            id: Uuid,
            full_name: String,
            house_no: Option<String>,
        }

        let member_opt: Option<MemberRecord> = sqlx::query_as(
            r#"
            SELECT m.id, m.full_name, h.house_no
            FROM members m
            LEFT JOIN households h ON m.household_id = h.id
            WHERE m.org_id = $1 
              AND (
                  m.user_id = $2 
                  OR m.phone IN (SELECT phone FROM users WHERE id = $2)
              )
            LIMIT 1
            "#,
        )
        .bind(org_id)
        .bind(auth.user_id)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);

        let member = match member_opt {
            Some(m) => m,
            None => {
                return (
                    axum::http::StatusCode::OK,
                    Json(json!({
                        "success": true,
                        "data": {
                            "member": null,
                            "total_unpaid_idr": 0,
                            "total_paid_idr": 0,
                            "invoices": []
                        }
                    })),
                )
                    .into_response();
            }
        };

        // 2. Query invoices tahun berjalan
        let start_date = NaiveDate::from_ymd_opt(current_year, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(current_year, 12, 31).unwrap();

        #[derive(sqlx::FromRow)]
        struct InvoiceApiDbRow {
            id: Uuid,
            plan_name: String,
            period_month: NaiveDate,
            amount_idr: i64,
            status: String,
            paid_on: Option<NaiveDate>,
            method: Option<String>,
        }

        let invoices: Vec<InvoiceApiDbRow> = sqlx::query_as(
            r#"
            SELECT i.id, fp.name AS plan_name, i.period_month, i.amount_idr, i.status,
                   p.paid_on, p.method
            FROM invoices i
            JOIN fee_plans fp ON i.fee_plan_id = fp.id
            LEFT JOIN payments p ON p.invoice_id = i.id
            WHERE i.org_id = $1 
              AND i.member_id = $2
              AND i.period_month >= $3 
              AND i.period_month <= $4
            ORDER BY i.period_month DESC
            "#,
        )
        .bind(org_id)
        .bind(member.id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let mut total_unpaid_idr: i64 = 0;
        let mut total_paid_idr: i64 = 0;
        let mut invoice_items: Vec<InvoiceItemDto> = Vec::new();

        for inv in invoices {
            if inv.status == "lunas" {
                total_paid_idr += inv.amount_idr;
            } else {
                total_unpaid_idr += inv.amount_idr;
            }

            invoice_items.push(InvoiceItemDto {
                id: inv.id,
                plan_name: inv.plan_name,
                period_month: inv.period_month.format("%Y-%m-%d").to_string(),
                amount_idr: inv.amount_idr,
                amount_formatted: ViewService::format_rupiah(inv.amount_idr),
                status: inv.status,
                paid_on: inv.paid_on.map(|d| d.format("%Y-%m-%d").to_string()),
                method: inv.method,
            });
        }

        (
            axum::http::StatusCode::OK,
            Json(json!({
                "success": true,
                "data": {
                    "member": MemberFeeSummaryDto {
                        id: member.id,
                        full_name: member.full_name,
                        house_no: member.house_no.unwrap_or_else(|| "-".to_string()),
                    },
                    "year": current_year,
                    "total_unpaid_idr": total_unpaid_idr,
                    "total_unpaid_formatted": ViewService::format_rupiah(total_unpaid_idr),
                    "total_paid_idr": total_paid_idr,
                    "total_paid_formatted": ViewService::format_rupiah(total_paid_idr),
                    "invoices": invoice_items
                }
            })),
        )
            .into_response()
    }

    /// Endpoint Transparansi Kas & Iuran Lingkungan: GET /api/v1/iuran/ringkasan
    pub async fn public_summary(req: Request, Query(params): Query<ApiFeeQuery>) -> Response {
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

        // Total Billed
        let total_billed_idr: i64 = sqlx::query_scalar(
            r#"
            SELECT COALESCE(SUM(amount_idr), 0)
            FROM invoices
            WHERE org_id = $1 AND period_month >= $2 AND period_month <= $3
            "#,
        )
        .bind(org_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

        // Total Collected
        let total_collected_idr: i64 = sqlx::query_scalar(
            r#"
            SELECT COALESCE(SUM(amount_idr), 0)
            FROM invoices
            WHERE org_id = $1 AND period_month >= $2 AND period_month <= $3 AND status = 'lunas'
            "#,
        )
        .bind(org_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

        let total_arrears_idr = total_billed_idr - total_collected_idr;
        let compliance_percentage = if total_billed_idr > 0 {
            ((total_collected_idr as f64 / total_billed_idr as f64) * 100.0).round() as i64
        } else {
            100
        };

        (
            axum::http::StatusCode::OK,
            Json(json!({
                "success": true,
                "data": {
                    "org_name": auth.current_org_name.unwrap_or_else(|| "Paguyuban".to_string()),
                    "year": current_year,
                    "total_billed_idr": total_billed_idr,
                    "total_billed_formatted": ViewService::format_rupiah(total_billed_idr),
                    "total_collected_idr": total_collected_idr,
                    "total_collected_formatted": ViewService::format_rupiah(total_collected_idr),
                    "total_arrears_idr": total_arrears_idr,
                    "total_arrears_formatted": ViewService::format_rupiah(total_arrears_idr),
                    "compliance_percentage": compliance_percentage
                }
            })),
        )
            .into_response()
    }
}
