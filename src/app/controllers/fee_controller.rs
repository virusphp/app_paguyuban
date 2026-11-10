use axum::{
    extract::{Form, Path, Query},
    response::{Html, IntoResponse, Redirect, Response},
};
use chrono::{Datelike, Local, NaiveDate};
use lumina::core::blade::BladeContext;
use lumina::prelude::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::app::services::{AuthContext, FeeService, ViewService};

#[derive(Debug, Deserialize)]
pub struct FeeFilterQuery {
    pub year: Option<i32>,
    pub q: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTariffForm {
    pub name: String,
    pub amount_idr: i64,
    pub period: String,
    pub active_from: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateInvoiceForm {
    pub fee_plan_id: Uuid,
    pub month: Option<u32>,
    pub year: i32,
    pub scope: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PayInvoiceForm {
    pub method: String,
    pub paid_on: String,
    pub note: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MonthCell {
    pub month_num: u32,
    pub month_name: String,
    pub status: String, // 'lunas', 'belum', 'empty'
    pub invoice_id: Option<Uuid>,
    pub amount_raw: i64,
    pub amount_formatted: String,
    pub paid_date: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MemberMatrixRow {
    pub member_id: Uuid,
    pub full_name: String,
    pub phone: Option<String>,
    pub house_no: String,
    pub months: Vec<MonthCell>,
    pub total_paid_raw: i64,
    pub total_paid_formatted: String,
    pub total_unpaid_raw: i64,
    pub total_unpaid_formatted: String,
    pub paid_months_count: usize,
    pub unpaid_months_count: usize,
}

pub struct FeeController;

impl FeeController {
    /// Matriks Rekapitulasi Iuran Warga 12 Bulan (GET /iuran)
    pub async fn index(req: Request, Query(params): Query<FeeFilterQuery>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        let can_manage = auth.require_role(&["admin", "bendahara"]).is_ok();
        let org_name = auth
            .current_org_name
            .clone()
            .unwrap_or_else(|| "Paguyuban".to_string());

        let current_date = Local::now().date_naive();
        let selected_year = params.year.unwrap_or(current_date.year());
        let search_text = params.q.unwrap_or_default().trim().to_string();

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        // 1. Ambil daftar warga aktif
        #[derive(sqlx::FromRow)]
        struct MemberDbRow {
            id: Uuid,
            full_name: String,
            phone: Option<String>,
            house_no: Option<String>,
        }

        let search_pattern = format!("%{}%", search_text);
        let members: Vec<MemberDbRow> = sqlx::query_as(
            r#"
            SELECT m.id, m.full_name, m.phone, h.house_no
            FROM members m
            LEFT JOIN households h ON m.household_id = h.id
            WHERE m.org_id = $1 
              AND m.status = 'aktif'
              AND (
                  $2 = '%%' 
                  OR m.full_name ILIKE $2 
                  OR h.house_no ILIKE $2
              )
            ORDER BY COALESCE(h.house_no, 'ZZZ') ASC, m.full_name ASC
            "#,
        )
        .bind(org_id)
        .bind(&search_pattern)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        // 2. Ambil tagihan pada tahun yang dipilih
        let start_date = match NaiveDate::from_ymd_opt(selected_year, 1, 1) {
            Some(d) => d,
            None => NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        };
        let end_date = match NaiveDate::from_ymd_opt(selected_year, 12, 31) {
            Some(d) => d,
            None => NaiveDate::from_ymd_opt(2026, 12, 31).unwrap(),
        };

        #[derive(sqlx::FromRow)]
        struct InvoiceDbRow {
            id: Uuid,
            member_id: Uuid,
            period_month: NaiveDate,
            amount_idr: i64,
            status: String,
            paid_on: Option<NaiveDate>,
        }

        let invoices: Vec<InvoiceDbRow> = sqlx::query_as(
            r#"
            SELECT i.id, i.member_id, i.period_month, i.amount_idr, i.status, p.paid_on
            FROM invoices i
            LEFT JOIN payments p ON p.invoice_id = i.id
            WHERE i.org_id = $1 
              AND i.period_month >= $2 
              AND i.period_month <= $3
            ORDER BY i.period_month ASC
            "#,
        )
        .bind(org_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let month_names = [
            "Jan", "Feb", "Mar", "Apr", "Mei", "Jun", "Jul", "Agu", "Sep", "Okt", "Nov", "Des",
        ];

        let mut matrix_rows: Vec<MemberMatrixRow> = Vec::new();
        let mut total_billed_idr: i64 = 0;
        let mut total_collected_idr: i64 = 0;
        let mut total_arrears_idr: i64 = 0;
        let mut total_invoices_count: usize = 0;
        let mut paid_invoices_count: usize = 0;
        let mut unpaid_members_count: usize = 0;

        for member in members {
            let mut row_cells: Vec<MonthCell> = Vec::with_capacity(12);
            let mut member_paid_idr: i64 = 0;
            let mut member_unpaid_idr: i64 = 0;
            let mut member_paid_count: usize = 0;
            let mut member_unpaid_count: usize = 0;

            for m in 1..=12 {
                let month_name = month_names[(m - 1) as usize].to_string();
                let matching_inv = invoices
                    .iter()
                    .find(|inv| inv.member_id == member.id && inv.period_month.month() == m);

                if let Some(inv) = matching_inv {
                    total_invoices_count += 1;
                    total_billed_idr += inv.amount_idr;

                    if inv.status == "lunas" {
                        member_paid_idr += inv.amount_idr;
                        member_paid_count += 1;
                        total_collected_idr += inv.amount_idr;
                        paid_invoices_count += 1;

                        row_cells.push(MonthCell {
                            month_num: m,
                            month_name,
                            status: "lunas".to_string(),
                            invoice_id: Some(inv.id),
                            amount_raw: inv.amount_idr,
                            amount_formatted: ViewService::format_rupiah(inv.amount_idr),
                            paid_date: inv.paid_on.map(|d| d.format("%d %b").to_string()),
                        });
                    } else {
                        member_unpaid_idr += inv.amount_idr;
                        member_unpaid_count += 1;
                        total_arrears_idr += inv.amount_idr;

                        row_cells.push(MonthCell {
                            month_num: m,
                            month_name,
                            status: "belum".to_string(),
                            invoice_id: Some(inv.id),
                            amount_raw: inv.amount_idr,
                            amount_formatted: ViewService::format_rupiah(inv.amount_idr),
                            paid_date: None,
                        });
                    }
                } else {
                    row_cells.push(MonthCell {
                        month_num: m,
                        month_name,
                        status: "empty".to_string(),
                        invoice_id: None,
                        amount_raw: 0,
                        amount_formatted: "Rp0".to_string(),
                        paid_date: None,
                    });
                }
            }

            if member_unpaid_count > 0 {
                unpaid_members_count += 1;
            }

            matrix_rows.push(MemberMatrixRow {
                member_id: member.id,
                full_name: member.full_name,
                phone: member.phone,
                house_no: member.house_no.unwrap_or_else(|| "-".to_string()),
                months: row_cells,
                total_paid_raw: member_paid_idr,
                total_paid_formatted: ViewService::format_rupiah(member_paid_idr),
                total_unpaid_raw: member_unpaid_idr,
                total_unpaid_formatted: ViewService::format_rupiah(member_unpaid_idr),
                paid_months_count: member_paid_count,
                unpaid_months_count: member_unpaid_count,
            });
        }

        let compliance_percentage = if total_billed_idr > 0 {
            ((total_collected_idr as f64 / total_billed_idr as f64) * 100.0).round() as i64
        } else {
            100
        };

        let available_years = vec![selected_year - 1, selected_year, selected_year + 1];

        let mut ctx = BladeContext::new();
        ctx.insert(
            "title",
            format!("Rekap Iuran {} — Paguyuban", selected_year),
        );
        ctx.insert("active", "iuran");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("can_manage", can_manage);
        ctx.insert("selected_year", selected_year);
        ctx.insert("available_years", available_years);
        ctx.insert("query_search", search_text);

        ctx.insert("matrix_rows", matrix_rows);
        ctx.insert("total_billed_raw", total_billed_idr);
        ctx.insert(
            "total_billed_formatted",
            ViewService::format_rupiah(total_billed_idr),
        );
        ctx.insert("total_collected_raw", total_collected_idr);
        ctx.insert(
            "total_collected_formatted",
            ViewService::format_rupiah(total_collected_idr),
        );
        ctx.insert("total_arrears_raw", total_arrears_idr);
        ctx.insert(
            "total_arrears_formatted",
            ViewService::format_rupiah(total_arrears_idr),
        );
        ctx.insert("total_invoices_count", total_invoices_count);
        ctx.insert("paid_invoices_count", paid_invoices_count);
        ctx.insert("unpaid_members_count", unpaid_members_count);
        ctx.insert("compliance_percentage", compliance_percentage);

        let html = ViewService::render_blade("fees/index.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Kelola Tarif Iuran (GET /iuran/tarif)
    pub async fn tariffs(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        let can_manage = auth.require_role(&["admin", "bendahara"]).is_ok();
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
        struct FeePlanRow {
            id: Uuid,
            name: String,
            amount_idr: i64,
            period: String,
            active_from: NaiveDate,
            amount_formatted: String,
        }

        #[derive(sqlx::FromRow)]
        struct RawPlan {
            id: Uuid,
            name: String,
            amount_idr: i64,
            period: String,
            active_from: NaiveDate,
        }

        let raw_plans: Vec<RawPlan> = sqlx::query_as(
            "SELECT id, name, amount_idr, period, active_from FROM fee_plans WHERE org_id = $1 ORDER BY created_at DESC",
        )
        .bind(org_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let fee_plans: Vec<FeePlanRow> = raw_plans
            .into_iter()
            .map(|p| FeePlanRow {
                id: p.id,
                name: p.name,
                amount_idr: p.amount_idr,
                period: p.period,
                active_from: p.active_from,
                amount_formatted: ViewService::format_rupiah(p.amount_idr),
            })
            .collect();

        let today_date = Local::now().format("%Y-%m-%d").to_string();

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Tarif Iuran — Paguyuban");
        ctx.insert("active", "iuran");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("can_manage", can_manage);
        ctx.insert("fee_plans", fee_plans);
        ctx.insert("today_date", today_date);

        let html = ViewService::render_blade("fees/tariffs.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Simpan Tarif Baru (POST /iuran/tarif)
    pub async fn store_tariff(req: Request, Form(form): Form<CreateTariffForm>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "bendahara"]) {
            return res;
        }

        if form.name.trim().is_empty() || form.amount_idr <= 0 {
            return Redirect::to("/iuran/tarif").into_response();
        }

        let active_from = NaiveDate::parse_from_str(&form.active_from, "%Y-%m-%d")
            .unwrap_or_else(|_| Local::now().date_naive());

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let plan_id = Uuid::new_v4();
        let insert_res = sqlx::query(
            r#"
            INSERT INTO fee_plans (id, org_id, name, amount_idr, period, active_from)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(plan_id)
        .bind(org_id)
        .bind(form.name.trim())
        .bind(form.amount_idr)
        .bind(form.period.trim())
        .bind(active_from)
        .execute(&pool)
        .await;

        if let Err(e) = insert_res {
            return Html(format!("<h1>Gagal Menyimpan Tarif: {}</h1>", e)).into_response();
        }

        // Catat Audit Log
        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (org_id, actor_user_id, action, entity, entity_id, meta)
            VALUES ($1, $2, 'create_fee_plan', 'fee_plans', $3, $4)
            "#,
        )
        .bind(org_id)
        .bind(auth.user_id)
        .bind(plan_id.to_string())
        .bind(json!({
            "name": form.name.trim(),
            "amount_idr": form.amount_idr,
            "period": form.period.trim()
        }))
        .execute(&pool)
        .await;

        Redirect::to("/iuran/tarif").into_response()
    }

    /// Form Generate Tagihan Massal (GET /iuran/generate)
    pub async fn generate_view(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "bendahara"]) {
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

        #[derive(sqlx::FromRow, Serialize)]
        struct PlanOption {
            id: Uuid,
            name: String,
            period: String,
            amount_formatted: String,
        }

        #[derive(sqlx::FromRow)]
        struct RawPlanOption {
            id: Uuid,
            name: String,
            period: String,
            amount_idr: i64,
        }

        let raw_plans: Vec<RawPlanOption> = sqlx::query_as(
            "SELECT id, name, period, amount_idr FROM fee_plans WHERE org_id = $1 ORDER BY created_at DESC",
        )
        .bind(org_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let fee_plans: Vec<PlanOption> = raw_plans
            .into_iter()
            .map(|p| PlanOption {
                id: p.id,
                name: p.name,
                period: p.period,
                amount_formatted: ViewService::format_rupiah(p.amount_idr),
            })
            .collect();

        let active_members_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM members WHERE org_id = $1 AND status = 'aktif'",
        )
        .bind(org_id)
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

        let now = Local::now().date_naive();
        let current_month = now.month();
        let current_year = now.year();

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Terbitkan Tagihan Massal — Paguyuban");
        ctx.insert("active", "iuran");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("fee_plans", fee_plans);
        ctx.insert("active_members_count", active_members_count);
        ctx.insert("current_month", current_month);
        ctx.insert("current_year", current_year);

        let html = ViewService::render_blade("fees/generate.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Aksi Cepat Sinkronkan Tagihan Bulan Berjalan (POST /iuran/sinkronkan)
    pub async fn sync_invoices_action(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "bendahara"]) {
            return res;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        if let Ok(pool) = sqlx::PgPool::connect(&db_url).await {
            let count = FeeService::sync_invoices_for_all_active_members(&pool, org_id)
                .await
                .unwrap_or(0);

            let _ = sqlx::query(
                r#"
                INSERT INTO audit_logs (org_id, actor_user_id, action, entity, meta)
                VALUES ($1, $2, 'SYNC_INVOICES', 'invoices', $3);
                "#,
            )
            .bind(Some(org_id))
            .bind(auth.user_id)
            .bind(json!({ "generated_count": count }))
            .execute(&pool)
            .await;
        }

        Redirect::to("/iuran").into_response()
    }

    /// Eksekusi Generate Tagihan Massal (POST /iuran/generate)
    pub async fn generate_action(req: Request, Form(form): Form<GenerateInvoiceForm>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "bendahara"]) {
            return res;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let now = Local::now().date_naive();
        let scope = form.scope.as_deref().unwrap_or("single");

        let (start_month, end_month) = match scope {
            "up_to_current" => {
                let max_m = if form.year == now.year() {
                    now.month()
                } else if form.year < now.year() {
                    12
                } else {
                    1
                };
                (1, max_m)
            }
            "full_year" => (1, 12),
            _ => {
                let m = form.month.unwrap_or(now.month()).clamp(1, 12);
                (m, m)
            }
        };

        let inserted_count = match FeeService::generate_invoices_batch(
            &pool,
            org_id,
            form.fee_plan_id,
            form.year,
            start_month,
            end_month,
        )
        .await
        {
            Ok(cnt) => cnt,
            Err(e) => {
                return Html(format!("<h1>Gagal Menerbitkan Tagihan: {}</h1>", e)).into_response()
            }
        };

        // Catat Audit Log
        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (org_id, actor_user_id, action, entity, entity_id, meta)
            VALUES ($1, $2, 'generate_invoices', 'invoices', NULL, $3)
            "#,
        )
        .bind(org_id)
        .bind(auth.user_id)
        .bind(json!({
            "plan_id": form.fee_plan_id,
            "year": form.year,
            "scope": scope,
            "start_month": start_month,
            "end_month": end_month,
            "generated_count": inserted_count
        }))
        .execute(&pool)
        .await;

        Redirect::to(&format!("/iuran?year={}", form.year)).into_response()
    }

    /// Form Bayar Tagihan (GET /iuran/bayar/:invoice_id)
    pub async fn pay_view(req: Request, Path(invoice_id): Path<Uuid>) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "bendahara"]) {
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
        #[allow(dead_code)]
        struct InvDetail {
            id: Uuid,
            amount_idr: i64,
            period_month: NaiveDate,
            status: String,
            member_name: String,
            house_no: Option<String>,
            plan_name: String,
        }

        let invoice: InvDetail = match sqlx::query_as(
            r#"
            SELECT i.id, i.amount_idr, i.period_month, i.status,
                   m.full_name AS member_name, h.house_no,
                   fp.name AS plan_name
            FROM invoices i
            JOIN members m ON i.member_id = m.id
            LEFT JOIN households h ON m.household_id = h.id
            JOIN fee_plans fp ON i.fee_plan_id = fp.id
            WHERE i.id = $1 AND i.org_id = $2
            "#,
        )
        .bind(invoice_id)
        .bind(org_id)
        .fetch_one(&pool)
        .await
        {
            Ok(inv) => inv,
            Err(_) => return Html("<h1>Tagihan Tidak Ditemukan</h1>".to_string()).into_response(),
        };

        let today_date = Local::now().format("%Y-%m-%d").to_string();
        let period_label = invoice.period_month.format("%B %Y").to_string();

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Catat Pembayaran Iuran — Paguyuban");
        ctx.insert("active", "iuran");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("invoice_id", invoice.id);
        ctx.insert("invoice_member_name", invoice.member_name);
        ctx.insert(
            "invoice_house_no",
            invoice.house_no.unwrap_or_else(|| "-".to_string()),
        );
        ctx.insert("invoice_plan_name", invoice.plan_name);
        ctx.insert("invoice_period_label", period_label);
        ctx.insert(
            "invoice_amount_formatted",
            ViewService::format_rupiah(invoice.amount_idr),
        );
        ctx.insert("today_date", today_date);

        let html = ViewService::render_blade("fees/pay.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Eksekusi Pelunasan Tagihan Atomik (POST /iuran/bayar/:invoice_id)
    pub async fn pay_action(
        req: Request,
        Path(invoice_id): Path<Uuid>,
        Form(form): Form<PayInvoiceForm>,
    ) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let org_id = match auth.require_org() {
            Ok(id) => id,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "bendahara"]) {
            return res;
        }

        let paid_on = NaiveDate::parse_from_str(&form.paid_on, "%Y-%m-%d")
            .unwrap_or_else(|_| Local::now().date_naive());

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        // Ambil invoice
        #[derive(sqlx::FromRow)]
        struct InvRow {
            amount_idr: i64,
            period_month: NaiveDate,
            member_name: String,
            house_no: Option<String>,
        }

        let inv_data: InvRow = match sqlx::query_as(
            r#"
            SELECT i.amount_idr, i.period_month, m.full_name AS member_name, h.house_no
            FROM invoices i
            JOIN members m ON i.member_id = m.id
            LEFT JOIN households h ON m.household_id = h.id
            WHERE i.id = $1 AND i.org_id = $2
            "#,
        )
        .bind(invoice_id)
        .bind(org_id)
        .fetch_one(&pool)
        .await
        {
            Ok(r) => r,
            Err(_) => return Html("<h1>Tagihan Tidak Ditemukan</h1>".to_string()).into_response(),
        };

        // TRANSAKSI ATOMIK (Pelunasan + Kas Masuk)
        let mut tx = match pool.begin().await {
            Ok(t) => t,
            Err(e) => {
                return Html(format!("<h1>Gagal Memulai Transaksi: {}</h1>", e)).into_response()
            }
        };

        // 1. Update invoice status
        let upd_res =
            sqlx::query("UPDATE invoices SET status = 'lunas' WHERE id = $1 AND org_id = $2")
                .bind(invoice_id)
                .bind(org_id)
                .execute(&mut *tx)
                .await;

        if let Err(e) = upd_res {
            return Html(format!("<h1>Gagal Memperbarui Tagihan: {}</h1>", e)).into_response();
        }

        // 2. Insert ke payments
        let payment_id = Uuid::new_v4();
        let pay_res = sqlx::query(
            r#"
            INSERT INTO payments (id, org_id, invoice_id, amount_idr, method, paid_on, confirmed_by, note)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(payment_id)
        .bind(org_id)
        .bind(invoice_id)
        .bind(inv_data.amount_idr)
        .bind(form.method.trim())
        .bind(paid_on)
        .bind(auth.user_id)
        .bind(&form.note)
        .execute(&mut *tx)
        .await;

        if let Err(e) = pay_res {
            return Html(format!("<h1>Gagal Mencatat Pembayaran: {}</h1>", e)).into_response();
        }

        // 3. Insert ke transactions (Kas Paguyuban: kind = 'masuk', category = 'iuran')
        let trans_id = Uuid::new_v4();
        let period_str = inv_data.period_month.format("%b %Y").to_string();
        let house_str = inv_data.house_no.unwrap_or_else(|| "-".to_string());
        let desc = format!(
            "Iuran {} - {} ({})",
            period_str, inv_data.member_name, house_str
        );

        let trans_res = sqlx::query(
            r#"
            INSERT INTO transactions (id, org_id, kind, category, description, amount_idr, occurred_on, source_payment_id, created_by)
            VALUES ($1, $2, 'masuk', 'iuran', $3, $4, $5, $6, $7)
            "#,
        )
        .bind(trans_id)
        .bind(org_id)
        .bind(desc)
        .bind(inv_data.amount_idr)
        .bind(paid_on)
        .bind(payment_id)
        .bind(auth.user_id)
        .execute(&mut *tx)
        .await;

        if let Err(e) = trans_res {
            return Html(format!("<h1>Gagal Mencatat Buku Kas: {}</h1>", e)).into_response();
        }

        // 4. Catat Audit Log
        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (org_id, actor_user_id, action, entity, entity_id, meta)
            VALUES ($1, $2, 'pay_invoice', 'invoices', $3, $4)
            "#,
        )
        .bind(org_id)
        .bind(auth.user_id)
        .bind(invoice_id.to_string())
        .bind(json!({
            "payment_id": payment_id.to_string(),
            "transaction_id": trans_id.to_string(),
            "amount_idr": inv_data.amount_idr,
            "method": form.method.trim()
        }))
        .execute(&mut *tx)
        .await;

        // Commit transaksi
        if let Err(e) = tx.commit().await {
            return Html(format!("<h1>Gagal Menyimpan Transaksi Atomik: {}</h1>", e))
                .into_response();
        }

        let year = inv_data.period_month.year();
        Redirect::to(&format!("/iuran?year={}", year)).into_response()
    }
}
