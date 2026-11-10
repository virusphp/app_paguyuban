use axum::{
    extract::{Form, Path, Query},
    response::{Html, IntoResponse, Redirect, Response},
};
use chrono::NaiveDate;
use lumina::core::blade::BladeContext;
use lumina::prelude::Request;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::app::services::{AuthContext, FeeService, ViewService};

#[derive(Debug, Deserialize)]
pub struct MemberFilterQuery {
    pub q: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateMemberForm {
    pub full_name: String,
    pub phone: Option<String>,
    pub house_no: String,
    pub house_note: Option<String>,
    pub status: String,
    pub joined_on: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StatusForm {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct ImportCsvForm {
    pub csv_content: String,
}

pub struct MemberController;

impl MemberController {
    /// Daftar Warga (GET /anggota)
    pub async fn index(req: Request, Query(params): Query<MemberFilterQuery>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        let can_edit = auth.require_role(&["admin", "sekretaris"]).is_ok();
        let org_name = auth
            .current_org_name
            .clone()
            .unwrap_or_else(|| "Paguyuban".to_string());

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        // 1. Statistik ringkas
        let total_members: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM members WHERE org_id = $1")
                .bind(org_id)
                .fetch_one(&pool)
                .await
                .unwrap_or(0);

        let total_households: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM households WHERE org_id = $1")
                .bind(org_id)
                .fetch_one(&pool)
                .await
                .unwrap_or(0);

        let active_members: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM members WHERE org_id = $1 AND status = 'aktif'",
        )
        .bind(org_id)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

        let pending_members = total_members - active_members;

        // 2. Query filter daftar warga
        let search_text = params.q.unwrap_or_default().trim().to_string();
        let status_filter = params.status.unwrap_or_default().trim().to_string();

        let search_pattern = format!("%{}%", search_text);

        #[derive(sqlx::FromRow, serde::Serialize)]
        struct MemberRow {
            id: Uuid,
            full_name: String,
            phone: Option<String>,
            status: String,
            joined_on: NaiveDate,
            has_account: bool,
            house_no: Option<String>,
            house_note: Option<String>,
            joined_date: String,
        }

        #[derive(sqlx::FromRow)]
        struct RawMemberRow {
            id: Uuid,
            full_name: String,
            phone: Option<String>,
            status: String,
            joined_on: NaiveDate,
            has_account: bool,
            house_no: Option<String>,
            house_note: Option<String>,
        }

        let raw_rows: Vec<RawMemberRow> = if !search_text.is_empty() && !status_filter.is_empty() {
            sqlx::query_as(
                r#"
                SELECT 
                    m.id,
                    m.full_name,
                    m.phone,
                    m.status,
                    m.joined_on,
                    (m.user_id IS NOT NULL) as has_account,
                    h.house_no,
                    h.note as house_note
                FROM members m
                LEFT JOIN households h ON m.household_id = h.id
                WHERE m.org_id = $1 
                  AND m.status = $2
                  AND (m.full_name ILIKE $3 OR m.phone ILIKE $3 OR h.house_no ILIKE $3)
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
                SELECT 
                    m.id,
                    m.full_name,
                    m.phone,
                    m.status,
                    m.joined_on,
                    (m.user_id IS NOT NULL) as has_account,
                    h.house_no,
                    h.note as house_note
                FROM members m
                LEFT JOIN households h ON m.household_id = h.id
                WHERE m.org_id = $1 
                  AND (m.full_name ILIKE $2 OR m.phone ILIKE $2 OR h.house_no ILIKE $2)
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
                SELECT 
                    m.id,
                    m.full_name,
                    m.phone,
                    m.status,
                    m.joined_on,
                    (m.user_id IS NOT NULL) as has_account,
                    h.house_no,
                    h.note as house_note
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
                SELECT 
                    m.id,
                    m.full_name,
                    m.phone,
                    m.status,
                    m.joined_on,
                    (m.user_id IS NOT NULL) as has_account,
                    h.house_no,
                    h.note as house_note
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

        let members: Vec<MemberRow> = raw_rows
            .into_iter()
            .map(|r| {
                let joined_date = r.joined_on.format("%d %b %Y").to_string();
                MemberRow {
                    id: r.id,
                    full_name: r.full_name,
                    phone: r.phone,
                    status: r.status,
                    joined_on: r.joined_on,
                    has_account: r.has_account,
                    house_no: r.house_no,
                    house_note: r.house_note,
                    joined_date,
                }
            })
            .collect();

        let mut ctx = BladeContext::new();
        ctx.insert("title", format!("Daftar Warga — {}", org_name));
        ctx.insert("area", "admin");
        ctx.insert("active", "anggota");
        ctx.insert("user_name", &auth.user_name);
        ctx.insert("org_name", &org_name);
        ctx.insert("current_org_id", org_id.to_string());
        ctx.insert(
            "role_label",
            auth.current_role.unwrap_or_default().to_uppercase(),
        );
        ctx.insert("can_edit", can_edit);
        ctx.insert("total_members", total_members);
        ctx.insert("total_households", total_households);
        ctx.insert("active_members", active_members);
        ctx.insert("pending_members", pending_members);
        ctx.insert("members", members);
        ctx.insert("search_query", search_text);
        ctx.insert("filter_status", status_filter);

        let html = ViewService::render_blade("members/index.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Form Tambah Warga (GET /anggota/baru)
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

        let org_name = auth
            .current_org_name
            .clone()
            .unwrap_or_else(|| "Paguyuban".to_string());

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(_) => return Redirect::to("/anggota").into_response(),
        };

        #[derive(sqlx::FromRow, serde::Serialize)]
        struct HouseOption {
            house_no: String,
            note: Option<String>,
        }

        let existing_houses: Vec<HouseOption> = sqlx::query_as(
            "SELECT house_no, note FROM households WHERE org_id = $1 ORDER BY house_no ASC",
        )
        .bind(org_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let today = chrono::Local::now().format("%Y-%m-%d").to_string();

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Tambah Warga Baru");
        ctx.insert("area", "admin");
        ctx.insert("active", "anggota");
        ctx.insert("user_name", &auth.user_name);
        ctx.insert("org_name", &org_name);
        ctx.insert("current_org_id", org_id.to_string());
        ctx.insert(
            "role_label",
            auth.current_role.unwrap_or_default().to_uppercase(),
        );
        ctx.insert("is_edit", false);
        ctx.insert("today_date", today);
        ctx.insert("existing_houses", existing_houses);

        let html = ViewService::render_blade("members/form.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Simpan Warga Baru (POST /anggota/baru)
    pub async fn create(req: Request, Form(payload): Form<CreateMemberForm>) -> Response {
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

        let full_name = payload.full_name.trim();
        let house_no = payload.house_no.trim();
        let phone = payload
            .phone
            .as_deref()
            .map(str::trim)
            .filter(|p| !p.is_empty());
        let status = payload.status.trim();
        let house_note = payload
            .house_note
            .as_deref()
            .map(str::trim)
            .filter(|n| !n.is_empty());

        if full_name.is_empty() || house_no.is_empty() {
            return Redirect::to("/anggota/baru").into_response();
        }

        let joined_on = payload
            .joined_on
            .as_deref()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
            .unwrap_or_else(|| chrono::Local::now().date_naive());

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(_) => return Redirect::to("/anggota").into_response(),
        };

        let mut tx = match pool.begin().await {
            Ok(t) => t,
            Err(_) => return Redirect::to("/anggota/baru").into_response(),
        };

        // 1. Cari atau buat household
        let household_id: Uuid = match sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM households WHERE org_id = $1 AND house_no = $2",
        )
        .bind(org_id)
        .bind(house_no)
        .fetch_optional(&mut *tx)
        .await
        {
            Ok(Some(id)) => id,
            _ => {
                let new_house_id = Uuid::new_v4();
                let res = sqlx::query(
                    r#"
                    INSERT INTO households (id, org_id, house_no, note)
                    VALUES ($1, $2, $3, $4);
                    "#,
                )
                .bind(new_house_id)
                .bind(org_id)
                .bind(house_no)
                .bind(house_note)
                .execute(&mut *tx)
                .await;

                if let Err(e) = res {
                    eprintln!("Failed to insert household: {}", e);
                    let _ = tx.rollback().await;
                    return Redirect::to("/anggota/baru").into_response();
                }

                new_house_id
            }
        };

        // 2. Insert member
        let member_id = Uuid::new_v4();
        let insert_res = sqlx::query(
            r#"
            INSERT INTO members (id, org_id, household_id, full_name, phone, status, joined_on)
            VALUES ($1, $2, $3, $4, $5, $6, $7);
            "#,
        )
        .bind(member_id)
        .bind(org_id)
        .bind(household_id)
        .bind(full_name)
        .bind(phone)
        .bind(status)
        .bind(joined_on)
        .execute(&mut *tx)
        .await;

        if let Err(e) = insert_res {
            eprintln!("Failed to insert member: {}", e);
            let _ = tx.rollback().await;
            return Redirect::to("/anggota/baru").into_response();
        }

        // 3. Catat audit_logs
        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (org_id, actor_user_id, action, entity, entity_id, meta)
            VALUES ($1, $2, 'CREATE_MEMBER', 'members', $3, $4);
            "#,
        )
        .bind(Some(org_id))
        .bind(auth.user_id)
        .bind(member_id.to_string())
        .bind(json!({
            "full_name": full_name,
            "house_no": house_no,
            "phone": phone
        }))
        .execute(&mut *tx)
        .await;

        if let Err(e) = tx.commit().await {
            eprintln!("Failed to commit: {}", e);
            return Redirect::to("/anggota/baru").into_response();
        }

        // Sinkronisasi tagihan otomatis jika warga aktif
        if status == "aktif" {
            let _ = FeeService::sync_invoices_for_member(&pool, org_id, member_id, joined_on).await;
        }

        Redirect::to("/anggota").into_response()
    }

    /// Form Ubah Data Warga (GET /anggota/:id)
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

        let org_name = auth
            .current_org_name
            .clone()
            .unwrap_or_else(|| "Paguyuban".to_string());

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(_) => return Redirect::to("/anggota").into_response(),
        };

        #[derive(sqlx::FromRow, serde::Serialize)]
        struct MemberDetail {
            id: Uuid,
            full_name: String,
            phone: Option<String>,
            status: String,
            joined_on: NaiveDate,
            house_no: Option<String>,
            house_note: Option<String>,
        }

        let member: Option<MemberDetail> = sqlx::query_as(
            r#"
            SELECT 
                m.id,
                m.full_name,
                m.phone,
                m.status,
                m.joined_on,
                h.house_no,
                h.note as house_note
            FROM members m
            LEFT JOIN households h ON m.household_id = h.id
            WHERE m.id = $1 AND m.org_id = $2;
            "#,
        )
        .bind(id)
        .bind(org_id)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);

        let member = match member {
            Some(m) => m,
            None => return Redirect::to("/anggota").into_response(),
        };

        #[derive(sqlx::FromRow, serde::Serialize)]
        struct HouseOption {
            house_no: String,
            note: Option<String>,
        }

        let existing_houses: Vec<HouseOption> = sqlx::query_as(
            "SELECT house_no, note FROM households WHERE org_id = $1 ORDER BY house_no ASC",
        )
        .bind(org_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let mut ctx = BladeContext::new();
        ctx.insert("title", format!("Ubah Data — {}", member.full_name));
        ctx.insert("area", "admin");
        ctx.insert("active", "anggota");
        ctx.insert("user_name", &auth.user_name);
        ctx.insert("org_name", &org_name);
        ctx.insert("current_org_id", org_id.to_string());
        ctx.insert(
            "role_label",
            auth.current_role.unwrap_or_default().to_uppercase(),
        );
        ctx.insert("is_edit", true);
        ctx.insert("member", member);
        ctx.insert("existing_houses", existing_houses);

        let html = ViewService::render_blade("members/form.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Simpan Perubahan Data Warga (POST /anggota/:id)
    pub async fn update(
        req: Request,
        Path(id): Path<Uuid>,
        Form(payload): Form<CreateMemberForm>,
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

        let full_name = payload.full_name.trim();
        let house_no = payload.house_no.trim();
        let phone = payload
            .phone
            .as_deref()
            .map(str::trim)
            .filter(|p| !p.is_empty());
        let status = payload.status.trim();
        let house_note = payload
            .house_note
            .as_deref()
            .map(str::trim)
            .filter(|n| !n.is_empty());

        if full_name.is_empty() || house_no.is_empty() {
            return Redirect::to(&format!("/anggota/{}", id)).into_response();
        }

        let joined_on = payload
            .joined_on
            .as_deref()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
            .unwrap_or_else(|| chrono::Local::now().date_naive());

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(_) => return Redirect::to("/anggota").into_response(),
        };

        let mut tx = match pool.begin().await {
            Ok(t) => t,
            Err(_) => return Redirect::to(&format!("/anggota/{}", id)).into_response(),
        };

        // Cari atau buat household
        let household_id: Uuid = match sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM households WHERE org_id = $1 AND house_no = $2",
        )
        .bind(org_id)
        .bind(house_no)
        .fetch_optional(&mut *tx)
        .await
        {
            Ok(Some(hid)) => hid,
            _ => {
                let new_hid = Uuid::new_v4();
                let _ = sqlx::query(
                    "INSERT INTO households (id, org_id, house_no, note) VALUES ($1, $2, $3, $4)",
                )
                .bind(new_hid)
                .bind(org_id)
                .bind(house_no)
                .bind(house_note)
                .execute(&mut *tx)
                .await;
                new_hid
            }
        };

        // Update member
        let update_res = sqlx::query(
            r#"
            UPDATE members 
            SET household_id = $1, full_name = $2, phone = $3, status = $4, joined_on = $5
            WHERE id = $6 AND org_id = $7;
            "#,
        )
        .bind(household_id)
        .bind(full_name)
        .bind(phone)
        .bind(status)
        .bind(joined_on)
        .bind(id)
        .bind(org_id)
        .execute(&mut *tx)
        .await;

        if let Err(e) = update_res {
            eprintln!("Failed to update member: {}", e);
            let _ = tx.rollback().await;
            return Redirect::to(&format!("/anggota/{}", id)).into_response();
        }

        // Catat audit_logs
        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (org_id, actor_user_id, action, entity, entity_id, meta)
            VALUES ($1, $2, 'UPDATE_MEMBER', 'members', $3, $4);
            "#,
        )
        .bind(Some(org_id))
        .bind(auth.user_id)
        .bind(id.to_string())
        .bind(json!({
            "full_name": full_name,
            "house_no": house_no,
            "status": status
        }))
        .execute(&mut *tx)
        .await;

        let _ = tx.commit().await;

        // Sinkronisasi tagihan otomatis jika warga aktif
        if status == "aktif" {
            let _ = FeeService::sync_invoices_for_member(&pool, org_id, id, joined_on).await;
        }

        Redirect::to("/anggota").into_response()
    }

    /// Ubah Cepat Status Warga (POST /anggota/:id/status)
    pub async fn toggle_status(
        req: Request,
        Path(id): Path<Uuid>,
        Form(payload): Form<StatusForm>,
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

        let new_status = payload.status.trim();
        if !["aktif", "menunggu", "nonaktif"].contains(&new_status) {
            return Redirect::to("/anggota").into_response();
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        if let Ok(pool) = sqlx::PgPool::connect(&db_url).await {
            let _ = sqlx::query("UPDATE members SET status = $1 WHERE id = $2 AND org_id = $3")
                .bind(new_status)
                .bind(id)
                .bind(org_id)
                .execute(&pool)
                .await;

            let _ = sqlx::query(
                r#"
                INSERT INTO audit_logs (org_id, actor_user_id, action, entity, entity_id, meta)
                VALUES ($1, $2, 'UPDATE_MEMBER_STATUS', 'members', $3, $4);
                "#,
            )
            .bind(Some(org_id))
            .bind(auth.user_id)
            .bind(id.to_string())
            .bind(json!({ "new_status": new_status }))
            .execute(&pool)
            .await;

            if new_status == "aktif" {
                if let Ok(joined_on) = sqlx::query_scalar::<_, NaiveDate>(
                    "SELECT joined_on FROM members WHERE id = $1 AND org_id = $2",
                )
                .bind(id)
                .bind(org_id)
                .fetch_one(&pool)
                .await
                {
                    let _ =
                        FeeService::sync_invoices_for_member(&pool, org_id, id, joined_on).await;
                }
            }
        }

        Redirect::to("/anggota").into_response()
    }

    /// Halaman Formulir Impor CSV (GET /anggota/impor)
    pub async fn show_import(req: Request) -> Response {
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

        let org_name = auth
            .current_org_name
            .clone()
            .unwrap_or_else(|| "Paguyuban".to_string());

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Impor Data Warga — CSV");
        ctx.insert("area", "admin");
        ctx.insert("active", "anggota");
        ctx.insert("user_name", &auth.user_name);
        ctx.insert("org_name", &org_name);
        ctx.insert("current_org_id", org_id.to_string());
        ctx.insert(
            "role_label",
            auth.current_role.unwrap_or_default().to_uppercase(),
        );

        let html = ViewService::render_blade("members/import.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Proses Impor CSV Massal (POST /anggota/impor)
    pub async fn import_csv(req: Request, Form(payload): Form<ImportCsvForm>) -> Response {
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
            Err(_) => return Redirect::to("/anggota").into_response(),
        };

        let today = chrono::Local::now().date_naive();
        let mut imported_count = 0;

        for line in payload.csv_content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Lewati baris header jika ada
            let lower = line.to_lowercase();
            if lower.starts_with("nomor_rumah")
                || lower.starts_with("house_no")
                || lower.starts_with("no_rumah")
            {
                continue;
            }

            let parts: Vec<&str> = line.split(',').map(str::trim).collect();
            if parts.len() < 2 {
                continue;
            }

            let house_no = parts[0];
            let full_name = parts[1];
            if house_no.is_empty() || full_name.is_empty() {
                continue;
            }

            let phone = if parts.len() > 2 && !parts[2].is_empty() {
                Some(parts[2].to_string())
            } else {
                None
            };

            let status = if parts.len() > 3 && !parts[3].is_empty() {
                parts[3].to_string()
            } else {
                "aktif".to_string()
            };

            // Cari atau buat household
            let household_id: Uuid = match sqlx::query_scalar::<_, Uuid>(
                "SELECT id FROM households WHERE org_id = $1 AND house_no = $2",
            )
            .bind(org_id)
            .bind(house_no)
            .fetch_optional(&pool)
            .await
            {
                Ok(Some(hid)) => hid,
                _ => {
                    let new_hid = Uuid::new_v4();
                    let _ = sqlx::query(
                        "INSERT INTO households (id, org_id, house_no) VALUES ($1, $2, $3)",
                    )
                    .bind(new_hid)
                    .bind(org_id)
                    .bind(house_no)
                    .execute(&pool)
                    .await;
                    new_hid
                }
            };

            // Insert member
            let member_id = Uuid::new_v4();
            let insert_res = sqlx::query(
                r#"
                INSERT INTO members (id, org_id, household_id, full_name, phone, status, joined_on)
                VALUES ($1, $2, $3, $4, $5, $6, $7);
                "#,
            )
            .bind(member_id)
            .bind(org_id)
            .bind(household_id)
            .bind(full_name)
            .bind(phone)
            .bind(status)
            .bind(today)
            .execute(&pool)
            .await;

            if insert_res.is_ok() {
                imported_count += 1;
            }
        }

        // Catat audit_logs
        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (org_id, actor_user_id, action, entity, meta)
            VALUES ($1, $2, 'IMPORT_MEMBERS', 'members', $3);
            "#,
        )
        .bind(Some(org_id))
        .bind(auth.user_id)
        .bind(json!({ "imported_count": imported_count }))
        .execute(&pool)
        .await;

        // Sinkronisasi otomatis tagihan iuran untuk seluruh warga aktif yang baru diimpor
        let _ = FeeService::sync_invoices_for_all_active_members(&pool, org_id).await;

        Redirect::to("/anggota").into_response()
    }
}
