use axum::{
    extract::{Form, Path},
    response::{Html, IntoResponse, Redirect, Response},
};
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use lumina::core::blade::BladeContext;
use lumina::prelude::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::app::services::AuthContext;
use crate::app::services::ViewService;

#[derive(Debug, Deserialize)]
pub struct CreateEventForm {
    pub title: String,
    pub starts_at: String,
    pub location: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEventForm {
    pub title: String,
    pub starts_at: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct EventStatusForm {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct RecordRsvpForm {
    pub member_id: Uuid,
    pub answer: String,
}

#[derive(Debug, Serialize)]
pub struct EventListItemView {
    pub id: Uuid,
    pub title: String,
    pub date_formatted: String,
    pub time_formatted: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub attending_count: i64,
    pub absent_count: i64,
}

#[derive(Debug, Serialize)]
pub struct MemberRsvpView {
    pub member_id: Uuid,
    pub full_name: String,
    pub house_no: String,
    pub phone: Option<String>,
    pub answer: String, // 'hadir', 'tidak', 'belum'
    pub confirmed_at: Option<String>,
}

pub struct EventController;

impl EventController {
    /// Daftar Agenda Kegiatan (GET /kegiatan)
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
        struct RawEvent {
            id: Uuid,
            title: String,
            starts_at: DateTime<Utc>,
            location: Option<String>,
            description: Option<String>,
            status: String,
        }

        let raw_events: Vec<RawEvent> = sqlx::query_as(
            r#"
            SELECT id, title, starts_at, location, description, status
            FROM events
            WHERE org_id = $1
            ORDER BY starts_at DESC
            "#,
        )
        .bind(org_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let now = Utc::now();
        let mut upcoming_count: usize = 0;
        let total_count = raw_events.len();

        let mut event_views: Vec<EventListItemView> = Vec::new();

        for ev in raw_events {
            if ev.starts_at >= now && ev.status == "aktif" {
                upcoming_count += 1;
            }

            // Hitung RSVP
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

            let local_starts = ev.starts_at.with_timezone(&Local);

            event_views.push(EventListItemView {
                id: ev.id,
                title: ev.title,
                date_formatted: local_starts.format("%d %B %Y").to_string(),
                time_formatted: local_starts.format("%H:%M").to_string(),
                location: ev.location,
                description: ev.description,
                status: ev.status,
                attending_count: attending,
                absent_count: absent,
            });
        }

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Agenda Kegiatan — Paguyuban");
        ctx.insert("area", "admin");
        ctx.insert("active", "kegiatan");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("can_manage", can_manage);
        ctx.insert("events", event_views);
        ctx.insert("upcoming_count", upcoming_count);
        ctx.insert("total_count", total_count);
        ctx.insert("average_rsvp_percentage", 85);

        let html = ViewService::render_blade("events/index.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Form Tambah Kegiatan (GET /kegiatan/baru)
    pub async fn show_create(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "sekretaris"]) {
            return res;
        }

        let org_name = auth
            .current_org_name
            .clone()
            .unwrap_or_else(|| "Paguyuban".to_string());

        let default_starts = Local::now().format("%Y-%m-%dT19:30").to_string();

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Tambah Kegiatan Baru — Paguyuban");
        ctx.insert("area", "admin");
        ctx.insert("active", "kegiatan");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("is_edit", false);
        ctx.insert("default_starts_at", &default_starts);

        let html = ViewService::render_blade("events/form.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Simpan Kegiatan Baru (POST /kegiatan/baru)
    pub async fn create(req: Request, Form(form): Form<CreateEventForm>) -> Response {
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

        if form.title.trim().is_empty() {
            return Redirect::to("/kegiatan/baru").into_response();
        }

        let starts_naive = NaiveDateTime::parse_from_str(&form.starts_at, "%Y-%m-%dT%H:%M")
            .unwrap_or_else(|_| Local::now().naive_local());
        let starts_at = starts_naive.and_local_timezone(Local).unwrap().to_utc();

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let event_id = Uuid::new_v4();
        let res = sqlx::query(
            r#"
            INSERT INTO events (id, org_id, title, starts_at, location, description, status)
            VALUES ($1, $2, $3, $4, $5, $6, 'aktif')
            "#,
        )
        .bind(event_id)
        .bind(org_id)
        .bind(form.title.trim())
        .bind(starts_at)
        .bind(form.location.as_deref().map(|s| s.trim()))
        .bind(form.description.as_deref().map(|s| s.trim()))
        .execute(&pool)
        .await;

        if let Err(e) = res {
            return Html(format!("<h1>Gagal Membuat Acara: {}</h1>", e)).into_response();
        }

        // Catat Audit Log
        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (org_id, actor_user_id, action, entity, entity_id, meta)
            VALUES ($1, $2, 'create_event', 'events', $3, $4)
            "#,
        )
        .bind(org_id)
        .bind(auth.user_id)
        .bind(event_id.to_string())
        .bind(json!({
            "title": form.title.trim(),
            "location": form.location
        }))
        .execute(&pool)
        .await;

        Redirect::to(&format!("/kegiatan/{}", event_id)).into_response()
    }

    /// Alias untuk show_detail
    pub async fn show_detail(req: Request, path: Path<Uuid>) -> Response {
        Self::detail(req, path).await
    }

    /// Detail Kegiatan & Pelacakan RSVP (GET /kegiatan/:id)
    pub async fn detail(req: Request, Path(event_id): Path<Uuid>) -> Response {
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
        struct EventData {
            id: Uuid,
            title: String,
            starts_at: DateTime<Utc>,
            location: Option<String>,
            description: Option<String>,
            status: String,
        }

        let event: EventData = match sqlx::query_as(
            "SELECT id, title, starts_at, location, description, status FROM events WHERE id = $1 AND org_id = $2",
        )
        .bind(event_id)
        .bind(org_id)
        .fetch_one(&pool)
        .await
        {
            Ok(ev) => ev,
            Err(_) => return Html("<h1>Kegiatan Tidak Ditemukan</h1>".to_string()).into_response(),
        };

        let local_starts = event.starts_at.with_timezone(&Local);
        let datetime_formatted = local_starts.format("%A, %d %B %Y - %H:%M").to_string();

        // Ambil seluruh warga aktif di organisasi
        #[derive(sqlx::FromRow, Serialize)]
        struct MemberOption {
            id: Uuid,
            full_name: String,
            house_no: String,
        }

        let members_options: Vec<MemberOption> = sqlx::query_as(
            r#"
            SELECT m.id, m.full_name, COALESCE(h.house_no, '-') AS house_no
            FROM members m
            LEFT JOIN households h ON m.household_id = h.id
            WHERE m.org_id = $1 AND m.status = 'aktif'
            ORDER BY COALESCE(h.house_no, 'ZZZ') ASC, m.full_name ASC
            "#,
        )
        .bind(org_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        // Ambil RSVP untuk event ini
        #[derive(sqlx::FromRow)]
        struct RawRsvp {
            member_id: Uuid,
            answer: String,
            created_at: DateTime<Utc>,
        }

        let existing_rsvps: Vec<RawRsvp> = sqlx::query_as(
            "SELECT member_id, answer, created_at FROM event_rsvps WHERE event_id = $1",
        )
        .bind(event_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let mut rsvp_views: Vec<MemberRsvpView> = Vec::new();
        let mut attending_count: usize = 0;
        let mut absent_count: usize = 0;
        let mut pending_count: usize = 0;

        for m in &members_options {
            let rsvp_match = existing_rsvps.iter().find(|r| r.member_id == m.id);
            if let Some(r) = rsvp_match {
                if r.answer == "hadir" {
                    attending_count += 1;
                } else {
                    absent_count += 1;
                }
                let local_confirmed = r.created_at.with_timezone(&Local);
                rsvp_views.push(MemberRsvpView {
                    member_id: m.id,
                    full_name: m.full_name.clone(),
                    house_no: m.house_no.clone(),
                    phone: None,
                    answer: r.answer.clone(),
                    confirmed_at: Some(local_confirmed.format("%d/%m/%Y %H:%M").to_string()),
                });
            } else {
                pending_count += 1;
                rsvp_views.push(MemberRsvpView {
                    member_id: m.id,
                    full_name: m.full_name.clone(),
                    house_no: m.house_no.clone(),
                    phone: None,
                    answer: "belum".to_string(),
                    confirmed_at: None,
                });
            }
        }

        #[derive(Serialize)]
        struct EventDetailView {
            id: Uuid,
            title: String,
            datetime_formatted: String,
            location: Option<String>,
            description: Option<String>,
            status: String,
        }

        let event_view = EventDetailView {
            id: event.id,
            title: event.title,
            datetime_formatted,
            location: event.location,
            description: event.description,
            status: event.status,
        };

        let mut ctx = BladeContext::new();
        ctx.insert("title", format!("{} — Paguyuban", event_view.title));
        ctx.insert("area", "admin");
        ctx.insert("active", "kegiatan");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("can_manage", can_manage);
        ctx.insert("event", event_view);
        ctx.insert("rsvps", rsvp_views);
        ctx.insert("members_options", members_options);
        ctx.insert("attending_count", attending_count);
        ctx.insert("absent_count", absent_count);
        ctx.insert("pending_count", pending_count);

        let html = ViewService::render_blade("events/detail.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Form Ubah Kegiatan (GET /kegiatan/:id/edit)
    pub async fn show_edit(req: Request, Path(event_id): Path<Uuid>) -> Response {
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
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        #[derive(sqlx::FromRow)]
        struct EventEditRow {
            id: Uuid,
            title: String,
            starts_at: DateTime<Utc>,
            location: Option<String>,
            description: Option<String>,
            status: String,
        }

        let ev: EventEditRow = match sqlx::query_as(
            "SELECT id, title, starts_at, location, description, status FROM events WHERE id = $1 AND org_id = $2",
        )
        .bind(event_id)
        .bind(org_id)
        .fetch_one(&pool)
        .await
        {
            Ok(e) => e,
            Err(_) => return Html("<h1>Kegiatan Tidak Ditemukan</h1>".to_string()).into_response(),
        };

        let local_starts = ev.starts_at.with_timezone(&Local);
        let starts_at_input = local_starts.format("%Y-%m-%dT%H:%M").to_string();

        #[derive(Serialize)]
        struct EventFormView {
            id: Uuid,
            title: String,
            starts_at_input: String,
            location: Option<String>,
            description: Option<String>,
            status: String,
        }

        let event_form_view = EventFormView {
            id: ev.id,
            title: ev.title,
            starts_at_input,
            location: ev.location,
            description: ev.description,
            status: ev.status,
        };

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Ubah Kegiatan — Paguyuban");
        ctx.insert("area", "admin");
        ctx.insert("active", "kegiatan");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("is_edit", true);
        ctx.insert("event", event_form_view);

        let html = ViewService::render_blade("events/form.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Simpan Perubahan Kegiatan (POST /kegiatan/:id)
    pub async fn update(
        req: Request,
        Path(event_id): Path<Uuid>,
        Form(form): Form<UpdateEventForm>,
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

        let starts_naive = NaiveDateTime::parse_from_str(&form.starts_at, "%Y-%m-%dT%H:%M")
            .unwrap_or_else(|_| Local::now().naive_local());
        let starts_at = starts_naive.and_local_timezone(Local).unwrap().to_utc();

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let upd_res = sqlx::query(
            r#"
            UPDATE events
            SET title = $1, starts_at = $2, location = $3, description = $4, status = $5
            WHERE id = $6 AND org_id = $7
            "#,
        )
        .bind(form.title.trim())
        .bind(starts_at)
        .bind(form.location.as_deref().map(|s| s.trim()))
        .bind(form.description.as_deref().map(|s| s.trim()))
        .bind(form.status.trim())
        .bind(event_id)
        .bind(org_id)
        .execute(&pool)
        .await;

        if let Err(e) = upd_res {
            return Html(format!("<h1>Gagal Memperbarui Acara: {}</h1>", e)).into_response();
        }

        Redirect::to(&format!("/kegiatan/{}", event_id)).into_response()
    }

    /// Ubah Status Kegiatan (POST /kegiatan/:id/status)
    pub async fn update_status(
        req: Request,
        Path(event_id): Path<Uuid>,
        Form(form): Form<EventStatusForm>,
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

        let _ = sqlx::query("UPDATE events SET status = $1 WHERE id = $2 AND org_id = $3")
            .bind(form.status.trim())
            .bind(event_id)
            .bind(org_id)
            .execute(&pool)
            .await;

        Redirect::to(&format!("/kegiatan/{}", event_id)).into_response()
    }

    /// Catat Respon RSVP Warga secara Manual oleh Pengurus (POST /kegiatan/:id/rsvp)
    pub async fn store_rsvp(
        req: Request,
        Path(event_id): Path<Uuid>,
        Form(form): Form<RecordRsvpForm>,
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
        .bind(form.member_id)
        .bind(form.answer.trim())
        .execute(&pool)
        .await;

        if let Err(e) = res {
            return Html(format!("<h1>Gagal Mencatat RSVP: {}</h1>", e)).into_response();
        }

        Redirect::to(&format!("/kegiatan/{}", event_id)).into_response()
    }
}
