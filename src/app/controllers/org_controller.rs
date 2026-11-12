use axum::{
    extract::Form,
    response::{Html, IntoResponse, Redirect, Response},
};
use lumina::core::blade::BladeContext;
use lumina::prelude::Request;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::app::services::{AuthContext, ViewService};

#[derive(Debug, Deserialize)]
pub struct SwitchOrgForm {
    pub org_id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrgForm {
    pub name: String,
}

pub struct OrgController;

impl OrgController {
    /// Halaman Pilih Organisasi (GET /pilih-organisasi)
    pub async fn choose_org_view(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let user_id = auth.user_id;
        let is_superadmin = auth.is_superadmin;

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(_) => return Redirect::to("/login").into_response(),
        };

        #[derive(serde::Serialize)]
        struct OrgCard {
            id: String,
            name: String,
            org_type: String,
            role: String,
            members_count: i64,
            is_active: bool,
        }

        let orgs: Vec<OrgCard> = if is_superadmin {
            let rows: Vec<(Uuid, String, String, i64)> = sqlx::query_as(
                r#"
                SELECT o.id, o.name, o.type::text as org_type, 
                       COALESCE((SELECT COUNT(*) FROM members m WHERE m.org_id = o.id AND m.status = 'aktif'), 0) as members_count
                FROM organizations o
                WHERE o.status = 'aktif'
                ORDER BY o.name ASC;
                "#,
            )
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

            rows.into_iter()
                .map(|(id, name, org_type, count)| {
                    let is_current = auth.current_org_id.map(|cid| cid == id).unwrap_or(false);
                    OrgCard {
                        id: id.to_string(),
                        name,
                        org_type,
                        role: "Superadmin (Akses Penuh)".to_string(),
                        members_count: count,
                        is_active: is_current,
                    }
                })
                .collect()
        } else {
            let rows: Vec<(Uuid, String, String, String, i64)> = sqlx::query_as(
                r#"
                SELECT o.id, o.name, o.type::text as org_type, m.role::text as role,
                       COALESCE((SELECT COUNT(*) FROM members mem WHERE mem.org_id = o.id AND mem.status = 'aktif'), 0) as members_count
                FROM memberships m
                JOIN organizations o ON m.org_id = o.id
                WHERE m.user_id = $1 AND o.status = 'aktif'
                ORDER BY o.name ASC;
                "#,
            )
            .bind(user_id)
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

            rows.into_iter()
                .map(|(id, name, org_type, role, count)| {
                    let is_current = auth.current_org_id.map(|cid| cid == id).unwrap_or(false);
                    OrgCard {
                        id: id.to_string(),
                        name,
                        org_type,
                        role,
                        members_count: count,
                        is_active: is_current,
                    }
                })
                .collect()
        };

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Pilih Organisasi — Aplikasi Paguyuban");
        ctx.insert("user_name", &auth.user_name);
        ctx.insert("is_superadmin", is_superadmin);
        ctx.insert("orgs", orgs);

        let html = ViewService::render_blade("org/choose.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Beralih organisasi aktif (POST /pilih-organisasi)
    pub async fn switch_org(req: Request, Form(payload): Form<SwitchOrgForm>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let user_id = auth.user_id;
        let is_superadmin = auth.is_superadmin;

        let target_org_id = match Uuid::parse_str(payload.org_id.trim()) {
            Ok(u) => u,
            Err(_) => return Redirect::to("/dashboard").into_response(),
        };

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(_) => return Redirect::to("/dashboard").into_response(),
        };

        // Verifikasi keanggotaan di organisasi target (atau lolos jika superadmin)
        #[derive(sqlx::FromRow)]
        struct TargetOrg {
            name: String,
            role: Option<String>,
        }

        let org_info: Option<TargetOrg> = if is_superadmin {
            sqlx::query_as(
                r#"
                SELECT o.name, m.role::text as role
                FROM organizations o
                LEFT JOIN memberships m ON o.id = m.org_id AND m.user_id = $1
                WHERE o.id = $2 AND o.status = 'aktif';
                "#,
            )
            .bind(user_id)
            .bind(target_org_id)
            .fetch_optional(&pool)
            .await
            .unwrap_or(None)
        } else {
            sqlx::query_as(
                r#"
                SELECT o.name, m.role::text as role
                FROM memberships m
                JOIN organizations o ON m.org_id = o.id
                WHERE m.user_id = $1 AND o.id = $2 AND o.status = 'aktif';
                "#,
            )
            .bind(user_id)
            .bind(target_org_id)
            .fetch_optional(&pool)
            .await
            .unwrap_or(None)
        };

        if let Some(org) = org_info {
            let role = org.role.unwrap_or_else(|| {
                if is_superadmin {
                    "admin".to_string()
                } else {
                    "anggota".to_string()
                }
            });

            let _ = req
                .session
                .insert("current_org_id", target_org_id.to_string())
                .await;
            let _ = req
                .session
                .insert("current_org_name", org.name.clone())
                .await;
            let _ = req.session.insert("current_role", role.clone()).await;

            // Catat log
            let _ = sqlx::query(
                r#"
                INSERT INTO audit_logs (org_id, actor_user_id, action, entity, meta)
                VALUES ($1, $2, 'SWITCH_ORG', 'organizations', $3);
                "#,
            )
            .bind(Some(target_org_id))
            .bind(user_id)
            .bind(json!({ "new_org_id": target_org_id, "new_org_name": org.name }))
            .execute(&pool)
            .await;
        }

        Redirect::to("/dashboard").into_response()
    }

    /// Halaman Pengaturan Organisasi (GET /pengaturan)
    pub async fn settings(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        let can_edit = auth.require_role(&["admin"]).is_ok();
        let user_name = auth.user_name.clone();
        let role_label = auth
            .current_role
            .clone()
            .unwrap_or_else(|| "Pengurus".to_string());

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(_) => return Redirect::to("/dashboard").into_response(),
        };

        // Ambil data organisasi
        #[derive(sqlx::FromRow, serde::Serialize)]
        struct OrgDetail {
            id: Uuid,
            name: String,
            r#type: String,
            status: String,
            created_at: chrono::DateTime<chrono::Utc>,
        }

        let org_detail: Option<OrgDetail> = sqlx::query_as(
            r#"
            SELECT id, name, type::text as type, status::text as status, created_at
            FROM organizations
            WHERE id = $1;
            "#,
        )
        .bind(org_id)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);

        let org = match org_detail {
            Some(o) => o,
            None => return Redirect::to("/dashboard").into_response(),
        };

        // Ambil daftar pengurus aktif (admin, bendahara, sekretaris)
        #[derive(sqlx::FromRow, serde::Serialize)]
        struct OfficerItem {
            name: String,
            phone: String,
            role: String,
        }

        let officers: Vec<OfficerItem> = sqlx::query_as(
            r#"
            SELECT u.name, u.phone, m.role::text as role
            FROM memberships m
            JOIN users u ON m.user_id = u.id
            WHERE m.org_id = $1 AND m.role IN ('admin', 'bendahara', 'sekretaris')
            ORDER BY 
                CASE m.role 
                    WHEN 'admin' THEN 1 
                    WHEN 'sekretaris' THEN 2 
                    WHEN 'bendahara' THEN 3 
                    ELSE 4 
                END;
            "#,
        )
        .bind(org_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let wib = chrono::FixedOffset::east_opt(7 * 3600).unwrap();
        let created_date_formatted = org
            .created_at
            .with_timezone(&wib)
            .format("%d %B %Y")
            .to_string();

        let mut ctx = lumina::core::blade::BladeContext::new();
        ctx.insert("title", format!("Pengaturan — {}", org.name));
        ctx.insert("active", "pengaturan");
        ctx.insert("area", "admin");
        ctx.insert("user_name", &user_name);
        ctx.insert("org_name", &org.name);
        ctx.insert("role_label", role_label.to_uppercase());
        ctx.insert("current_org_id", org_id.to_string());
        ctx.insert("can_edit", can_edit);
        ctx.insert("org_id", org.id.to_string());
        ctx.insert("org_type", org.r#type.to_uppercase());
        ctx.insert("org_status", org.status);
        ctx.insert("created_date", created_date_formatted);
        ctx.insert("officers", officers);

        let html = crate::app::services::ViewService::render_blade("org/settings.blade.rs", &ctx);
        axum::response::Html(html).into_response()
    }

    /// Ubah Pengaturan Organisasi (POST /pengaturan)
    pub async fn update_settings(req: Request, Form(payload): Form<UpdateOrgForm>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin"]) {
            return res;
        }

        let new_name = payload.name.trim();
        if new_name.is_empty() {
            return Redirect::to("/pengaturan").into_response();
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(_) => return Redirect::to("/pengaturan").into_response(),
        };

        let _ = sqlx::query(
            "UPDATE organizations SET name = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2;",
        )
        .bind(new_name)
        .bind(org_id)
        .execute(&pool)
        .await;

        let _ = req
            .session
            .insert("current_org_name", new_name.to_string())
            .await;

        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (org_id, actor_user_id, action, entity, meta)
            VALUES ($1, $2, 'UPDATE_SETTINGS', 'organizations', $3);
            "#,
        )
        .bind(Some(org_id))
        .bind(auth.user_id)
        .bind(json!({ "new_name": new_name }))
        .execute(&pool)
        .await;

        Redirect::to("/pengaturan").into_response()
    }
}
