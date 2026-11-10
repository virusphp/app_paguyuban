use axum::response::{IntoResponse, Json, Response};
use chrono::{DateTime, Utc};
use lumina::prelude::Request;
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

use crate::app::services::AuthContext;

#[derive(Debug, Serialize)]
pub struct ApiAnnouncementDto {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub pinned: bool,
    pub published_at: String,
    pub author_name: Option<String>,
}

pub struct AnnouncementApiController;

impl AnnouncementApiController {
    /// Endpoint Mobile SvelteKit: GET /api/v1/info
    pub async fn list_announcements(req: Request) -> Response {
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
            Err(res) => return res,
        };

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

        #[derive(sqlx::FromRow)]
        struct RawAnnouncement {
            id: Uuid,
            title: String,
            body: String,
            pinned: bool,
            published_at: DateTime<Utc>,
            author_name: Option<String>,
        }

        let raw_items: Vec<RawAnnouncement> = sqlx::query_as(
            r#"
            SELECT a.id, a.title, a.body, a.pinned, a.published_at, u.name as author_name
            FROM announcements a
            LEFT JOIN users u ON a.created_by = u.id
            WHERE a.org_id = $1
            ORDER BY a.pinned DESC, a.published_at DESC
            "#,
        )
        .bind(org_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let dtos: Vec<ApiAnnouncementDto> = raw_items
            .into_iter()
            .map(|a| ApiAnnouncementDto {
                id: a.id,
                title: a.title,
                body: a.body,
                pinned: a.pinned,
                published_at: a.published_at.to_rfc3339(),
                author_name: a.author_name,
            })
            .collect();

        Json(json!({
            "success": true,
            "data": dtos
        }))
        .into_response()
    }
}
