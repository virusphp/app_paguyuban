use axum::{
    extract::{Path, Query},
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
};
use chrono::{DateTime, Local, Utc};
use lumina::prelude::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::app::services::{AuthContext, LetterPdfService};

#[derive(Debug, Deserialize)]
pub struct ApiLetterQuery {
    pub kind: Option<String>,
    pub q: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiLetterSummaryDto {
    pub id: Uuid,
    pub kind: String,
    pub number: String,
    pub subject: String,
    pub recipient: String,
    pub status: String,
    pub date_formatted: String,
    pub author_name: Option<String>,
    pub sent_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiLetterDetailDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub org_name: String,
    pub kind: String,
    pub number: String,
    pub subject: String,
    pub recipient: String,
    pub body: String,
    pub status: String,
    pub date_formatted: String,
    pub author_name: Option<String>,
    pub sent_at: Option<String>,
}

pub struct LetterApiController;

impl LetterApiController {
    /// Endpoint Mobile SvelteKit: GET /api/v1/surat
    pub async fn list_letters(req: Request, Query(query): Query<ApiLetterQuery>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "success": false, "error": e.to_string() })),
                )
                    .into_response();
            }
        };

        // Cek wewenang: Warga biasa (anggota/bendahara) HANYA boleh melihat status 'terkirim'
        let can_view_drafts = auth.is_superadmin
            || matches!(
                auth.current_role.as_deref(),
                Some("admin") | Some("sekretaris")
            );

        #[derive(sqlx::FromRow)]
        struct RawLetterRow {
            id: Uuid,
            kind: String,
            number: String,
            subject: String,
            recipient: String,
            status: String,
            created_at: DateTime<Utc>,
            sent_at: Option<DateTime<Utc>>,
            author_name: Option<String>,
        }

        let raw_letters: Vec<RawLetterRow> = match sqlx::query_as(
            r#"
            SELECT l.id, l.kind, l.number, l.subject, l.recipient, l.status,
                   l.created_at, l.sent_at, u.name as author_name
            FROM letters l
            LEFT JOIN users u ON l.created_by = u.id
            WHERE l.org_id = $1
              AND ($2::bool = true OR l.status = 'terkirim')
              AND ($3::text IS NULL OR l.kind = $3)
              AND ($4::text IS NULL OR (
                    l.subject ILIKE '%' || $4 || '%' 
                    OR l.number ILIKE '%' || $4 || '%' 
                    OR l.recipient ILIKE '%' || $4 || '%'
                  ))
            ORDER BY l.created_at DESC;
            "#,
        )
        .bind(org_id)
        .bind(can_view_drafts)
        .bind(query.kind.as_deref())
        .bind(query.q.as_deref())
        .fetch_all(&pool)
        .await
        {
            Ok(items) => items,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "success": false, "error": e.to_string() })),
                )
                    .into_response();
            }
        };

        let data: Vec<ApiLetterSummaryDto> = raw_letters
            .into_iter()
            .map(|r| {
                let local_time: DateTime<Local> = DateTime::from(r.created_at);
                let date_formatted = local_time.format("%d %b %Y").to_string();
                let sent_at = r.sent_at.map(|st| {
                    DateTime::<Local>::from(st)
                        .format("%d %b %Y %H:%M")
                        .to_string()
                });

                ApiLetterSummaryDto {
                    id: r.id,
                    kind: r.kind,
                    number: r.number,
                    subject: r.subject,
                    recipient: r.recipient,
                    status: r.status,
                    date_formatted,
                    author_name: r.author_name,
                    sent_at,
                }
            })
            .collect();

        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": data,
                "total": data.len(),
            })),
        )
            .into_response()
    }

    /// Endpoint Mobile SvelteKit: GET /api/v1/surat/:id
    pub async fn get_letter(req: Request, Path(letter_id): Path<Uuid>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        let org_name = auth
            .current_org_name
            .clone()
            .unwrap_or_else(|| "Paguyuban".to_string());

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "success": false, "error": e.to_string() })),
                )
                    .into_response();
            }
        };

        let can_view_drafts = auth.is_superadmin
            || matches!(
                auth.current_role.as_deref(),
                Some("admin") | Some("sekretaris")
            );

        #[derive(sqlx::FromRow)]
        struct RawLetterDetail {
            id: Uuid,
            kind: String,
            number: String,
            subject: String,
            recipient: String,
            body: String,
            status: String,
            created_at: DateTime<Utc>,
            sent_at: Option<DateTime<Utc>>,
            author_name: Option<String>,
        }

        let raw: Option<RawLetterDetail> = match sqlx::query_as(
            r#"
            SELECT l.id, l.kind, l.number, l.subject, l.recipient, l.body, l.status,
                   l.created_at, l.sent_at, u.name as author_name
            FROM letters l
            LEFT JOIN users u ON l.created_by = u.id
            WHERE l.id = $1 AND l.org_id = $2;
            "#,
        )
        .bind(letter_id)
        .bind(org_id)
        .fetch_optional(&pool)
        .await
        {
            Ok(opt) => opt,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "success": false, "error": e.to_string() })),
                )
                    .into_response();
            }
        };

        let letter = match raw {
            Some(l) => l,
            None => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "success": false,
                        "error": "Dokumen surat tidak ditemukan."
                    })),
                )
                    .into_response();
            }
        };

        // Jika warga biasa mencoba mengakses surat yang masih draft
        if !can_view_drafts && letter.status != "terkirim" {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "success": false,
                    "error": "Surat masih dalam status draft dan belum diterbitkan."
                })),
            )
                .into_response();
        }

        let local_time: DateTime<Local> = DateTime::from(letter.created_at);
        let date_formatted = local_time.format("%d %B %Y").to_string();
        let sent_at = letter.sent_at.map(|st| {
            DateTime::<Local>::from(st)
                .format("%d %B %Y %H:%M")
                .to_string()
        });

        let detail = ApiLetterDetailDto {
            id: letter.id,
            org_id,
            org_name,
            kind: letter.kind,
            number: letter.number,
            subject: letter.subject,
            recipient: letter.recipient,
            body: letter.body,
            status: letter.status,
            date_formatted,
            author_name: letter.author_name,
            sent_at,
        };

        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": detail,
            })),
        )
            .into_response()
    }

    /// Endpoint Mobile SvelteKit: GET /api/v1/surat/:id/pdf
    pub async fn export_pdf(req: Request, Path(letter_id): Path<Uuid>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        let org_name = auth
            .current_org_name
            .clone()
            .unwrap_or_else(|| "Paguyuban".to_string());

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "success": false, "error": e.to_string() })),
                )
                    .into_response();
            }
        };

        let can_view_drafts = auth.is_superadmin
            || matches!(
                auth.current_role.as_deref(),
                Some("admin") | Some("sekretaris")
            );

        #[derive(sqlx::FromRow)]
        struct RawPdfData {
            number: String,
            subject: String,
            body: String,
            recipient: String,
            status: String,
            created_at: DateTime<Utc>,
        }

        let raw: Option<RawPdfData> = match sqlx::query_as(
            r#"
            SELECT number, subject, body, recipient, status, created_at 
            FROM letters 
            WHERE id = $1 AND org_id = $2;
            "#,
        )
        .bind(letter_id)
        .bind(org_id)
        .fetch_optional(&pool)
        .await
        {
            Ok(opt) => opt,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "success": false, "error": e.to_string() })),
                )
                    .into_response();
            }
        };

        let letter = match raw {
            Some(l) => l,
            None => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "success": false,
                        "error": "Dokumen surat tidak ditemukan."
                    })),
                )
                    .into_response();
            }
        };

        if !can_view_drafts && letter.status != "terkirim" {
            return (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "success": false,
                    "error": "Dokumen surat masih dalam draft dan belum diterbitkan."
                })),
            )
                .into_response();
        }

        let local_time: DateTime<Local> = DateTime::from(letter.created_at);
        let date_str = local_time.format("%d %B %Y").to_string();

        let pdf_bytes = LetterPdfService::generate_pdf(
            &org_name,
            &letter.number,
            &letter.subject,
            &letter.recipient,
            &date_str,
            &letter.body,
        );

        let safe_filename = format!("surat_{}.pdf", letter.number.replace(['/', ' '], "_"));

        let headers = [
            (header::CONTENT_TYPE, "application/pdf"),
            (
                header::CONTENT_DISPOSITION,
                &format!("inline; filename=\"{}\"", safe_filename),
            ),
        ];

        (headers, pdf_bytes).into_response()
    }
}
