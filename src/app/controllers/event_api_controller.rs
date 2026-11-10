use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Json, Response},
};
use chrono::{DateTime, Utc};
use lumina::prelude::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::app::services::AuthContext;

#[derive(Debug, Deserialize)]
pub struct EventApiQuery {
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SendRsvpPayload {
    pub member_id: Option<Uuid>,
    pub answer: String,
}

#[derive(Debug, Serialize)]
pub struct ApiEventDto {
    pub id: Uuid,
    pub title: String,
    pub starts_at: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub attending_count: i64,
    pub absent_count: i64,
    pub my_rsvp: Option<String>,
}

pub struct EventApiController;

impl EventApiController {
    /// Endpoint Mobile SvelteKit: GET /api/v1/kegiatan
    pub async fn list_events(req: Request, Query(query): Query<EventApiQuery>) -> Response {
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

        // Cari member_id milik auth.user_id jika ada
        let my_member_id: Option<Uuid> =
            sqlx::query_scalar("SELECT id FROM members WHERE user_id = $1 AND org_id = $2 LIMIT 1")
                .bind(auth.user_id)
                .bind(org_id)
                .fetch_optional(&pool)
                .await
                .unwrap_or(None);

        #[derive(sqlx::FromRow)]
        struct RawEvent {
            id: Uuid,
            title: String,
            starts_at: DateTime<Utc>,
            location: Option<String>,
            description: Option<String>,
            status: String,
        }

        let raw_events: Vec<RawEvent> = if let Some(status_filter) = &query.status {
            if status_filter != "semua" {
                sqlx::query_as(
                    r#"
                    SELECT id, title, starts_at, location, description, status
                    FROM events
                    WHERE org_id = $1 AND status = $2
                    ORDER BY starts_at ASC
                    "#,
                )
                .bind(org_id)
                .bind(status_filter)
                .fetch_all(&pool)
                .await
                .unwrap_or_default()
            } else {
                sqlx::query_as(
                    r#"
                    SELECT id, title, starts_at, location, description, status
                    FROM events
                    WHERE org_id = $1
                    ORDER BY starts_at ASC
                    "#,
                )
                .bind(org_id)
                .fetch_all(&pool)
                .await
                .unwrap_or_default()
            }
        } else {
            sqlx::query_as(
                r#"
                SELECT id, title, starts_at, location, description, status
                FROM events
                WHERE org_id = $1
                ORDER BY starts_at ASC
                "#,
            )
            .bind(org_id)
            .fetch_all(&pool)
            .await
            .unwrap_or_default()
        };

        let mut event_dtos = Vec::new();

        for ev in raw_events {
            let attending: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM event_rsvps WHERE event_id = $1 AND answer = 'hadir'",
            )
            .bind(ev.id)
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

            let absent: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM event_rsvps WHERE event_id = $1 AND answer = 'tidak'",
            )
            .bind(ev.id)
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

            let my_rsvp: Option<String> = if let Some(mid) = my_member_id {
                sqlx::query_scalar(
                    "SELECT answer FROM event_rsvps WHERE event_id = $1 AND member_id = $2",
                )
                .bind(ev.id)
                .bind(mid)
                .fetch_optional(&pool)
                .await
                .unwrap_or(None)
            } else {
                None
            };

            event_dtos.push(ApiEventDto {
                id: ev.id,
                title: ev.title,
                starts_at: ev.starts_at.to_rfc3339(),
                location: ev.location,
                description: ev.description,
                status: ev.status,
                attending_count: attending,
                absent_count: absent,
                my_rsvp,
            });
        }

        Json(json!({
            "success": true,
            "data": event_dtos
        }))
        .into_response()
    }

    /// Endpoint Mobile SvelteKit: POST /api/v1/kegiatan/:id/rsvp
    pub async fn send_rsvp(
        req: Request,
        Path(event_id): Path<Uuid>,
        Json(payload): Json<SendRsvpPayload>,
    ) -> Response {
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

        let answer = payload.answer.trim().to_lowercase();
        if answer != "hadir" && answer != "tidak" {
            return (
                axum::http::StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "error": "Jawaban RSVP tidak valid. Pilih antara 'hadir' atau 'tidak'"
                })),
            )
                .into_response();
        }

        // Tentukan member_id: jika diberikan dalam payload gunakan itu (jika berhak), atau gunakan member linked to user
        let target_member_id = if let Some(mid) = payload.member_id {
            mid
        } else {
            let my_mid: Option<Uuid> = sqlx::query_scalar(
                "SELECT id FROM members WHERE user_id = $1 AND org_id = $2 LIMIT 1",
            )
            .bind(auth.user_id)
            .bind(org_id)
            .fetch_optional(&pool)
            .await
            .unwrap_or(None);

            match my_mid {
                Some(mid) => mid,
                None => {
                    return (
                        axum::http::StatusCode::BAD_REQUEST,
                        Json(json!({
                            "success": false,
                            "error": "Akun Anda belum terhubung dengan data warga aktif."
                        })),
                    )
                        .into_response();
                }
            }
        };

        // Pastikan acara ada dalam org_id yang sama
        let event_exists: Option<Uuid> =
            sqlx::query_scalar("SELECT id FROM events WHERE id = $1 AND org_id = $2")
                .bind(event_id)
                .bind(org_id)
                .fetch_optional(&pool)
                .await
                .unwrap_or(None);

        if event_exists.is_none() {
            return (
                axum::http::StatusCode::NOT_FOUND,
                Json(json!({ "success": false, "error": "Kegiatan tidak ditemukan" })),
            )
                .into_response();
        }

        let rsvp_id = Uuid::new_v4();
        let res = sqlx::query(
            r#"
            INSERT INTO event_rsvps (id, org_id, event_id, member_id, answer)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (event_id, member_id) DO UPDATE SET answer = EXCLUDED.answer
            "#,
        )
        .bind(rsvp_id)
        .bind(org_id)
        .bind(event_id)
        .bind(target_member_id)
        .bind(&answer)
        .execute(&pool)
        .await;

        if let Err(e) = res {
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "error": format!("Gagal menyimpan RSVP: {}", e) })),
            )
                .into_response();
        }

        Json(json!({
            "success": true,
            "message": "RSVP berhasil disimpan",
            "data": {
                "event_id": event_id,
                "member_id": target_member_id,
                "answer": answer
            }
        }))
        .into_response()
    }
}
