use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use lumina::core::auth::AuthUser;
use lumina::prelude::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
use uuid::Uuid;

use crate::app::services::{login_rate_limiter, AuthContext};

#[derive(Debug, Deserialize)]
pub struct ApiLoginPayload {
    pub phone: String,
    pub password: String,
    pub org_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct SwitchOrgPayload {
    pub org_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct OrgSummaryDto {
    pub id: Uuid,
    pub name: String,
    pub org_type: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct UserSummaryDto {
    pub id: Uuid,
    pub name: String,
    pub phone: String,
    pub is_superadmin: bool,
    pub role: Option<String>,
}

pub struct AuthApiController;

impl AuthApiController {
    /// Endpoint Mobile SvelteKit: POST /api/v1/auth/login
    pub async fn login(_req: Request, Json(payload): Json<ApiLoginPayload>) -> Response {
        let phone = payload.phone.trim();
        let password = payload.password.trim();

        if phone.is_empty() || password.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "error": "Nomor HP dan kata sandi wajib diisi."
                })),
            )
                .into_response();
        }

        // 1. Rate Limiting: Maksimal 5 percobaan gagal per 15 menit
        let window = Duration::from_secs(15 * 60);
        let max_attempts = 5;
        if let Err(remaining_secs) = login_rate_limiter().check_limit(phone, max_attempts, window) {
            let remaining_mins = remaining_secs.div_ceil(60);
            return (
                StatusCode::TOO_MANY_REQUESTS,
                Json(json!({
                    "success": false,
                    "error": format!(
                        "Terlalu banyak percobaan masuk yang gagal. Silakan coba lagi dalam {} menit.",
                        remaining_mins
                    ),
                    "retry_after_seconds": remaining_secs
                })),
            )
                .into_response();
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "success": false,
                        "error": format!("Kesalahan koneksi basis data: {}", e)
                    })),
                )
                    .into_response();
            }
        };

        // 2. Cari pengguna berdasarkan nomor HP
        #[derive(sqlx::FromRow)]
        struct UserRecord {
            id: Uuid,
            name: String,
            phone: String,
            password_hash: String,
            is_superadmin: bool,
            status: String,
        }

        let user_opt: Option<UserRecord> = sqlx::query_as(
            "SELECT id, name, phone, password_hash, is_superadmin, status FROM users WHERE phone = $1",
        )
        .bind(phone)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);

        let user = match user_opt {
            Some(u) => u,
            None => {
                login_rate_limiter().record_failure(phone, window);
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "success": false,
                        "error": "Nomor HP atau kata sandi tidak cocok."
                    })),
                )
                    .into_response();
            }
        };

        // 3. Verifikasi kata sandi bcrypt
        let is_valid = bcrypt::verify(password, &user.password_hash).unwrap_or(false);
        if !is_valid {
            login_rate_limiter().record_failure(phone, window);
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "success": false,
                    "error": "Nomor HP atau kata sandi tidak cocok."
                })),
            )
                .into_response();
        }

        // 4. Verifikasi status akun aktif
        if user.status != "aktif" {
            login_rate_limiter().record_failure(phone, window);
            return (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "success": false,
                    "error": "Akun Anda berstatus nonaktif atau sedang menunggu aktivasi."
                })),
            )
                .into_response();
        }

        // Login sukses: reset pencatat percobaan gagal
        login_rate_limiter().reset(phone);

        // 5. Perbarui waktu masuk terakhir (last_login_at)
        let _ = sqlx::query("UPDATE users SET last_login_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(user.id)
            .execute(&pool)
            .await;

        // 6. Ambil seluruh keanggotaan organisasi yang aktif
        #[derive(sqlx::FromRow)]
        struct OrgRow {
            id: Uuid,
            name: String,
            org_type: String,
            role: String,
        }

        let org_rows: Vec<OrgRow> = sqlx::query_as(
            r#"
            SELECT o.id, o.name, o.type::text as org_type, m.role::text as role
            FROM memberships m
            JOIN organizations o ON m.org_id = o.id
            WHERE m.user_id = $1 AND o.status = 'aktif'
            ORDER BY o.name ASC;
            "#,
        )
        .bind(user.id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let available_orgs: Vec<OrgSummaryDto> = org_rows
            .iter()
            .map(|r| OrgSummaryDto {
                id: r.id,
                name: r.name.clone(),
                org_type: r.org_type.clone(),
                role: r.role.clone(),
            })
            .collect();

        // 7. Pilih organisasi aktif
        let selected_org = if let Some(target_org_id) = payload.org_id {
            match available_orgs.iter().find(|o| o.id == target_org_id) {
                Some(org) => Some(org),
                None => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({
                            "success": false,
                            "error": "Organisasi yang diminta tidak valid atau Anda bukan anggota aktif."
                        })),
                    )
                        .into_response();
                }
            }
        } else {
            available_orgs.first()
        };

        let selected_role = selected_org.map(|o| o.role.clone()).unwrap_or_else(|| {
            if user.is_superadmin {
                "superadmin".to_string()
            } else {
                "anggota".to_string()
            }
        });

        // 8. Buat JWT Bearer Token Lumina
        let mut permissions = Vec::new();
        if let Some(org) = selected_org {
            permissions.push(format!("org_id:{}", org.id));
            permissions.push(format!("org_name:{}", org.name));
            permissions.push(format!("org_type:{}", org.org_type));
        }
        if user.is_superadmin {
            permissions.push("is_superadmin:true".to_string());
        }

        let mut auth_user = AuthUser::with_name(
            1, // Placeholder integer ID internal Lumina
            &user.name,
            user.phone.clone(),
            selected_role.clone(),
            permissions,
            24 * 30, // Berlaku 30 hari (720 jam) untuk kemudahan PWA mobile
        );
        auth_user.sub = user.id.to_string();

        let token = match lumina::http::auth::generate_token(&auth_user) {
            Ok(t) => t,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "success": false,
                        "error": format!("Gagal menerbitkan token JWT: {}", e)
                    })),
                )
                    .into_response();
            }
        };

        // 9. Catat ke audit_logs
        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (org_id, actor_user_id, action, entity, meta)
            VALUES ($1, $2, 'LOGIN_API_MOBILE', 'users', $3);
            "#,
        )
        .bind(selected_org.map(|o| o.id))
        .bind(user.id)
        .bind(json!({
            "phone": phone,
            "user_name": user.name,
            "role": selected_role,
            "client": "mobile_pwa"
        }))
        .execute(&pool)
        .await;

        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Autentikasi berhasil.",
                "token": token,
                "token_type": "Bearer",
                "expires_in_hours": 720,
                "user": UserSummaryDto {
                    id: user.id,
                    name: user.name,
                    phone: user.phone,
                    is_superadmin: user.is_superadmin,
                    role: Some(selected_role),
                },
                "organization": selected_org,
                "available_organizations": available_orgs,
            })),
        )
            .into_response()
    }

    /// Endpoint Mobile SvelteKit: GET /api/v1/auth/me
    pub async fn me(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(resp) => return resp,
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

        #[derive(sqlx::FromRow)]
        #[allow(dead_code)]
        struct UserProfile {
            id: Uuid,
            name: String,
            phone: String,
            is_superadmin: bool,
            status: String,
        }

        let user: Option<UserProfile> = sqlx::query_as(
            "SELECT id, name, phone, is_superadmin, status FROM users WHERE id = $1",
        )
        .bind(auth.user_id)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);

        let user = match user {
            Some(u) => u,
            None => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({ "success": false, "error": "Pengguna tidak ditemukan." })),
                )
                    .into_response();
            }
        };

        // Ambil daftar organisasi yang dapat diakses pengguna
        #[derive(sqlx::FromRow)]
        struct OrgRow {
            id: Uuid,
            name: String,
            org_type: String,
            role: String,
        }

        let org_rows: Vec<OrgRow> = sqlx::query_as(
            r#"
            SELECT o.id, o.name, o.type::text as org_type, m.role::text as role
            FROM memberships m
            JOIN organizations o ON m.org_id = o.id
            WHERE m.user_id = $1 AND o.status = 'aktif'
            ORDER BY o.name ASC;
            "#,
        )
        .bind(auth.user_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let available_orgs: Vec<OrgSummaryDto> = org_rows
            .into_iter()
            .map(|r| OrgSummaryDto {
                id: r.id,
                name: r.name,
                org_type: r.org_type,
                role: r.role,
            })
            .collect();

        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "user": UserSummaryDto {
                    id: user.id,
                    name: user.name,
                    phone: user.phone,
                    is_superadmin: user.is_superadmin,
                    role: auth.current_role.clone(),
                },
                "active_organization": {
                    "id": auth.current_org_id,
                    "name": auth.current_org_name,
                    "role": auth.current_role,
                },
                "available_organizations": available_orgs,
            })),
        )
            .into_response()
    }

    /// Endpoint Mobile SvelteKit: POST /api/v1/auth/switch-org
    pub async fn switch_org(req: Request, Json(payload): Json<SwitchOrgPayload>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(resp) => return resp,
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

        // Verifikasi keanggotaan pada organisasi tujuan
        #[derive(sqlx::FromRow)]
        struct MembershipTarget {
            org_id: Uuid,
            org_name: String,
            org_type: String,
            role: String,
        }

        let target_membership: Option<MembershipTarget> = sqlx::query_as(
            r#"
            SELECT o.id as org_id, o.name as org_name, o.type::text as org_type, m.role::text as role
            FROM memberships m
            JOIN organizations o ON m.org_id = o.id
            WHERE m.user_id = $1 AND m.org_id = $2 AND o.status = 'aktif';
            "#,
        )
        .bind(auth.user_id)
        .bind(payload.org_id)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);

        let target = match target_membership {
            Some(m) => m,
            None => {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({
                        "success": false,
                        "error": "Anda tidak memiliki keanggotaan aktif pada organisasi tersebut."
                    })),
                )
                    .into_response();
            }
        };

        // Terbitkan token baru dengan konteks organisasi tujuan
        let mut permissions = vec![
            format!("org_id:{}", target.org_id),
            format!("org_name:{}", target.org_name),
            format!("org_type:{}", target.org_type),
        ];
        if auth.is_superadmin {
            permissions.push("is_superadmin:true".to_string());
        }

        let mut auth_user = AuthUser::with_name(
            1,
            &auth.user_name,
            "user@paguyuban.local".to_string(),
            target.role.clone(),
            permissions,
            24 * 30,
        );
        auth_user.sub = auth.user_id.to_string();

        let token = match lumina::http::auth::generate_token(&auth_user) {
            Ok(t) => t,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "success": false,
                        "error": format!("Gagal memperbarui token: {}", e)
                    })),
                )
                    .into_response();
            }
        };

        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": format!("Berhasil beralih ke organisasi {}.", target.org_name),
                "token": token,
                "token_type": "Bearer",
                "organization": OrgSummaryDto {
                    id: target.org_id,
                    name: target.org_name,
                    org_type: target.org_type,
                    role: target.role,
                }
            })),
        )
            .into_response()
    }

    /// Endpoint Mobile SvelteKit: POST /api/v1/auth/logout
    pub async fn logout(req: Request) -> Response {
        // Jika ada header Authorization Bearer, masukkan token ke daftar pembatalan (blacklist)
        if let Some(auth_val) = req
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
        {
            if let Some(token) = auth_val.strip_prefix("Bearer ") {
                let token = token.trim();
                lumina::core::auth::TokenBlacklist::revoke(token).await;
            }
        }

        // Hapus juga sesi cookie jika ada
        let _ = req.session.clear().await;

        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Berhasil keluar dari aplikasi."
            })),
        )
            .into_response()
    }
}
