use axum::{
    extract::{Form, Path},
    response::{Html, IntoResponse, Redirect, Response},
};
use chrono::{DateTime, Local, Utc};
use lumina::core::blade::BladeContext;
use lumina::prelude::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::app::services::AuthContext;
use crate::app::services::ViewService;

#[derive(Debug, Deserialize)]
pub struct AnnouncementForm {
    pub title: String,
    pub body: String,
    pub pinned: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AnnouncementItemView {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub pinned: bool,
    pub published_at_formatted: String,
    pub author_name: Option<String>,
}

pub struct AnnouncementController;

impl AnnouncementController {
    /// Daftar Papan Pengumuman Warga (GET /info)
    pub async fn index(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        let can_manage = auth.require_role(&["admin", "sekretaris"]).is_ok();

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let org_name: String = sqlx::query_scalar("SELECT name FROM organizations WHERE id = $1")
            .bind(org_id)
            .fetch_one(&pool)
            .await
            .unwrap_or_else(|_| "Paguyuban".to_string());

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

        let views: Vec<AnnouncementItemView> = raw_items
            .into_iter()
            .map(|a| {
                let local_time: DateTime<Local> = DateTime::from(a.published_at);
                AnnouncementItemView {
                    id: a.id,
                    title: a.title,
                    body: a.body,
                    pinned: a.pinned,
                    published_at_formatted: local_time.format("%d %b %Y %H:%M").to_string(),
                    author_name: a.author_name,
                }
            })
            .collect();

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Papan Informasi — Paguyuban");
        ctx.insert("area", "admin");
        ctx.insert("active", "info");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("can_manage", can_manage);
        ctx.insert("announcements", views);

        let html = ViewService::render_blade("announcements/index.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Form Buat Pengumuman Baru (GET /info/baru)
    pub async fn show_create(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "sekretaris"]) {
            return res;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let org_name: String = sqlx::query_scalar("SELECT name FROM organizations WHERE id = $1")
            .bind(org_id)
            .fetch_one(&pool)
            .await
            .unwrap_or_else(|_| "Paguyuban".to_string());

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Terbitkan Pengumuman Baru — Paguyuban");
        ctx.insert("area", "admin");
        ctx.insert("active", "info");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("is_edit", false);

        let html = ViewService::render_blade("announcements/form.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Simpan Pengumuman Baru (POST /info/baru)
    pub async fn create(req: Request, Form(form): Form<AnnouncementForm>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "sekretaris"]) {
            return res;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let is_pinned =
            form.pinned.as_deref() == Some("true") || form.pinned.as_deref() == Some("on");
        let item_id = Uuid::new_v4();
        let now = Utc::now();

        let insert_res = sqlx::query(
            r#"
            INSERT INTO announcements (id, org_id, title, body, pinned, published_at, created_by, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(item_id)
        .bind(org_id)
        .bind(form.title.trim())
        .bind(form.body.trim())
        .bind(is_pinned)
        .bind(now)
        .bind(auth.user_id)
        .bind(now)
        .execute(&pool)
        .await;

        if let Err(e) = insert_res {
            return Html(format!("<h1>Gagal Membuat Pengumuman: {}</h1>", e)).into_response();
        }

        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (id, org_id, user_id, action, target_type, target_id, details)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(org_id)
        .bind(auth.user_id)
        .bind("create_announcement")
        .bind("announcements")
        .bind(item_id)
        .bind(json!({
            "title": form.title.trim(),
            "pinned": is_pinned,
        }))
        .execute(&pool)
        .await;

        Redirect::to("/info").into_response()
    }

    /// Form Ubah Pengumuman (GET /info/:id/edit)
    pub async fn show_edit(req: Request, Path(id): Path<Uuid>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "sekretaris"]) {
            return res;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let org_name: String = sqlx::query_scalar("SELECT name FROM organizations WHERE id = $1")
            .bind(org_id)
            .fetch_one(&pool)
            .await
            .unwrap_or_else(|_| "Paguyuban".to_string());

        #[derive(sqlx::FromRow)]
        struct ExistingItem {
            id: Uuid,
            title: String,
            body: String,
            pinned: bool,
        }

        let item: Option<ExistingItem> = sqlx::query_as(
            r#"
            SELECT id, title, body, pinned
            FROM announcements
            WHERE id = $1 AND org_id = $2
            "#,
        )
        .bind(id)
        .bind(org_id)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);

        let item = match item {
            Some(i) => i,
            None => return Html("<h1>Pengumuman tidak ditemukan</h1>".to_string()).into_response(),
        };

        #[derive(Serialize)]
        struct EditFormView {
            id: Uuid,
            title: String,
            body: String,
            pinned: bool,
        }

        let view_data = EditFormView {
            id: item.id,
            title: item.title,
            body: item.body,
            pinned: item.pinned,
        };

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Ubah Pengumuman — Paguyuban");
        ctx.insert("area", "admin");
        ctx.insert("active", "info");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("is_edit", true);
        ctx.insert("announcement", view_data);

        let html = ViewService::render_blade("announcements/form.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Simpan Perubahan Pengumuman (POST /info/:id)
    pub async fn update(
        req: Request,
        Path(id): Path<Uuid>,
        Form(form): Form<AnnouncementForm>,
    ) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "sekretaris"]) {
            return res;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let is_pinned =
            form.pinned.as_deref() == Some("true") || form.pinned.as_deref() == Some("on");

        let upd_res = sqlx::query(
            r#"
            UPDATE announcements
            SET title = $1, body = $2, pinned = $3
            WHERE id = $4 AND org_id = $5
            "#,
        )
        .bind(form.title.trim())
        .bind(form.body.trim())
        .bind(is_pinned)
        .bind(id)
        .bind(org_id)
        .execute(&pool)
        .await;

        if let Err(e) = upd_res {
            return Html(format!("<h1>Gagal Memperbarui Pengumuman: {}</h1>", e)).into_response();
        }

        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (id, org_id, user_id, action, target_type, target_id, details)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(org_id)
        .bind(auth.user_id)
        .bind("update_announcement")
        .bind("announcements")
        .bind(id)
        .bind(json!({
            "title": form.title.trim(),
            "pinned": is_pinned,
        }))
        .execute(&pool)
        .await;

        Redirect::to("/info").into_response()
    }

    /// Toggle Sematkan / Pin Pengumuman (POST /info/:id/pin)
    pub async fn toggle_pin(req: Request, Path(id): Path<Uuid>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "sekretaris"]) {
            return res;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let upd_res = sqlx::query(
            r#"
            UPDATE announcements
            SET pinned = NOT pinned
            WHERE id = $1 AND org_id = $2
            "#,
        )
        .bind(id)
        .bind(org_id)
        .execute(&pool)
        .await;

        if let Err(e) = upd_res {
            return Html(format!("<h1>Gagal Mengubah Sematan: {}</h1>", e)).into_response();
        }

        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (id, org_id, user_id, action, target_type, target_id, details)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(org_id)
        .bind(auth.user_id)
        .bind("toggle_pin_announcement")
        .bind("announcements")
        .bind(id)
        .bind(json!({}))
        .execute(&pool)
        .await;

        Redirect::to("/info").into_response()
    }

    /// Hapus Pengumuman (POST /info/:id/hapus)
    pub async fn delete(req: Request, Path(id): Path<Uuid>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "sekretaris"]) {
            return res;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let del_res = sqlx::query(
            r#"
            DELETE FROM announcements
            WHERE id = $1 AND org_id = $2
            "#,
        )
        .bind(id)
        .bind(org_id)
        .execute(&pool)
        .await;

        if let Err(e) = del_res {
            return Html(format!("<h1>Gagal Menghapus Pengumuman: {}</h1>", e)).into_response();
        }

        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (id, org_id, user_id, action, target_type, target_id, details)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(org_id)
        .bind(auth.user_id)
        .bind("delete_announcement")
        .bind("announcements")
        .bind(id)
        .bind(json!({}))
        .execute(&pool)
        .await;

        Redirect::to("/info").into_response()
    }
}
