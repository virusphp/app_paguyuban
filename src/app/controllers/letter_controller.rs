use axum::{
    extract::{Form, Path, Query},
    http::header,
    response::{Html, IntoResponse, Redirect, Response},
};
use chrono::{DateTime, Datelike, Local, Utc};
use lumina::core::blade::BladeContext;
use lumina::prelude::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::services::{AuthContext, LetterPdfService, ViewService};

#[derive(Debug, Deserialize)]
pub struct FilterLetterQuery {
    pub kind: Option<String>,
    pub q: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLetterForm {
    pub kind: String,
    pub number: String,
    pub recipient: String,
    pub subject: String,
    pub body: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTemplateForm {
    pub name: String,
    pub body: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct LetterItemView {
    pub id: Uuid,
    pub kind: String,
    pub number: String,
    pub subject: String,
    pub recipient: String,
    pub status: String,
    pub date_formatted: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct CustomTemplateView {
    pub id: Uuid,
    pub name: String,
    pub body: String,
    pub created_at_formatted: String,
}

pub struct LetterController;

impl LetterController {
    /// Helper penomoran urut otomatis surat paguyuban
    pub fn generate_auto_number(seq_num: i64, org_name: &str, date: DateTime<Local>) -> String {
        let year = date.year();
        let month_roman = match date.month() {
            1 => "I",
            2 => "II",
            3 => "III",
            4 => "IV",
            5 => "V",
            6 => "VI",
            7 => "VII",
            8 => "VIII",
            9 => "IX",
            10 => "X",
            11 => "XI",
            _ => "XII",
        };

        let org_code = if org_name.to_lowercase().contains("kanfer") {
            "PGY-KNF2"
        } else {
            "PGY-RT"
        };

        format!("{:03}/{}/{}/{}", seq_num, org_code, month_roman, year)
    }

    /// Daftar Surat & Undangan (GET /surat)
    pub async fn index(req: Request, Query(query): Query<FilterLetterQuery>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        let can_manage = auth.require_role(&["admin", "sekretaris"]).is_ok();
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

        #[derive(sqlx::FromRow)]
        struct RawLetter {
            id: Uuid,
            kind: String,
            number: String,
            subject: String,
            recipient: String,
            status: String,
            created_at: DateTime<Utc>,
        }

        let raw_letters: Vec<RawLetter> = sqlx::query_as(
            r#"
            SELECT id, kind, number, subject, recipient, status, created_at
            FROM letters
            WHERE org_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(org_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let total_count = raw_letters.len();
        let surat_count = raw_letters.iter().filter(|l| l.kind == "surat").count();
        let undangan_count = raw_letters.iter().filter(|l| l.kind == "undangan").count();
        let sent_count = raw_letters
            .iter()
            .filter(|l| l.status == "terkirim")
            .count();

        let filter_kind = query.kind.unwrap_or_default();
        let search_query = query.q.unwrap_or_default();

        let filtered_letters: Vec<LetterItemView> = raw_letters
            .into_iter()
            .filter(|l| {
                if !filter_kind.is_empty() && l.kind != filter_kind {
                    return false;
                }
                if !search_query.is_empty() {
                    let q = search_query.to_lowercase();
                    let match_sub = l.subject.to_lowercase().contains(&q);
                    let match_num = l.number.to_lowercase().contains(&q);
                    let match_rec = l.recipient.to_lowercase().contains(&q);
                    return match_sub || match_num || match_rec;
                }
                true
            })
            .map(|l| {
                let local_time: DateTime<Local> = DateTime::from(l.created_at);
                LetterItemView {
                    id: l.id,
                    kind: l.kind,
                    number: l.number,
                    subject: l.subject,
                    recipient: l.recipient,
                    status: l.status,
                    date_formatted: local_time.format("%d %b %Y").to_string(),
                }
            })
            .collect();

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Surat & Undangan — Paguyuban");
        ctx.insert("area", "admin");
        ctx.insert("active", "surat");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("can_manage", can_manage);
        ctx.insert("total_count", total_count);
        ctx.insert("surat_count", surat_count);
        ctx.insert("undangan_count", undangan_count);
        ctx.insert("sent_count", sent_count);
        ctx.insert("filter_kind", &filter_kind);
        ctx.insert("search_query", &search_query);
        ctx.insert("letters", filtered_letters);

        let html = ViewService::render_blade("letters/index.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Form Buat Surat Baru (GET /surat/baru)
    pub async fn show_create(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "sekretaris"]) {
            return res;
        }

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

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

        let now = Local::now();
        let current_year = now.year();

        // Hitung jumlah surat tahun ini untuk auto-number
        let count_row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM letters
            WHERE org_id = $1 AND EXTRACT(YEAR FROM created_at) = $2
            "#,
        )
        .bind(org_id)
        .bind(current_year as f64)
        .fetch_one(&pool)
        .await
        .unwrap_or((0,));

        let auto_number = Self::generate_auto_number(count_row.0 + 1, &org_name, now);

        #[derive(sqlx::FromRow)]
        struct RawTmpl {
            id: Uuid,
            name: String,
            body: String,
            created_at: DateTime<Utc>,
        }

        let raw_templates: Vec<RawTmpl> = sqlx::query_as(
            "SELECT id, name, body, created_at FROM letter_templates WHERE org_id = $1 ORDER BY name ASC",
        )
        .bind(org_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let custom_templates: Vec<CustomTemplateView> = raw_templates
            .into_iter()
            .map(|t| {
                let local_time: DateTime<Local> = DateTime::from(t.created_at);
                CustomTemplateView {
                    id: t.id,
                    name: t.name,
                    body: t.body,
                    created_at_formatted: local_time.format("%d/%m/%Y").to_string(),
                }
            })
            .collect();

        #[derive(Serialize)]
        struct EmptyLetter {
            kind: String,
            number: String,
            recipient: String,
            subject: String,
            body: String,
            status: String,
        }

        let empty_letter = EmptyLetter {
            kind: "surat".to_string(),
            number: auto_number.clone(),
            recipient: String::new(),
            subject: String::new(),
            body: String::new(),
            status: "draft".to_string(),
        };

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Buat Surat Baru — Paguyuban");
        ctx.insert("area", "admin");
        ctx.insert("active", "surat");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("is_edit", false);
        ctx.insert("auto_number", &auto_number);
        ctx.insert("custom_templates", custom_templates);
        ctx.insert("letter", empty_letter);

        let html = ViewService::render_blade("letters/form.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Simpan Surat Baru (POST /surat/baru)
    pub async fn create(req: Request, Form(form): Form<CreateLetterForm>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "sekretaris"]) {
            return res;
        }

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        if form.number.trim().is_empty()
            || form.subject.trim().is_empty()
            || form.body.trim().is_empty()
            || form.recipient.trim().is_empty()
        {
            return Redirect::to("/surat/baru").into_response();
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let letter_id = Uuid::new_v4();
        let sent_at = if form.status == "terkirim" {
            Some(Utc::now())
        } else {
            None
        };

        let _ = sqlx::query(
            r#"
            INSERT INTO letters (id, org_id, kind, number, subject, body, recipient, status, created_by, sent_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
        )
        .bind(letter_id)
        .bind(org_id)
        .bind(form.kind.trim())
        .bind(form.number.trim())
        .bind(form.subject.trim())
        .bind(form.body.trim())
        .bind(form.recipient.trim())
        .bind(form.status.trim())
        .bind(auth.user_id)
        .bind(sent_at)
        .execute(&pool)
        .await;

        Redirect::to(&format!("/surat/{}", letter_id)).into_response()
    }

    /// Pratinjau Lembar Surat (GET /surat/:id)
    pub async fn show_detail(req: Request, Path(letter_id): Path<Uuid>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        let can_manage = auth.require_role(&["admin", "sekretaris"]).is_ok();
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

        #[derive(sqlx::FromRow)]
        struct RawDetail {
            id: Uuid,
            kind: String,
            number: String,
            subject: String,
            body: String,
            recipient: String,
            status: String,
            created_at: DateTime<Utc>,
        }

        let raw: RawDetail = match sqlx::query_as(
            r#"
            SELECT id, kind, number, subject, body, recipient, status, created_at
            FROM letters
            WHERE id = $1 AND org_id = $2
            "#,
        )
        .bind(letter_id)
        .bind(org_id)
        .fetch_one(&pool)
        .await
        {
            Ok(l) => l,
            Err(_) => return Html("<h1>Surat Tidak Ditemukan</h1>".to_string()).into_response(),
        };

        let local_time: DateTime<Local> = DateTime::from(raw.created_at);

        #[derive(Serialize)]
        struct LetterDetailView {
            id: Uuid,
            kind: String,
            number: String,
            subject: String,
            body: String,
            recipient: String,
            status: String,
            date_formatted: String,
        }

        let letter_view = LetterDetailView {
            id: raw.id,
            kind: raw.kind,
            number: raw.number,
            subject: raw.subject,
            body: raw.body,
            recipient: raw.recipient,
            status: raw.status,
            date_formatted: local_time.format("%d %B %Y").to_string(),
        };

        let mut ctx = BladeContext::new();
        ctx.insert("title", format!("Surat {} — Paguyuban", letter_view.number));
        ctx.insert("area", "admin");
        ctx.insert("active", "surat");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("can_manage", can_manage);
        ctx.insert("letter", letter_view);

        let html = ViewService::render_blade("letters/detail.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Form Ubah Surat (GET /surat/:id/edit)
    pub async fn show_edit(req: Request, Path(letter_id): Path<Uuid>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "sekretaris"]) {
            return res;
        }

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

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

        #[derive(sqlx::FromRow, Serialize)]
        struct EditRow {
            id: Uuid,
            kind: String,
            number: String,
            subject: String,
            body: String,
            recipient: String,
            status: String,
        }

        let letter: EditRow = match sqlx::query_as(
            "SELECT id, kind, number, subject, body, recipient, status FROM letters WHERE id = $1 AND org_id = $2",
        )
        .bind(letter_id)
        .bind(org_id)
        .fetch_one(&pool)
        .await
        {
            Ok(l) => l,
            Err(_) => return Html("<h1>Surat Tidak Ditemukan</h1>".to_string()).into_response(),
        };

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Ubah Surat — Paguyuban");
        ctx.insert("area", "admin");
        ctx.insert("active", "surat");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("is_edit", true);
        ctx.insert("auto_number", "");
        ctx.insert("custom_templates", Vec::<CustomTemplateView>::new());
        ctx.insert("letter", letter);

        let html = ViewService::render_blade("letters/form.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Simpan Perubahan Surat (POST /surat/:id)
    pub async fn update(
        req: Request,
        Path(letter_id): Path<Uuid>,
        Form(form): Form<CreateLetterForm>,
    ) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "sekretaris"]) {
            return res;
        }

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let sent_at = if form.status == "terkirim" {
            Some(Utc::now())
        } else {
            None
        };

        let _ = sqlx::query(
            r#"
            UPDATE letters
            SET kind = $1, number = $2, subject = $3, body = $4, recipient = $5, status = $6, sent_at = COALESCE(sent_at, $7)
            WHERE id = $8 AND org_id = $9
            "#,
        )
        .bind(form.kind.trim())
        .bind(form.number.trim())
        .bind(form.subject.trim())
        .bind(form.body.trim())
        .bind(form.recipient.trim())
        .bind(form.status.trim())
        .bind(sent_at)
        .bind(letter_id)
        .bind(org_id)
        .execute(&pool)
        .await;

        Redirect::to(&format!("/surat/{}", letter_id)).into_response()
    }

    /// Toggle Status Kirim Surat (POST /surat/:id/kirim)
    pub async fn toggle_sent(req: Request, Path(letter_id): Path<Uuid>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "sekretaris"]) {
            return res;
        }

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let current_status: Option<String> =
            sqlx::query_scalar("SELECT status FROM letters WHERE id = $1 AND org_id = $2")
                .bind(letter_id)
                .bind(org_id)
                .fetch_optional(&pool)
                .await
                .unwrap_or(None);

        if let Some(st) = current_status {
            let (new_st, sent_at) = if st == "terkirim" {
                ("draft", None)
            } else {
                ("terkirim", Some(Utc::now()))
            };

            let _ = sqlx::query(
                "UPDATE letters SET status = $1, sent_at = $2 WHERE id = $3 AND org_id = $4",
            )
            .bind(new_st)
            .bind(sent_at)
            .bind(letter_id)
            .bind(org_id)
            .execute(&pool)
            .await;
        }

        Redirect::to(&format!("/surat/{}", letter_id)).into_response()
    }

    /// Hapus Surat (POST /surat/:id/hapus)
    pub async fn delete(req: Request, Path(letter_id): Path<Uuid>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "sekretaris"]) {
            return res;
        }

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let _ = sqlx::query("DELETE FROM letters WHERE id = $1 AND org_id = $2")
            .bind(letter_id)
            .bind(org_id)
            .execute(&pool)
            .await;

        Redirect::to("/surat").into_response()
    }

    /// Unduh Berkas PDF Surat Resmi (GET /surat/:id/pdf)
    pub async fn export_pdf(req: Request, Path(letter_id): Path<Uuid>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

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

        #[derive(sqlx::FromRow)]
        struct RawPdfData {
            number: String,
            subject: String,
            body: String,
            recipient: String,
            created_at: DateTime<Utc>,
        }

        let raw: RawPdfData = match sqlx::query_as(
            "SELECT number, subject, body, recipient, created_at FROM letters WHERE id = $1 AND org_id = $2",
        )
        .bind(letter_id)
        .bind(org_id)
        .fetch_one(&pool)
        .await
        {
            Ok(l) => l,
            Err(_) => return Html("<h1>Surat Tidak Ditemukan</h1>".to_string()).into_response(),
        };

        let local_time: DateTime<Local> = DateTime::from(raw.created_at);
        let date_str = local_time.format("%d %B %Y").to_string();

        let pdf_bytes = LetterPdfService::generate_pdf(
            &org_name,
            &raw.number,
            &raw.subject,
            &raw.recipient,
            &date_str,
            &raw.body,
        );

        let safe_filename = format!("surat_{}.pdf", raw.number.replace(['/', ' '], "_"));

        let headers = [
            (header::CONTENT_TYPE, "application/pdf"),
            (
                header::CONTENT_DISPOSITION,
                &format!("inline; filename=\"{}\"", safe_filename),
            ),
        ];

        (headers, pdf_bytes).into_response()
    }

    /// Kelola Template Surat Kustom (GET /surat/template)
    pub async fn templates(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

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

        #[derive(sqlx::FromRow)]
        struct RawTmpl {
            id: Uuid,
            name: String,
            body: String,
            created_at: DateTime<Utc>,
        }

        let raw_templates: Vec<RawTmpl> = sqlx::query_as(
            "SELECT id, name, body, created_at FROM letter_templates WHERE org_id = $1 ORDER BY name ASC",
        )
        .bind(org_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let templates: Vec<CustomTemplateView> = raw_templates
            .into_iter()
            .map(|t| {
                let local_time: DateTime<Local> = DateTime::from(t.created_at);
                CustomTemplateView {
                    id: t.id,
                    name: t.name,
                    body: t.body,
                    created_at_formatted: local_time.format("%d/%m/%Y").to_string(),
                }
            })
            .collect();

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Template Surat — Paguyuban");
        ctx.insert("area", "admin");
        ctx.insert("active", "surat");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("templates", templates);

        let html = ViewService::render_blade("letters/templates.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Simpan Template Baru (POST /surat/template)
    pub async fn store_template(req: Request, Form(form): Form<CreateTemplateForm>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "sekretaris"]) {
            return res;
        }

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        if form.name.trim().is_empty() || form.body.trim().is_empty() {
            return Redirect::to("/surat/template").into_response();
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let tmpl_id = Uuid::new_v4();
        let _ = sqlx::query(
            "INSERT INTO letter_templates (id, org_id, name, body) VALUES ($1, $2, $3, $4)",
        )
        .bind(tmpl_id)
        .bind(org_id)
        .bind(form.name.trim())
        .bind(form.body.trim())
        .execute(&pool)
        .await;

        Redirect::to("/surat/template").into_response()
    }
}
