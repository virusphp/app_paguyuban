use axum::{
    extract::{Form, Path},
    response::{Html, IntoResponse, Redirect, Response},
};
use chrono::{FixedOffset, Utc};
use lumina::core::blade::BladeContext;
use lumina::prelude::Request;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::app::services::{AuthContext, ViewService};

#[derive(Debug, Deserialize)]
pub struct CreateOrgForm {
    pub org_name: String,
    pub org_type: String,
    pub admin_name: String,
    pub admin_phone: String,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateStatusForm {
    pub status: String,
}

pub struct SuperadminController;

impl SuperadminController {
    /// Dashboard Ringkasan Global & Developer Telemetry (GET /sa & GET /sa/dashboard)
    pub async fn dashboard(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_superadmin() {
            return res;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => {
                return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response();
            }
        };

        // 1. Metrik Global
        let total_orgs: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM organizations")
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

        let active_orgs: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM organizations WHERE status = 'aktif'")
                .fetch_one(&pool)
                .await
                .unwrap_or(0);

        let total_members: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM members WHERE status = 'aktif'")
                .fetch_one(&pool)
                .await
                .unwrap_or(0);

        let total_households: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM households")
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

        let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

        // Akumulasi Kas Global (Pemasukan - Pengeluaran)
        let total_income: i64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(amount_idr), 0) FROM transactions WHERE type = 'pemasukan'",
        )
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

        let total_expense: i64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(amount_idr), 0) FROM transactions WHERE type = 'pengeluaran'",
        )
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

        let global_cash_balance = total_income - total_expense;

        // Akumulasi Tagihan & Kepatuhan Iuran
        let total_invoices_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM invoices")
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

        let paid_invoices_count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM invoices WHERE status = 'lunas'")
                .fetch_one(&pool)
                .await
                .unwrap_or(0);

        let global_compliance_percent = if total_invoices_count > 0 {
            (paid_invoices_count * 100) / total_invoices_count
        } else {
            100
        };

        // 2. Daftar Organisasi Teratas (maks 6)
        #[derive(sqlx::FromRow, serde::Serialize)]
        struct TopOrg {
            id: Uuid,
            name: String,
            org_type: String,
            status: String,
            members_count: i64,
        }

        let top_orgs: Vec<TopOrg> = sqlx::query_as(
            r#"
            SELECT o.id, o.name, o.type as org_type, o.status,
                   COALESCE((SELECT COUNT(*) FROM members m WHERE m.org_id = o.id AND m.status = 'aktif'), 0) as members_count
            FROM organizations o
            ORDER BY o.created_at DESC
            LIMIT 6;
            "#,
        )
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        // 3. 5 Log Audit Terakhir
        #[derive(sqlx::FromRow, serde::Serialize)]
        struct RecentLog {
            created_at: String,
            actor_name: String,
            action: String,
            org_name: Option<String>,
        }

        let recent_logs_raw: Vec<(chrono::DateTime<Utc>, String, String, Option<String>)> =
            sqlx::query_as(
                r#"
            SELECT a.created_at, u.name as actor_name, a.action, o.name as org_name
            FROM audit_logs a
            JOIN users u ON a.actor_user_id = u.id
            LEFT JOIN organizations o ON a.org_id = o.id
            ORDER BY a.created_at DESC
            LIMIT 5;
            "#,
            )
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

        let wib_offset = FixedOffset::east_opt(7 * 3600).unwrap();
        let recent_logs: Vec<RecentLog> = recent_logs_raw
            .into_iter()
            .map(|(created_at, actor_name, action, org_name)| RecentLog {
                created_at: created_at
                    .with_timezone(&wib_offset)
                    .format("%d %b %H:%M")
                    .to_string(),
                actor_name,
                action,
                org_name,
            })
            .collect();

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Portal Superadmin & Developer — Sistem Paguyuban");
        ctx.insert("area", "sa");
        ctx.insert("active", "dashboard");
        ctx.insert("user_name", &auth.user_name);
        ctx.insert("role_label", "SUPERADMIN");
        ctx.insert("org_name", "Pusat Kontrol Superadmin");

        ctx.insert("total_orgs", total_orgs);
        ctx.insert("active_orgs", active_orgs);
        ctx.insert("total_members", total_members);
        ctx.insert("total_households", total_households);
        ctx.insert("total_users", total_users);
        ctx.insert("global_cash_balance", global_cash_balance);
        ctx.insert("total_invoices_count", total_invoices_count);
        ctx.insert("paid_invoices_count", paid_invoices_count);
        ctx.insert("global_compliance_percent", global_compliance_percent);
        ctx.insert("top_orgs", top_orgs);
        ctx.insert("recent_logs", recent_logs);

        let html = ViewService::render_blade("superadmin/dashboard.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Daftar Organisasi (GET /sa/organisasi)
    pub async fn list_orgs(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_superadmin() {
            return res;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => {
                return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response();
            }
        };

        // Hitung statistik
        let total_orgs: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM organizations")
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

        let active_orgs: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM organizations WHERE status = 'aktif'")
                .fetch_one(&pool)
                .await
                .unwrap_or(0);

        let pending_orgs = total_orgs - active_orgs;

        // Ambil daftar organisasi beserta admin utamanya
        #[derive(sqlx::FromRow, serde::Serialize)]
        struct OrgRow {
            id: Uuid,
            name: String,
            org_type: String,
            status: String,
            admin_name: Option<String>,
            admin_phone: Option<String>,
        }

        let orgs: Vec<OrgRow> = sqlx::query_as(
            r#"
            SELECT 
                o.id,
                o.name,
                o.type::text as org_type,
                o.status::text as status,
                u.name as admin_name,
                u.phone as admin_phone
            FROM organizations o
            LEFT JOIN memberships m ON o.id = m.org_id AND m.role = 'admin'
            LEFT JOIN users u ON m.user_id = u.id
            ORDER BY o.created_at DESC;
            "#,
        )
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Superadmin — Manajemen Organisasi");
        ctx.insert("area", "sa");
        ctx.insert("active", "organisasi");
        ctx.insert("user_name", &auth.user_name);
        ctx.insert("role_label", "SUPERADMIN");
        ctx.insert("total_orgs", total_orgs);
        ctx.insert("active_orgs", active_orgs);
        ctx.insert("pending_orgs", pending_orgs);
        ctx.insert("organizations", orgs);

        let html = ViewService::render_blade("superadmin/organizations.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Form Tambah Organisasi (GET /sa/organisasi/baru)
    pub async fn show_create_org(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_superadmin() {
            return res;
        }

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Tambah Organisasi — Superadmin");
        ctx.insert("area", "sa");
        ctx.insert("active", "organisasi");
        ctx.insert("user_name", &auth.user_name);
        ctx.insert("role_label", "SUPERADMIN");

        let html = ViewService::render_blade("superadmin/org_create.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Proses Simpan Organisasi Baru (POST /sa/organisasi/baru)
    pub async fn create_org(req: Request, Form(payload): Form<CreateOrgForm>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_superadmin() {
            return res;
        }

        let org_name = payload.org_name.trim();
        let org_type = payload.org_type.trim();
        let admin_name = payload.admin_name.trim();
        let admin_phone = payload.admin_phone.trim();

        if org_name.is_empty() || admin_name.is_empty() || admin_phone.is_empty() {
            return Redirect::to("/sa/organisasi/baru").into_response();
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(_) => return Redirect::to("/sa/organisasi").into_response(),
        };

        let raw_password = payload
            .password
            .unwrap_or_else(|| "password123".to_string());
        let password_hash = match bcrypt::hash(&raw_password, bcrypt::DEFAULT_COST) {
            Ok(h) => h,
            Err(_) => return Redirect::to("/sa/organisasi/baru").into_response(),
        };

        // Simpan dalam transaksi
        let mut tx = match pool.begin().await {
            Ok(t) => t,
            Err(_) => return Redirect::to("/sa/organisasi/baru").into_response(),
        };

        let new_org_id = Uuid::new_v4();

        // 1. Insert organisasi
        let insert_org_res = sqlx::query(
            r#"
            INSERT INTO organizations (id, name, type, status)
            VALUES ($1, $2, $3::org_type, 'aktif');
            "#,
        )
        .bind(new_org_id)
        .bind(org_name)
        .bind(org_type)
        .execute(&mut *tx)
        .await;

        if let Err(e) = insert_org_res {
            eprintln!("Failed to insert organization: {}", e);
            let _ = tx.rollback().await;
            return Redirect::to("/sa/organisasi/baru").into_response();
        }

        // 2. Insert atau ambil user admin pertama
        let user_id: Uuid =
            match sqlx::query_scalar::<_, Uuid>("SELECT id FROM users WHERE phone = $1")
                .bind(admin_phone)
                .fetch_optional(&mut *tx)
                .await
            {
                Ok(Some(existing_id)) => existing_id,
                Ok(None) => {
                    let new_user_id = Uuid::new_v4();
                    let insert_user = sqlx::query(
                        r#"
                    INSERT INTO users (id, name, phone, password_hash, is_superadmin, status)
                    VALUES ($1, $2, $3, $4, false, 'aktif');
                    "#,
                    )
                    .bind(new_user_id)
                    .bind(admin_name)
                    .bind(admin_phone)
                    .bind(password_hash)
                    .execute(&mut *tx)
                    .await;

                    if let Err(e) = insert_user {
                        eprintln!("Failed to insert admin user: {}", e);
                        let _ = tx.rollback().await;
                        return Redirect::to("/sa/organisasi/baru").into_response();
                    }

                    new_user_id
                }
                Err(_) => {
                    let _ = tx.rollback().await;
                    return Redirect::to("/sa/organisasi/baru").into_response();
                }
            };

        // 3. Insert keanggotaan (memberships) peran admin
        let insert_member = sqlx::query(
            r#"
            INSERT INTO memberships (org_id, user_id, role)
            VALUES ($1, $2, 'admin'::org_role)
            ON CONFLICT (org_id, user_id) DO UPDATE SET role = 'admin'::org_role;
            "#,
        )
        .bind(new_org_id)
        .bind(user_id)
        .execute(&mut *tx)
        .await;

        if let Err(e) = insert_member {
            eprintln!("Failed to insert membership: {}", e);
            let _ = tx.rollback().await;
            return Redirect::to("/sa/organisasi/baru").into_response();
        }

        // 4. Catat audit_logs
        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (org_id, actor_user_id, action, entity, entity_id, meta)
            VALUES ($1, $2, 'CREATE_ORG', 'organizations', $1, $3);
            "#,
        )
        .bind(Some(new_org_id))
        .bind(auth.user_id)
        .bind(json!({
            "org_name": org_name,
            "org_type": org_type,
            "admin_name": admin_name,
            "admin_phone": admin_phone
        }))
        .execute(&mut *tx)
        .await;

        if let Err(e) = tx.commit().await {
            eprintln!("Failed to commit transaction: {}", e);
            return Redirect::to("/sa/organisasi/baru").into_response();
        }

        Redirect::to("/sa/organisasi").into_response()
    }

    /// Ubah Status Organisasi (POST /sa/organisasi/:id/status)
    pub async fn update_status(
        req: Request,
        Path(id): Path<Uuid>,
        Form(payload): Form<UpdateStatusForm>,
    ) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_superadmin() {
            return res;
        }

        let new_status = payload.status.trim();
        if !["aktif", "menunggu", "nonaktif"].contains(&new_status) {
            return Redirect::to("/sa/organisasi").into_response();
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        if let Ok(pool) = sqlx::PgPool::connect(&db_url).await {
            let _ = sqlx::query("UPDATE organizations SET status = $1::org_status WHERE id = $2;")
                .bind(new_status)
                .bind(id)
                .execute(&pool)
                .await;

            let _ = sqlx::query(
                r#"
                INSERT INTO audit_logs (org_id, actor_user_id, action, entity, entity_id, meta)
                VALUES ($1, $2, 'UPDATE_STATUS', 'organizations', $1, $3);
                "#,
            )
            .bind(Some(id))
            .bind(auth.user_id)
            .bind(json!({ "new_status": new_status }))
            .execute(&pool)
            .await;
        }

        Redirect::to("/sa/organisasi").into_response()
    }

    /// Masuk Sebagai Admin (Impersonasi) (POST /sa/impersonasi/:id)
    pub async fn impersonate(req: Request, Path(id): Path<Uuid>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_superadmin() {
            return res;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(_) => return Redirect::to("/sa/organisasi").into_response(),
        };

        // Ambil nama organisasi
        let org_name: Option<String> =
            sqlx::query_scalar("SELECT name FROM organizations WHERE id = $1")
                .bind(id)
                .fetch_optional(&pool)
                .await
                .unwrap_or(None);

        if let Some(name) = org_name {
            // Set sesi aktif organisasi
            let _ = req.session.insert("current_org_id", id.to_string()).await;
            let _ = req.session.insert("current_org_name", name.clone()).await;
            let _ = req
                .session
                .insert("current_role", "admin".to_string())
                .await;

            // Catat log impersonasi wajib
            let _ = sqlx::query(
                r#"
                INSERT INTO audit_logs (org_id, actor_user_id, action, entity, entity_id, meta)
                VALUES ($1, $2, 'IMPERSONATE_ORG', 'organizations', $1, $3);
                "#,
            )
            .bind(Some(id))
            .bind(auth.user_id)
            .bind(json!({ "target_org_name": name, "actor": auth.user_name }))
            .execute(&pool)
            .await;

            return Redirect::to("/dashboard").into_response();
        }

        Redirect::to("/sa/organisasi").into_response()
    }

    /// Tampilan Log Aktivitas Global (GET /sa/log)
    pub async fn list_audit_logs(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_superadmin() {
            return res;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Database error: {}</h1>", e)).into_response(),
        };

        #[derive(sqlx::FromRow)]
        struct RawLog {
            created_at: chrono::DateTime<Utc>,
            actor_name: Option<String>,
            actor_phone: Option<String>,
            org_name: Option<String>,
            action: String,
            entity: String,
            meta: Option<serde_json::Value>,
        }

        let raw_logs: Vec<RawLog> = sqlx::query_as(
            r#"
            SELECT 
                a.at as created_at,
                u.name as actor_name,
                u.phone as actor_phone,
                o.name as org_name,
                a.action,
                a.entity,
                a.meta
            FROM audit_logs a
            LEFT JOIN users u ON a.actor_user_id = u.id
            LEFT JOIN organizations o ON a.org_id = o.id
            ORDER BY a.at DESC
            LIMIT 50;
            "#,
        )
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        #[derive(serde::Serialize)]
        struct ViewLog {
            created_at: String,
            actor_name: String,
            actor_phone: String,
            org_name: Option<String>,
            action: String,
            entity: String,
            meta_text: String,
        }

        let wib_offset = FixedOffset::east_opt(7 * 3600).unwrap();
        let logs: Vec<ViewLog> = raw_logs
            .into_iter()
            .map(|r| {
                let wib_time = r.created_at.with_timezone(&wib_offset);
                let time_str = wib_time.format("%d %b %Y, %H:%M WIB").to_string();
                let meta_str = r
                    .meta
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "-".to_string());

                ViewLog {
                    created_at: time_str,
                    actor_name: r.actor_name.unwrap_or_else(|| "Sistem".to_string()),
                    actor_phone: r.actor_phone.unwrap_or_else(|| "-".to_string()),
                    org_name: r.org_name,
                    action: r.action,
                    entity: r.entity,
                    meta_text: meta_str,
                }
            })
            .collect();

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Log Aktivitas Sistem — Superadmin");
        ctx.insert("area", "sa");
        ctx.insert("active", "log");
        ctx.insert("user_name", &auth.user_name);
        ctx.insert("role_label", "SUPERADMIN");
        ctx.insert("logs", logs);

        let html = ViewService::render_blade("superadmin/audit_logs.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Daftar Seluruh Pengguna Sistem (GET /sa/pengguna)
    pub async fn list_users(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_superadmin() {
            return res;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Database error: {}</h1>", e)).into_response(),
        };

        let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

        let total_superadmins: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE is_superadmin = true")
                .fetch_one(&pool)
                .await
                .unwrap_or(0);

        #[derive(sqlx::FromRow)]
        struct RawUserItem {
            name: String,
            phone: String,
            is_superadmin: bool,
            status: String,
            last_login_at: Option<chrono::DateTime<Utc>>,
            affiliations: Option<String>,
        }

        let raw_users: Vec<RawUserItem> = sqlx::query_as(
            r#"
            SELECT 
                u.name,
                u.phone,
                u.is_superadmin,
                u.status,
                u.last_login_at,
                COALESCE(
                    string_agg(DISTINCT o.name || ' (' || m.role::text || ')', ', '),
                    '-'
                ) as affiliations
            FROM users u
            LEFT JOIN memberships m ON u.id = m.user_id
            LEFT JOIN organizations o ON m.org_id = o.id
            GROUP BY u.id, u.name, u.phone, u.is_superadmin, u.status, u.last_login_at, u.created_at
            ORDER BY u.created_at DESC;
            "#,
        )
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        #[derive(serde::Serialize)]
        struct ViewUserItem {
            name: String,
            phone: String,
            role_badge: String,
            status: String,
            affiliations: String,
            last_login: String,
        }

        let wib_offset = FixedOffset::east_opt(7 * 3600).unwrap();
        let users: Vec<ViewUserItem> = raw_users
            .into_iter()
            .map(|u| {
                let last_login = u
                    .last_login_at
                    .map(|dt| {
                        dt.with_timezone(&wib_offset)
                            .format("%d %b %Y, %H:%M")
                            .to_string()
                    })
                    .unwrap_or_else(|| "Belum pernah".to_string());

                let role_badge = if u.is_superadmin {
                    "SUPERADMIN".to_string()
                } else {
                    "PENGGUNA".to_string()
                };

                ViewUserItem {
                    name: u.name,
                    phone: u.phone,
                    role_badge,
                    status: u.status,
                    affiliations: u.affiliations.unwrap_or_else(|| "-".to_string()),
                    last_login,
                }
            })
            .collect();

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Manajemen Pengguna — Superadmin");
        ctx.insert("area", "sa");
        ctx.insert("active", "pengguna");
        ctx.insert("user_name", &auth.user_name);
        ctx.insert("role_label", "SUPERADMIN");
        ctx.insert("total_users", total_users);
        ctx.insert("total_superadmins", total_superadmins);
        ctx.insert("users", users);

        let html = ViewService::render_blade("superadmin/users.blade.rs", &ctx);
        Html(html).into_response()
    }
}
