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
    pub nik: Option<String>,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub birth_place: Option<String>,
    pub birth_date: Option<String>,
    pub religion: Option<String>,
    pub occupation: Option<String>,
    pub family_role: Option<String>,
    pub is_head_of_family: Option<String>,
    pub is_fee_payer: Option<String>,
    pub emergency_phone: Option<String>,
    pub notes: Option<String>,
    pub house_no: String,
    pub family_card_no: Option<String>,
    pub ownership_status: Option<String>,
    pub rt_rw: Option<String>,
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

        #[derive(serde::Serialize)]
        struct MemberFormView {
            id: String,
            full_name: String,
            nik: String,
            gender: String,
            phone: String,
            birth_place: String,
            birth_date: String,
            religion: String,
            occupation: String,
            family_role: String,
            is_head_of_family: bool,
            is_fee_payer: bool,
            emergency_phone: String,
            notes: String,
            house_no: String,
            family_card_no: String,
            ownership_status: String,
            rt_rw: String,
            house_note: String,
            status: String,
            joined_on: String,
        }

        let default_member = MemberFormView {
            id: String::new(),
            full_name: String::new(),
            nik: String::new(),
            gender: "Laki-laki".to_string(),
            phone: String::new(),
            birth_place: String::new(),
            birth_date: String::new(),
            religion: "Islam".to_string(),
            occupation: String::new(),
            family_role: "Kepala Keluarga".to_string(),
            is_head_of_family: true,
            is_fee_payer: true,
            emergency_phone: String::new(),
            notes: String::new(),
            house_no: String::new(),
            family_card_no: String::new(),
            ownership_status: "Milik Sendiri".to_string(),
            rt_rw: String::new(),
            house_note: String::new(),
            status: "aktif".to_string(),
            joined_on: today.clone(),
        };

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
        ctx.insert("member", default_member);
        ctx.insert("existing_houses", existing_houses);
        ctx.insert("error", "");

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
        let nik = payload
            .nik
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let gender = payload
            .gender
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("Laki-laki");
        let phone = payload
            .phone
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let birth_place = payload
            .birth_place
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let birth_date = payload
            .birth_date
            .as_deref()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());
        let religion = payload
            .religion
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("Islam");
        let occupation = payload
            .occupation
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let family_role = payload
            .family_role
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("Kepala Keluarga");
        let is_head_of_family = payload
            .is_head_of_family
            .as_deref()
            .map(|v| v == "1" || v == "on" || v == "true")
            .unwrap_or(false)
            || family_role == "Kepala Keluarga";
        let is_fee_payer = payload
            .is_fee_payer
            .as_deref()
            .map(|v| v == "1" || v == "on" || v == "true")
            .unwrap_or(true);
        let emergency_phone = payload
            .emergency_phone
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let notes = payload
            .notes
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());

        let family_card_no = payload
            .family_card_no
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let ownership_status = payload
            .ownership_status
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("Milik Sendiri");
        let rt_rw = payload
            .rt_rw
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let house_note = payload
            .house_note
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let status = payload.status.trim();

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

        // 1. Cari atau buat/update household
        let household_id: Uuid = match sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM households WHERE org_id = $1 AND house_no = $2",
        )
        .bind(org_id)
        .bind(house_no)
        .fetch_optional(&mut *tx)
        .await
        {
            Ok(Some(id)) => {
                let _ = sqlx::query(
                    r#"
                    UPDATE households 
                    SET family_card_no = COALESCE($1, family_card_no),
                        ownership_status = COALESCE($2, ownership_status),
                        rt_rw = COALESCE($3, rt_rw),
                        note = COALESCE($4, note)
                    WHERE id = $5 AND org_id = $6;
                    "#,
                )
                .bind(family_card_no)
                .bind(ownership_status)
                .bind(rt_rw)
                .bind(house_note)
                .bind(id)
                .bind(org_id)
                .execute(&mut *tx)
                .await;
                id
            }
            _ => {
                let new_house_id = Uuid::new_v4();
                let res = sqlx::query(
                    r#"
                    INSERT INTO households (id, org_id, house_no, family_card_no, ownership_status, rt_rw, note)
                    VALUES ($1, $2, $3, $4, $5, $6, $7);
                    "#,
                )
                .bind(new_house_id)
                .bind(org_id)
                .bind(house_no)
                .bind(family_card_no)
                .bind(ownership_status)
                .bind(rt_rw)
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

        // 2. Insert member lengkap
        let member_id = Uuid::new_v4();
        let insert_res = sqlx::query(
            r#"
            INSERT INTO members (
                id, org_id, household_id, full_name, nik, gender, phone,
                birth_place, birth_date, religion, occupation, family_role,
                is_head_of_family, is_fee_payer, emergency_phone, notes,
                status, joined_on
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18);
            "#,
        )
        .bind(member_id)
        .bind(org_id)
        .bind(household_id)
        .bind(full_name)
        .bind(nik)
        .bind(gender)
        .bind(phone)
        .bind(birth_place)
        .bind(birth_date)
        .bind(religion)
        .bind(occupation)
        .bind(family_role)
        .bind(is_head_of_family)
        .bind(is_fee_payer)
        .bind(emergency_phone)
        .bind(notes)
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
            "nik": nik,
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

        #[derive(serde::Serialize)]
        struct MemberFormView {
            id: String,
            full_name: String,
            nik: String,
            gender: String,
            phone: String,
            birth_place: String,
            birth_date: String,
            religion: String,
            occupation: String,
            family_role: String,
            is_head_of_family: bool,
            is_fee_payer: bool,
            emergency_phone: String,
            notes: String,
            house_no: String,
            family_card_no: String,
            ownership_status: String,
            rt_rw: String,
            house_note: String,
            status: String,
            joined_on: String,
        }

        #[derive(sqlx::FromRow)]
        struct MemberDetailRow {
            id: Uuid,
            full_name: String,
            nik: Option<String>,
            gender: Option<String>,
            phone: Option<String>,
            birth_place: Option<String>,
            birth_date: Option<NaiveDate>,
            religion: Option<String>,
            occupation: Option<String>,
            family_role: Option<String>,
            is_head_of_family: Option<bool>,
            is_fee_payer: Option<bool>,
            emergency_phone: Option<String>,
            notes: Option<String>,
            status: String,
            joined_on: NaiveDate,
            house_no: Option<String>,
            family_card_no: Option<String>,
            ownership_status: Option<String>,
            rt_rw: Option<String>,
            house_note: Option<String>,
        }

        let row: Option<MemberDetailRow> = sqlx::query_as(
            r#"
            SELECT 
                m.id,
                m.full_name,
                m.nik,
                m.gender,
                m.phone,
                m.birth_place,
                m.birth_date,
                m.religion,
                m.occupation,
                m.family_role,
                m.is_head_of_family,
                m.is_fee_payer,
                m.emergency_phone,
                m.notes,
                m.status,
                m.joined_on,
                h.house_no,
                h.family_card_no,
                h.ownership_status,
                h.rt_rw,
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

        let row = match row {
            Some(r) => r,
            None => return Redirect::to("/anggota").into_response(),
        };

        let member = MemberFormView {
            id: row.id.to_string(),
            full_name: row.full_name,
            nik: row.nik.unwrap_or_default(),
            gender: row.gender.unwrap_or_else(|| "Laki-laki".to_string()),
            phone: row.phone.unwrap_or_default(),
            birth_place: row.birth_place.unwrap_or_default(),
            birth_date: row
                .birth_date
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_default(),
            religion: row.religion.unwrap_or_else(|| "Islam".to_string()),
            occupation: row.occupation.unwrap_or_default(),
            family_role: row
                .family_role
                .unwrap_or_else(|| "Kepala Keluarga".to_string()),
            is_head_of_family: row.is_head_of_family.unwrap_or(false),
            is_fee_payer: row.is_fee_payer.unwrap_or(true),
            emergency_phone: row.emergency_phone.unwrap_or_default(),
            notes: row.notes.unwrap_or_default(),
            status: row.status,
            joined_on: row.joined_on.format("%Y-%m-%d").to_string(),
            house_no: row.house_no.unwrap_or_default(),
            family_card_no: row.family_card_no.unwrap_or_default(),
            ownership_status: row
                .ownership_status
                .unwrap_or_else(|| "Milik Sendiri".to_string()),
            rt_rw: row.rt_rw.unwrap_or_default(),
            house_note: row.house_note.unwrap_or_default(),
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
        ctx.insert("today_date", row.joined_on.format("%Y-%m-%d").to_string());
        ctx.insert("error", "");

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
        let nik = payload
            .nik
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let gender = payload
            .gender
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("Laki-laki");
        let phone = payload
            .phone
            .as_deref()
            .map(str::trim)
            .filter(|p| !p.is_empty());
        let birth_place = payload
            .birth_place
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let birth_date = payload
            .birth_date
            .as_deref()
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());
        let religion = payload
            .religion
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("Islam");
        let occupation = payload
            .occupation
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let family_role = payload
            .family_role
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("Kepala Keluarga");
        let is_head_of_family = payload
            .is_head_of_family
            .as_deref()
            .map(|v| v == "1" || v == "on" || v == "true")
            .unwrap_or(false)
            || family_role == "Kepala Keluarga";
        let is_fee_payer = payload
            .is_fee_payer
            .as_deref()
            .map(|v| v == "1" || v == "on" || v == "true")
            .unwrap_or(true);
        let emergency_phone = payload
            .emergency_phone
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let notes = payload
            .notes
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());

        let family_card_no = payload
            .family_card_no
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let ownership_status = payload
            .ownership_status
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("Milik Sendiri");
        let rt_rw = payload
            .rt_rw
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let house_note = payload
            .house_note
            .as_deref()
            .map(str::trim)
            .filter(|n| !n.is_empty());
        let status = payload.status.trim();

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

        // Cari atau buat/update household
        let household_id: Uuid = match sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM households WHERE org_id = $1 AND house_no = $2",
        )
        .bind(org_id)
        .bind(house_no)
        .fetch_optional(&mut *tx)
        .await
        {
            Ok(Some(hid)) => {
                let _ = sqlx::query(
                    r#"
                    UPDATE households 
                    SET family_card_no = COALESCE($1, family_card_no),
                        ownership_status = COALESCE($2, ownership_status),
                        rt_rw = COALESCE($3, rt_rw),
                        note = COALESCE($4, note)
                    WHERE id = $5 AND org_id = $6;
                    "#,
                )
                .bind(family_card_no)
                .bind(ownership_status)
                .bind(rt_rw)
                .bind(house_note)
                .bind(hid)
                .bind(org_id)
                .execute(&mut *tx)
                .await;
                hid
            }
            _ => {
                let new_hid = Uuid::new_v4();
                let _ = sqlx::query(
                    r#"
                    INSERT INTO households (id, org_id, house_no, family_card_no, ownership_status, rt_rw, note)
                    VALUES ($1, $2, $3, $4, $5, $6, $7);
                    "#,
                )
                .bind(new_hid)
                .bind(org_id)
                .bind(house_no)
                .bind(family_card_no)
                .bind(ownership_status)
                .bind(rt_rw)
                .bind(house_note)
                .execute(&mut *tx)
                .await;
                new_hid
            }
        };

        // Update member lengkap
        let update_res = sqlx::query(
            r#"
            UPDATE members 
            SET household_id = $1, full_name = $2, nik = $3, gender = $4, phone = $5,
                birth_place = $6, birth_date = $7, religion = $8, occupation = $9,
                family_role = $10, is_head_of_family = $11, is_fee_payer = $12,
                emergency_phone = $13, notes = $14, status = $15, joined_on = $16
            WHERE id = $17 AND org_id = $18;
            "#,
        )
        .bind(household_id)
        .bind(full_name)
        .bind(nik)
        .bind(gender)
        .bind(phone)
        .bind(birth_place)
        .bind(birth_date)
        .bind(religion)
        .bind(occupation)
        .bind(family_role)
        .bind(is_head_of_family)
        .bind(is_fee_payer)
        .bind(emergency_phone)
        .bind(notes)
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
