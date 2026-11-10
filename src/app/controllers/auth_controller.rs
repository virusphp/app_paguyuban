use axum::{
    extract::Form,
    response::{Html, IntoResponse, Redirect, Response},
};
use lumina::core::blade::BladeContext;
use lumina::prelude::Request;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::app::services::{login_rate_limiter, ViewService};

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub phone: String,
    pub password: String,
}

pub struct AuthController;

impl AuthController {
    /// Menampilkan form login (GET /login)
    pub async fn show_login(req: Request) -> Response {
        // Jika sudah login di sesi, langsung redirect ke dashboard
        if let Ok(Some(_user_id)) = req.session.get::<String>("user_id").await {
            return Redirect::to("/dashboard").into_response();
        }

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Masuk — Aplikasi Manajemen Paguyuban");

        // Ambil pesan error jika ada
        if let Ok(Some(err)) = req.session.get::<String>("flash_error").await {
            ctx.insert("error", &err);
            let _ = req.session.remove::<String>("flash_error").await;
        }

        let html = ViewService::render_blade("auth/login.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Memproses submit form login (POST /login)
    pub async fn login(req: Request, Form(payload): Form<LoginForm>) -> Response {
        let phone = payload.phone.trim();
        let password = payload.password.trim();

        if phone.is_empty() || password.is_empty() {
            let mut ctx = BladeContext::new();
            ctx.insert("title", "Masuk — Aplikasi Manajemen Paguyuban");
            ctx.insert("error", "Nomor HP dan kata sandi wajib diisi.");
            ctx.insert("old_phone", phone);
            let html = ViewService::render_blade("auth/login.blade.rs", &ctx);
            return Html(html).into_response();
        }

        // Rate Limiter: maksimal 5 percobaan gagal per 15 menit
        let window = std::time::Duration::from_secs(15 * 60);
        let max_attempts = 5;
        if let Err(remaining_secs) = login_rate_limiter().check_limit(phone, max_attempts, window) {
            let remaining_mins = remaining_secs.div_ceil(60);
            let mut ctx = BladeContext::new();
            ctx.insert("title", "Masuk — Aplikasi Manajemen Paguyuban");
            ctx.insert(
                "error",
                format!(
                    "Terlalu banyak percobaan masuk yang gagal. Silakan coba lagi dalam {} menit.",
                    remaining_mins
                ),
            );
            ctx.insert("old_phone", phone);
            let html = ViewService::render_blade("auth/login.blade.rs", &ctx);
            return Html(html).into_response();
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Database connection error: {}", e);
                return Html("<h1>Kesalahan koneksi database</h1>".to_string()).into_response();
            }
        };

        // 1. Cari pengguna berdasarkan nomor HP
        #[derive(sqlx::FromRow)]
        struct UserRecord {
            id: Uuid,
            name: String,
            password_hash: String,
            is_superadmin: bool,
            status: String,
        }

        let user_opt: Option<UserRecord> = sqlx::query_as(
            "SELECT id, name, password_hash, is_superadmin, status FROM users WHERE phone = $1",
        )
        .bind(phone)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);

        let user = match user_opt {
            Some(u) => u,
            None => {
                login_rate_limiter().record_failure(phone, window);
                let mut ctx = BladeContext::new();
                ctx.insert("title", "Masuk — Aplikasi Manajemen Paguyuban");
                ctx.insert("error", "Nomor HP atau kata sandi tidak cocok.");
                ctx.insert("old_phone", phone);
                let html = ViewService::render_blade("auth/login.blade.rs", &ctx);
                return Html(html).into_response();
            }
        };

        // 2. Verifikasi kata sandi bcrypt
        let is_valid = bcrypt::verify(password, &user.password_hash).unwrap_or(false);
        if !is_valid {
            login_rate_limiter().record_failure(phone, window);
            let mut ctx = BladeContext::new();
            ctx.insert("title", "Masuk — Aplikasi Manajemen Paguyuban");
            ctx.insert("error", "Nomor HP atau kata sandi tidak cocok.");
            ctx.insert("old_phone", phone);
            let html = ViewService::render_blade("auth/login.blade.rs", &ctx);
            return Html(html).into_response();
        }

        // Cek status akun
        if user.status != "aktif" {
            login_rate_limiter().record_failure(phone, window);
            let mut ctx = BladeContext::new();
            ctx.insert("title", "Masuk — Aplikasi Manajemen Paguyuban");
            ctx.insert(
                "error",
                "Akun Anda berstatus nonaktif atau sedang menunggu aktivasi.",
            );
            let html = ViewService::render_blade("auth/login.blade.rs", &ctx);
            return Html(html).into_response();
        }

        // Login berhasil: reset counter kegagalan
        login_rate_limiter().reset(phone);

        // 3. Update last_login_at
        let _ = sqlx::query("UPDATE users SET last_login_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(user.id)
            .execute(&pool)
            .await;

        // 4. Cari keanggotaan organisasi pengguna
        #[derive(sqlx::FromRow, serde::Serialize)]
        struct OrgMembership {
            id: Uuid,
            name: String,
            role: String,
        }

        let orgs: Vec<OrgMembership> = sqlx::query_as(
            r#"
            SELECT o.id, o.name, m.role::text as role
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

        // 5. Simpan informasi login ke sesi
        let _ = req.session.insert("user_id", user.id.to_string()).await;
        let _ = req.session.insert("user_name", user.name.clone()).await;
        let _ = req
            .session
            .insert("is_superadmin", user.is_superadmin)
            .await;

        let active_org_id = if let Some(first_org) = orgs.first() {
            let _ = req
                .session
                .insert("current_org_id", first_org.id.to_string())
                .await;
            let _ = req
                .session
                .insert("current_org_name", first_org.name.clone())
                .await;
            let _ = req
                .session
                .insert("current_role", first_org.role.clone())
                .await;
            Some(first_org.id)
        } else if user.is_superadmin {
            let _ = req
                .session
                .insert("current_org_name", "Superadmin".to_string())
                .await;
            let _ = req
                .session
                .insert("current_role", "superadmin".to_string())
                .await;
            None
        } else {
            None
        };

        // 6. Catat log aktivitas ke audit_logs
        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (org_id, actor_user_id, action, entity, meta)
            VALUES ($1, $2, 'LOGIN', 'users', $3);
            "#,
        )
        .bind(active_org_id)
        .bind(user.id)
        .bind(json!({ "phone": phone, "user_name": user.name }))
        .execute(&pool)
        .await;

        // 7. Redirect ke dashboard atau panel superadmin
        if user.is_superadmin && active_org_id.is_none() {
            Redirect::to("/sa/organisasi").into_response()
        } else {
            Redirect::to("/dashboard").into_response()
        }
    }

    /// Logout (GET atau POST /logout)
    pub async fn logout(req: Request) -> Response {
        let _ = req.session.clear().await;
        Redirect::to("/login").into_response()
    }
}
