use axum::{
    extract::Query,
    response::{IntoResponse, Json, Response},
};
use lumina::prelude::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::app::services::AuthContext;

#[derive(Debug, Deserialize)]
pub struct ApiMemberQuery {
    pub q: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfilePayload {
    pub full_name: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize)]
pub struct FamilyMemberDto {
    pub id: Uuid,
    pub full_name: String,
    pub phone: Option<String>,
    pub status: String,
}

#[derive(Serialize)]
pub struct HouseholdDto {
    pub id: Uuid,
    pub house_no: String,
    pub note: Option<String>,
    pub family_members: Vec<FamilyMemberDto>,
}

#[derive(Serialize)]
pub struct MeProfileDto {
    pub id: Uuid,
    pub full_name: String,
    pub phone: Option<String>,
    pub status: String,
    pub joined_on: String,
    pub organization: Option<OrgDto>,
    pub household: Option<HouseholdDto>,
}

#[derive(Serialize)]
pub struct OrgDto {
    pub id: Uuid,
    pub name: String,
    pub role: String,
}

pub struct MemberApiController;

impl MemberApiController {
    /// Endpoint Mobile SvelteKit: GET /api/v1/me
    pub async fn get_me(req: Request) -> Response {
        let auth =
            match AuthContext::from_request(&req).await {
                Ok(ctx) => ctx,
                Err(_) => return (
                    axum::http::StatusCode::UNAUTHORIZED,
                    Json(json!({ "success": false, "error": "Sesi tidak valid atau belum login" })),
                )
                    .into_response(),
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

        // Cari data member terkait user_id atau phone
        #[derive(sqlx::FromRow)]
        #[allow(dead_code)]
        struct MemberRecord {
            id: Uuid,
            org_id: Uuid,
            household_id: Option<Uuid>,
            full_name: String,
            phone: Option<String>,
            status: String,
            joined_on: chrono::NaiveDate,
        }

        let member_record: Option<MemberRecord> = sqlx::query_as(
            r#"
            SELECT id, org_id, household_id, full_name, phone, status, joined_on
            FROM members
            WHERE user_id = $1
            LIMIT 1;
            "#,
        )
        .bind(auth.user_id)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);

        let member = match member_record {
            Some(m) => m,
            None => {
                // Buat fallback jika user adalah pengurus tapi belum terdata di members
                MemberRecord {
                    id: auth.user_id,
                    org_id: auth.current_org_id.unwrap_or_default(),
                    household_id: None,
                    full_name: auth.user_name.clone(),
                    phone: None,
                    status: "aktif".to_string(),
                    joined_on: chrono::Local::now().date_naive(),
                }
            }
        };

        // Ambil data household dan anggota keluarga jika ada
        let mut household_dto: Option<HouseholdDto> = None;
        if let Some(hid) = member.household_id {
            #[derive(sqlx::FromRow)]
            struct HouseRow {
                id: Uuid,
                house_no: String,
                note: Option<String>,
            }

            if let Ok(Some(h)) = sqlx::query_as::<_, HouseRow>(
                "SELECT id, house_no, note FROM households WHERE id = $1",
            )
            .bind(hid)
            .fetch_optional(&pool)
            .await
            {
                #[derive(sqlx::FromRow)]
                struct FamilyRow {
                    id: Uuid,
                    full_name: String,
                    phone: Option<String>,
                    status: String,
                }

                let families: Vec<FamilyRow> = sqlx::query_as(
                    "SELECT id, full_name, phone, status FROM members WHERE household_id = $1 AND id != $2 ORDER BY full_name ASC",
                )
                .bind(hid)
                .bind(member.id)
                .fetch_all(&pool)
                .await
                .unwrap_or_default();

                household_dto = Some(HouseholdDto {
                    id: h.id,
                    house_no: h.house_no,
                    note: h.note,
                    family_members: families
                        .into_iter()
                        .map(|f| FamilyMemberDto {
                            id: f.id,
                            full_name: f.full_name,
                            phone: f.phone,
                            status: f.status,
                        })
                        .collect(),
                });
            }
        }

        let org_dto = auth.current_org_id.map(|oid| OrgDto {
            id: oid,
            name: auth
                .current_org_name
                .clone()
                .unwrap_or_else(|| "Paguyuban".to_string()),
            role: auth
                .current_role
                .clone()
                .unwrap_or_else(|| "anggota".to_string()),
        });

        let profile = MeProfileDto {
            id: member.id,
            full_name: member.full_name,
            phone: member.phone,
            status: member.status,
            joined_on: member.joined_on.format("%Y-%m-%d").to_string(),
            organization: org_dto,
            household: household_dto,
        };

        Json(json!({
            "success": true,
            "data": profile
        }))
        .into_response()
    }

    /// Endpoint Mobile SvelteKit: PUT /api/v1/me
    pub async fn update_me(req: Request, Json(payload): Json<UpdateProfilePayload>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(_) => {
                return (
                    axum::http::StatusCode::UNAUTHORIZED,
                    Json(json!({ "success": false, "error": "Sesi tidak valid" })),
                )
                    .into_response()
            }
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

        if let Some(ref name) = payload.full_name {
            let _ = sqlx::query("UPDATE users SET name = $1 WHERE id = $2")
                .bind(name)
                .bind(auth.user_id)
                .execute(&pool)
                .await;

            let _ = sqlx::query("UPDATE members SET full_name = $1 WHERE user_id = $2")
                .bind(name)
                .bind(auth.user_id)
                .execute(&pool)
                .await;
        }

        if let Some(ref phone) = payload.phone {
            let _ = sqlx::query("UPDATE members SET phone = $1 WHERE user_id = $2")
                .bind(phone)
                .bind(auth.user_id)
                .execute(&pool)
                .await;
        }

        Json(json!({
            "success": true,
            "message": "Profil berhasil diperbarui"
        }))
        .into_response()
    }

    /// Endpoint Mobile SvelteKit: GET /api/v1/members
    pub async fn list_members(req: Request, Query(params): Query<ApiMemberQuery>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(_) => {
                return (
                    axum::http::StatusCode::UNAUTHORIZED,
                    Json(json!({ "success": false, "error": "Sesi tidak valid" })),
                )
                    .into_response()
            }
        };

        let org_id = match auth.require_org() {
            Ok(oid) => oid,
            Err(_) => {
                return (
                    axum::http::StatusCode::BAD_REQUEST,
                    Json(json!({ "success": false, "error": "Organisasi aktif belum dipilih" })),
                )
                    .into_response()
            }
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

        let search_text = params.q.unwrap_or_default().trim().to_string();
        let status_filter = params.status.unwrap_or_default().trim().to_string();
        let search_pattern = format!("%{}%", search_text);

        #[derive(sqlx::FromRow, Serialize)]
        struct MemberApiDto {
            id: Uuid,
            full_name: String,
            phone: Option<String>,
            status: String,
            house_no: Option<String>,
        }

        let members: Vec<MemberApiDto> = if !search_text.is_empty() && !status_filter.is_empty() {
            sqlx::query_as(
                r#"
                SELECT m.id, m.full_name, m.phone, m.status, h.house_no
                FROM members m
                LEFT JOIN households h ON m.household_id = h.id
                WHERE m.org_id = $1 AND m.status = $2 AND (m.full_name ILIKE $3 OR h.house_no ILIKE $3)
                ORDER BY h.house_no ASC NULLS LAST, m.full_name ASC;
                "#,
            )
            .bind(org_id)
            .bind(&status_filter)
            .bind(&search_pattern)
            .fetch_all(&pool)
            .await
            .unwrap_or_default()
        } else if !search_text.is_empty() {
            sqlx::query_as(
                r#"
                SELECT m.id, m.full_name, m.phone, m.status, h.house_no
                FROM members m
                LEFT JOIN households h ON m.household_id = h.id
                WHERE m.org_id = $1 AND (m.full_name ILIKE $2 OR h.house_no ILIKE $2)
                ORDER BY h.house_no ASC NULLS LAST, m.full_name ASC;
                "#,
            )
            .bind(org_id)
            .bind(&search_pattern)
            .fetch_all(&pool)
            .await
            .unwrap_or_default()
        } else if !status_filter.is_empty() {
            sqlx::query_as(
                r#"
                SELECT m.id, m.full_name, m.phone, m.status, h.house_no
                FROM members m
                LEFT JOIN households h ON m.household_id = h.id
                WHERE m.org_id = $1 AND m.status = $2
                ORDER BY h.house_no ASC NULLS LAST, m.full_name ASC;
                "#,
            )
            .bind(org_id)
            .bind(&status_filter)
            .fetch_all(&pool)
            .await
            .unwrap_or_default()
        } else {
            sqlx::query_as(
                r#"
                SELECT m.id, m.full_name, m.phone, m.status, h.house_no
                FROM members m
                LEFT JOIN households h ON m.household_id = h.id
                WHERE m.org_id = $1
                ORDER BY h.house_no ASC NULLS LAST, m.full_name ASC;
                "#,
            )
            .bind(org_id)
            .fetch_all(&pool)
            .await
            .unwrap_or_default()
        };

        let total = members.len();

        Json(json!({
            "success": true,
            "total": total,
            "data": members
        }))
        .into_response()
    }
}
