use axum::{
    extract::{Form, Path, Query},
    response::{Html, IntoResponse, Redirect, Response},
};
use chrono::{Datelike, Local, NaiveDate};
use docx_rs::*;
use lumina::core::blade::BladeContext;
use lumina::prelude::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;
use uuid::Uuid;

use crate::app::services::{AuthContext, ViewService};

#[derive(Debug, Deserialize)]
pub struct CashFilterQuery {
    pub year: Option<i32>,
    pub kind: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCashInForm {
    pub category: String,
    pub amount_idr: i64,
    pub description: String,
    pub donor_name: Option<String>,
    pub occurred_on: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCashOutForm {
    pub category: String,
    pub amount_idr: i64,
    pub description: String,
    pub donor_name: Option<String>,
    pub occurred_on: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateInKindForm {
    pub donor_name: String,
    pub item: String,
    pub quantity_text: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct TxRowView {
    pub id: Uuid,
    pub date_formatted: String,
    pub kind: String,
    pub category: String,
    pub category_label: String,
    pub description: String,
    pub donor_name: Option<String>,
    pub amount_idr: i64,
    pub amount_formatted: String,
    pub running_balance_formatted: String,
    pub source_payment_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Clone)]
pub struct InKindItemView {
    pub id: Uuid,
    pub date_formatted: String,
    pub donor_name: String,
    pub item: String,
    pub quantity_text: String,
}

pub struct CashController;

impl CashController {
    /// Buku Kas Umum & Riwayat Mutasi (GET /kas)
    pub async fn index(req: Request, Query(params): Query<CashFilterQuery>) -> Response {
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
        let filter_kind = params.kind.unwrap_or_default().trim().to_string();
        let filter_category = params.category.unwrap_or_default().trim().to_string();

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        // 1. Ambil seluruh transaksi dalam org_id secara kronologis untuk kalkulasi running balance
        #[derive(sqlx::FromRow)]
        struct RawTx {
            id: Uuid,
            kind: String,
            category: String,
            description: String,
            amount_idr: i64,
            occurred_on: NaiveDate,
            donor_name: Option<String>,
            source_payment_id: Option<Uuid>,
        }

        let all_txs: Vec<RawTx> = sqlx::query_as(
            r#"
            SELECT id, kind, category, description, amount_idr, occurred_on, donor_name, source_payment_id
            FROM transactions
            WHERE org_id = $1
            ORDER BY occurred_on ASC, created_at ASC
            "#,
        )
        .bind(org_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let mut cumulative_balance: i64 = 0;
        let mut year_income: i64 = 0;
        let mut year_expense: i64 = 0;
        let mut income_count: usize = 0;
        let mut expense_count: usize = 0;

        let mut tx_views_chrono: Vec<TxRowView> = Vec::new();

        for tx in all_txs {
            if tx.kind == "masuk" {
                cumulative_balance += tx.amount_idr;
            } else {
                cumulative_balance -= tx.amount_idr;
            }

            let is_in_year = tx.occurred_on.year() == selected_year;
            if is_in_year {
                if tx.kind == "masuk" {
                    year_income += tx.amount_idr;
                    income_count += 1;
                } else {
                    year_expense += tx.amount_idr;
                    expense_count += 1;
                }
            }

            let cat_label = match tx.category.as_str() {
                "saldo_awal" => "Saldo Awal".to_string(),
                "iuran" => "Iuran Warga".to_string(),
                "iuran_panitia" => "Iuran Panitia".to_string(),
                "donasi" => "Donasi Warga".to_string(),
                "lelang" => "Hasil Lelang".to_string(),
                "konsumsi" => "Konsumsi".to_string(),
                "hadiah" => "Hadiah & Lomba".to_string(),
                "perlengkapan" => "Perlengkapan".to_string(),
                "operasional" => "Operasional".to_string(),
                other => other.to_string(),
            };

            tx_views_chrono.push(TxRowView {
                id: tx.id,
                date_formatted: tx.occurred_on.format("%d/%m/%Y").to_string(),
                kind: tx.kind,
                category: tx.category,
                category_label: cat_label,
                description: tx.description,
                donor_name: tx.donor_name,
                amount_idr: tx.amount_idr,
                amount_formatted: ViewService::format_rupiah(tx.amount_idr),
                running_balance_formatted: ViewService::format_rupiah(cumulative_balance),
                source_payment_id: tx.source_payment_id,
            });
        }

        let current_balance = cumulative_balance;

        // Filter berdasarkan tahun, jenis, dan kategori untuk tampilan tabel
        let mut displayed_txs: Vec<TxRowView> = tx_views_chrono
            .into_iter()
            .filter(|tx| {
                let parts: Vec<&str> = tx.date_formatted.split('/').collect();
                if parts.len() == 3 {
                    if let Ok(y) = parts[2].parse::<i32>() {
                        if y != selected_year {
                            return false;
                        }
                    }
                }
                if !filter_kind.is_empty() && tx.kind != filter_kind {
                    return false;
                }
                if !filter_category.is_empty() && tx.category != filter_category {
                    return false;
                }
                true
            })
            .collect();

        // Urutkan mutasi terbaru di atas
        displayed_txs.reverse();

        // 2. Ambil Donasi Barang (In-Kind)
        #[derive(sqlx::FromRow)]
        struct RawInKind {
            id: Uuid,
            donor_name: String,
            item: String,
            quantity_text: String,
            created_at: chrono::DateTime<chrono::Utc>,
        }

        let raw_inkind: Vec<RawInKind> = sqlx::query_as(
            "SELECT id, donor_name, item, quantity_text, created_at FROM in_kind_donations WHERE org_id = $1 ORDER BY created_at DESC",
        )
        .bind(org_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let in_kind_items: Vec<InKindItemView> = raw_inkind
            .into_iter()
            .map(|k| InKindItemView {
                id: k.id,
                date_formatted: k.created_at.format("%d/%m/%Y").to_string(),
                donor_name: k.donor_name,
                item: k.item,
                quantity_text: k.quantity_text,
            })
            .collect();

        let available_years = vec![selected_year - 1, selected_year, selected_year + 1];

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Buku Kas & Keuangan — Paguyuban");
        ctx.insert("area", "admin");
        ctx.insert("active", "kas");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("can_manage", can_manage);
        ctx.insert("selected_year", selected_year);
        ctx.insert("available_years", available_years);
        ctx.insert("filter_kind", filter_kind);
        ctx.insert("filter_category", filter_category);

        ctx.insert(
            "current_balance_formatted",
            ViewService::format_rupiah(current_balance),
        );
        ctx.insert(
            "total_income_formatted",
            ViewService::format_rupiah(year_income),
        );
        ctx.insert(
            "total_expense_formatted",
            ViewService::format_rupiah(year_expense),
        );
        ctx.insert("income_count", income_count);
        ctx.insert("expense_count", expense_count);
        ctx.insert("in_kind_count", in_kind_items.len());

        ctx.insert("transactions", displayed_txs);
        ctx.insert("in_kind_items", in_kind_items);

        let html = ViewService::render_blade("cash/index.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Form Catat Kas Masuk (GET /kas/masuk)
    pub async fn show_in(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "bendahara"]) {
            return res;
        }

        let org_name = auth
            .current_org_name
            .clone()
            .unwrap_or_else(|| "Paguyuban".to_string());
        let today_date = Local::now().format("%Y-%m-%d").to_string();

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Catat Kas Masuk — Paguyuban");
        ctx.insert("area", "admin");
        ctx.insert("active", "kas");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("today_date", today_date);

        let html = ViewService::render_blade("cash/form_in.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Simpan Kas Masuk (POST /kas/masuk)
    pub async fn store_in(req: Request, Form(form): Form<CreateCashInForm>) -> Response {
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

        if form.amount_idr <= 0 || form.description.trim().is_empty() {
            return Redirect::to("/kas/masuk").into_response();
        }

        let occurred_on = NaiveDate::parse_from_str(&form.occurred_on, "%Y-%m-%d")
            .unwrap_or_else(|_| Local::now().date_naive());

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let tx_id = Uuid::new_v4();
        let donor = form
            .donor_name
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty());

        let res = sqlx::query(
            r#"
            INSERT INTO transactions (id, org_id, kind, category, description, amount_idr, occurred_on, donor_name, created_by)
            VALUES ($1, $2, 'masuk', $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(tx_id)
        .bind(org_id)
        .bind(form.category.trim())
        .bind(form.description.trim())
        .bind(form.amount_idr)
        .bind(occurred_on)
        .bind(donor)
        .bind(auth.user_id)
        .execute(&pool)
        .await;

        if let Err(e) = res {
            return Html(format!("<h1>Gagal Menyimpan Kas Masuk: {}</h1>", e)).into_response();
        }

        // Catat Audit Log
        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (org_id, actor_user_id, action, entity, entity_id, meta)
            VALUES ($1, $2, 'create_cash_in', 'transactions', $3, $4)
            "#,
        )
        .bind(org_id)
        .bind(auth.user_id)
        .bind(tx_id.to_string())
        .bind(json!({
            "category": form.category.trim(),
            "amount_idr": form.amount_idr,
            "description": form.description.trim()
        }))
        .execute(&pool)
        .await;

        Redirect::to(&format!("/kas?year={}", occurred_on.year())).into_response()
    }

    /// Form Catat Pengeluaran (GET /kas/keluar)
    pub async fn show_out(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "bendahara"]) {
            return res;
        }

        let org_name = auth
            .current_org_name
            .clone()
            .unwrap_or_else(|| "Paguyuban".to_string());
        let today_date = Local::now().format("%Y-%m-%d").to_string();

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Catat Pengeluaran Kas — Paguyuban");
        ctx.insert("area", "admin");
        ctx.insert("active", "kas");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("today_date", &today_date);

        let html = ViewService::render_blade("cash/form_out.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Simpan Kas Keluar (POST /kas/keluar)
    pub async fn store_out(req: Request, Form(form): Form<CreateCashOutForm>) -> Response {
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

        if form.amount_idr <= 0 || form.description.trim().is_empty() {
            return Redirect::to("/kas/keluar").into_response();
        }

        let occurred_on = NaiveDate::parse_from_str(&form.occurred_on, "%Y-%m-%d")
            .unwrap_or_else(|_| Local::now().date_naive());

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let tx_id = Uuid::new_v4();
        let recipient = form
            .donor_name
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty());

        let res = sqlx::query(
            r#"
            INSERT INTO transactions (id, org_id, kind, category, description, amount_idr, occurred_on, donor_name, created_by)
            VALUES ($1, $2, 'keluar', $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(tx_id)
        .bind(org_id)
        .bind(form.category.trim())
        .bind(form.description.trim())
        .bind(form.amount_idr)
        .bind(occurred_on)
        .bind(recipient)
        .bind(auth.user_id)
        .execute(&pool)
        .await;

        if let Err(e) = res {
            return Html(format!("<h1>Gagal Menyimpan Kas Keluar: {}</h1>", e)).into_response();
        }

        // Catat Audit Log
        let _ = sqlx::query(
            r#"
            INSERT INTO audit_logs (org_id, actor_user_id, action, entity, entity_id, meta)
            VALUES ($1, $2, 'create_cash_out', 'transactions', $3, $4)
            "#,
        )
        .bind(org_id)
        .bind(auth.user_id)
        .bind(tx_id.to_string())
        .bind(json!({
            "category": form.category.trim(),
            "amount_idr": form.amount_idr,
            "description": form.description.trim()
        }))
        .execute(&pool)
        .await;

        Redirect::to(&format!("/kas?year={}", occurred_on.year())).into_response()
    }

    /// Form Catat Donasi Barang (GET /kas/donasi-barang)
    pub async fn show_inkind(req: Request) -> Response {
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        if let Err(res) = auth.require_role(&["admin", "bendahara"]) {
            return res;
        }

        let org_name = auth
            .current_org_name
            .clone()
            .unwrap_or_else(|| "Paguyuban".to_string());

        let mut ctx = BladeContext::new();
        ctx.insert("title", "Catat Donasi Barang — Paguyuban");
        ctx.insert("area", "admin");
        ctx.insert("active", "kas");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());

        let html = ViewService::render_blade("cash/form_inkind.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Simpan Donasi Barang (POST /kas/donasi-barang)
    pub async fn store_inkind(req: Request, Form(form): Form<CreateInKindForm>) -> Response {
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

        if form.donor_name.trim().is_empty()
            || form.item.trim().is_empty()
            || form.quantity_text.trim().is_empty()
        {
            return Redirect::to("/kas/donasi-barang").into_response();
        }

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
        });

        let pool = match sqlx::PgPool::connect(&db_url).await {
            Ok(p) => p,
            Err(e) => return Html(format!("<h1>Kesalahan Database: {}</h1>", e)).into_response(),
        };

        let item_id = Uuid::new_v4();
        let res = sqlx::query(
            r#"
            INSERT INTO in_kind_donations (id, org_id, donor_name, item, quantity_text)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(item_id)
        .bind(org_id)
        .bind(form.donor_name.trim())
        .bind(form.item.trim())
        .bind(form.quantity_text.trim())
        .execute(&pool)
        .await;

        if let Err(e) = res {
            return Html(format!("<h1>Gagal Menyimpan Donasi Barang: {}</h1>", e)).into_response();
        }

        Redirect::to("/kas").into_response()
    }

    /// Pratinjau LPJ Resmi Format Kanfer 2 (GET /laporan/lpj/:year)
    pub async fn lpj_preview(req: Request, Path(year): Path<i32>) -> Response {
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

        let start_date = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();

        #[derive(sqlx::FromRow)]
        struct RawTx {
            kind: String,
            category: String,
            description: String,
            amount_idr: i64,
            occurred_on: NaiveDate,
            donor_name: Option<String>,
        }

        let txs: Vec<RawTx> = sqlx::query_as(
            r#"
            SELECT kind, category, description, amount_idr, occurred_on, donor_name
            FROM transactions
            WHERE org_id = $1 AND occurred_on >= $2 AND occurred_on <= $3
            ORDER BY occurred_on ASC, created_at ASC
            "#,
        )
        .bind(org_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let mut income_list: Vec<TxRowView> = Vec::new();
        let mut expense_list: Vec<TxRowView> = Vec::new();
        let mut total_income: i64 = 0;
        let mut total_expense: i64 = 0;

        for tx in txs {
            let cat_label = match tx.category.as_str() {
                "saldo_awal" => "Saldo Awal".to_string(),
                "iuran" => "Iuran Warga".to_string(),
                "iuran_panitia" => "Iuran Panitia".to_string(),
                "donasi" => "Donasi Warga".to_string(),
                "lelang" => "Hasil Lelang".to_string(),
                "konsumsi" => "Konsumsi".to_string(),
                "hadiah" => "Hadiah & Lomba".to_string(),
                "perlengkapan" => "Perlengkapan".to_string(),
                "operasional" => "Operasional".to_string(),
                other => other.to_string(),
            };

            let view_item = TxRowView {
                id: Uuid::nil(),
                date_formatted: tx.occurred_on.format("%d/%m/%Y").to_string(),
                kind: tx.kind.clone(),
                category: tx.category,
                category_label: cat_label,
                description: tx.description,
                donor_name: tx.donor_name,
                amount_idr: tx.amount_idr,
                amount_formatted: ViewService::format_rupiah(tx.amount_idr),
                running_balance_formatted: String::new(),
                source_payment_id: None,
            };

            if tx.kind == "masuk" {
                total_income += tx.amount_idr;
                income_list.push(view_item);
            } else {
                total_expense += tx.amount_idr;
                expense_list.push(view_item);
            }
        }

        let final_balance = total_income - total_expense;

        // Ambil donasi barang
        #[derive(sqlx::FromRow)]
        struct RawInKind {
            donor_name: String,
            item: String,
            quantity_text: String,
        }

        let inkind_rows: Vec<RawInKind> = sqlx::query_as(
            "SELECT donor_name, item, quantity_text FROM in_kind_donations WHERE org_id = $1 ORDER BY created_at ASC",
        )
        .bind(org_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let inkind_list: Vec<InKindItemView> = inkind_rows
            .into_iter()
            .map(|k| InKindItemView {
                id: Uuid::nil(),
                date_formatted: String::new(),
                donor_name: k.donor_name,
                item: k.item,
                quantity_text: k.quantity_text,
            })
            .collect();

        let print_date = Local::now().format("%d %B %Y").to_string();

        let mut ctx = BladeContext::new();
        ctx.insert("title", format!("LPJ Keuangan {} — {}", year, org_name));
        ctx.insert("area", "admin");
        ctx.insert("active", "kas");
        ctx.insert("org_name", &org_name);
        ctx.insert("role_label", auth.role_display());
        ctx.insert("selected_year", year);
        ctx.insert("print_date", &print_date);

        ctx.insert(
            "total_income_formatted",
            ViewService::format_rupiah(total_income),
        );
        ctx.insert(
            "total_expense_formatted",
            ViewService::format_rupiah(total_expense),
        );
        ctx.insert(
            "final_balance_formatted",
            ViewService::format_rupiah(final_balance),
        );

        ctx.insert("income_list", income_list);
        ctx.insert("expense_list", expense_list);
        ctx.insert("inkind_list", inkind_list);

        let html = ViewService::render_blade("cash/lpj_preview.blade.rs", &ctx);
        Html(html).into_response()
    }

    /// Unduh Berkas LPJ Word .docx Resmi (GET /laporan/lpj/:year/docx)
    pub async fn export_docx(req: Request, Path(year): Path<i32>) -> Response {
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

        let start_date = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();

        #[derive(sqlx::FromRow)]
        struct RawTx {
            kind: String,
            category: String,
            description: String,
            amount_idr: i64,
            occurred_on: NaiveDate,
            donor_name: Option<String>,
        }

        let txs: Vec<RawTx> = sqlx::query_as(
            r#"
            SELECT kind, category, description, amount_idr, occurred_on, donor_name
            FROM transactions
            WHERE org_id = $1 AND occurred_on >= $2 AND occurred_on <= $3
            ORDER BY occurred_on ASC, created_at ASC
            "#,
        )
        .bind(org_id)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        let mut income_rows: Vec<RawTx> = Vec::new();
        let mut expense_rows: Vec<RawTx> = Vec::new();
        let mut total_income: i64 = 0;
        let mut total_expense: i64 = 0;

        for tx in txs {
            if tx.kind == "masuk" {
                total_income += tx.amount_idr;
                income_rows.push(tx);
            } else {
                total_expense += tx.amount_idr;
                expense_rows.push(tx);
            }
        }

        let final_balance = total_income - total_expense;

        // Ambil donasi barang
        #[derive(sqlx::FromRow)]
        struct RawInKind {
            donor_name: String,
            item: String,
            quantity_text: String,
        }

        let inkind_rows: Vec<RawInKind> = sqlx::query_as(
            "SELECT donor_name, item, quantity_text FROM in_kind_donations WHERE org_id = $1 ORDER BY created_at ASC",
        )
        .bind(org_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

        // Bangun Dokumen DOCX menggunakan crate docx-rs
        let mut docx = Docx::new()
            .add_paragraph(
                Paragraph::new().align(AlignmentType::Center).add_run(
                    Run::new()
                        .size(30)
                        .bold()
                        .add_text("LAPORAN PERTANGGUNGJAWABAN (LPJ) KEUANGAN"),
                ),
            )
            .add_paragraph(
                Paragraph::new().align(AlignmentType::Center).add_run(
                    Run::new()
                        .size(26)
                        .bold()
                        .add_text(format!("{} — RW 09 SLAMARAN", org_name.to_uppercase())),
                ),
            )
            .add_paragraph(
                Paragraph::new().align(AlignmentType::Center).add_run(
                    Run::new()
                        .size(22)
                        .italic()
                        .add_text(format!("PERIODE TAHUN {}", year)),
                ),
            )
            .add_paragraph(Paragraph::new()) // Spasi
            .add_paragraph(
                Paragraph::new().add_run(
                    Run::new()
                        .bold()
                        .size(24)
                        .add_text("1. RINGKASAN FINANSIAL"),
                ),
            );

        // Tabel Ringkasan
        let summary_table = Table::new(vec![
            TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().bold().add_text("Pos Ringkasan")),
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().bold().add_text("Jumlah Rupiah")),
                ),
            ]),
            TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text("Total Pemasukan")),
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new()
                        .add_run(Run::new().add_text(ViewService::format_rupiah(total_income))),
                ),
            ]),
            TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text("Total Pengeluaran")),
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new()
                        .add_run(Run::new().add_text(ViewService::format_rupiah(total_expense))),
                ),
            ]),
            TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().bold().add_text("Sisa Saldo Kas Akhir")),
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new()
                            .bold()
                            .add_text(ViewService::format_rupiah(final_balance)),
                    ),
                ),
            ]),
        ]);
        docx = docx.add_table(summary_table);

        // Spasi & Bagian A: Pemasukan
        docx = docx.add_paragraph(Paragraph::new()).add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .bold()
                    .size(24)
                    .add_text("2. BAGIAN A: RINCIAN PEMASUKAN DANA"),
            ),
        );

        let mut income_tbl_rows = vec![TableRow::new(vec![
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().bold().add_text("No"))),
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().bold().add_text("Tanggal"))),
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().bold().add_text("Kategori"))),
            TableCell::new().add_paragraph(
                Paragraph::new().add_run(Run::new().bold().add_text("Donatur / Pihak")),
            ),
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().bold().add_text("Keterangan"))),
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().bold().add_text("Jumlah"))),
        ])];

        for (idx, in_tx) in income_rows.iter().enumerate() {
            income_tbl_rows.push(TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text(format!("{}", idx + 1))),
                ),
                TableCell::new().add_paragraph(Paragraph::new().add_run(
                    Run::new().add_text(in_tx.occurred_on.format("%d/%m/%Y").to_string()),
                )),
                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(Run::new().add_text(&in_tx.category))),
                TableCell::new().add_paragraph(
                    Paragraph::new()
                        .add_run(Run::new().add_text(in_tx.donor_name.as_deref().unwrap_or("-"))),
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text(&in_tx.description)),
                ),
                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(
                        Run::new().add_text(ViewService::format_rupiah(in_tx.amount_idr)),
                    )),
            ]));
        }

        docx = docx.add_table(Table::new(income_tbl_rows));

        // Spasi & Bagian B: Pengeluaran
        docx = docx.add_paragraph(Paragraph::new()).add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .bold()
                    .size(24)
                    .add_text("3. BAGIAN B: RINCIAN PENGELUARAN DANA"),
            ),
        );

        let mut expense_tbl_rows = vec![TableRow::new(vec![
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().bold().add_text("No"))),
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().bold().add_text("Tanggal"))),
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().bold().add_text("Kategori"))),
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().bold().add_text("Penerima"))),
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().bold().add_text("Keterangan"))),
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().bold().add_text("Jumlah"))),
        ])];

        for (idx, out_tx) in expense_rows.iter().enumerate() {
            expense_tbl_rows.push(TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text(format!("{}", idx + 1))),
                ),
                TableCell::new().add_paragraph(Paragraph::new().add_run(
                    Run::new().add_text(out_tx.occurred_on.format("%d/%m/%Y").to_string()),
                )),
                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(Run::new().add_text(&out_tx.category))),
                TableCell::new().add_paragraph(
                    Paragraph::new()
                        .add_run(Run::new().add_text(out_tx.donor_name.as_deref().unwrap_or("-"))),
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text(&out_tx.description)),
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(
                        Run::new().add_text(ViewService::format_rupiah(out_tx.amount_idr)),
                    ),
                ),
            ]));
        }

        docx = docx.add_table(Table::new(expense_tbl_rows));

        // Spasi & Bagian D: Donasi Barang
        docx = docx.add_paragraph(Paragraph::new()).add_paragraph(
            Paragraph::new().add_run(
                Run::new()
                    .bold()
                    .size(24)
                    .add_text("4. BAGIAN D: BANTUAN & DONASI BARANG (NATURA)"),
            ),
        );

        let mut inkind_tbl_rows = vec![TableRow::new(vec![
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().bold().add_text("No"))),
            TableCell::new().add_paragraph(
                Paragraph::new().add_run(Run::new().bold().add_text("Nama Donatur")),
            ),
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().bold().add_text("Nama Barang"))),
            TableCell::new()
                .add_paragraph(Paragraph::new().add_run(Run::new().bold().add_text("Jumlah"))),
        ])];

        for (idx, item) in inkind_rows.iter().enumerate() {
            inkind_tbl_rows.push(TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text(format!("{}", idx + 1))),
                ),
                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(Run::new().add_text(&item.donor_name))),
                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(Run::new().add_text(&item.item))),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text(&item.quantity_text)),
                ),
            ]));
        }

        docx = docx.add_table(Table::new(inkind_tbl_rows));

        // Spasi & Lembar Pengesahan
        docx = docx
            .add_paragraph(Paragraph::new())
            .add_paragraph(
                Paragraph::new()
                    .align(AlignmentType::Right)
                    .add_run(Run::new().add_text(format!("Pekalongan, {} Oktober {}", 24, year))),
            )
            .add_paragraph(Paragraph::new());

        let sig_table = Table::new(vec![
            TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new()
                        .align(AlignmentType::Center)
                        .add_run(Run::new().add_text("Ketua Paguyuban,")),
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new()
                        .align(AlignmentType::Center)
                        .add_run(Run::new().add_text("Sekretaris,")),
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new()
                        .align(AlignmentType::Center)
                        .add_run(Run::new().add_text("Bendahara,")),
                ),
            ]),
            TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new()
                        .align(AlignmentType::Center)
                        .add_run(Run::new().add_text("\n\n\n( Bambang Sudarsono )")),
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new()
                        .align(AlignmentType::Center)
                        .add_run(Run::new().add_text("\n\n\n( Ahmad Fauzi )")),
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new()
                        .align(AlignmentType::Center)
                        .add_run(Run::new().add_text("\n\n\n( Siti Rahayu )")),
                ),
            ]),
        ]);
        docx = docx.add_table(sig_table);

        let mut buffer = Cursor::new(Vec::new());
        if let Err(e) = docx.build().pack(&mut buffer) {
            return Html(format!("<h1>Gagal Membuat Berkas DOCX: {}</h1>", e)).into_response();
        }

        let docx_bytes = buffer.into_inner();
        let filename = format!("LPJ_Keuangan_{}_{}.docx", year, org_name.replace(' ', "_"));

        (
            [
                (
                    axum::http::header::CONTENT_TYPE,
                    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                ),
                (
                    axum::http::header::CONTENT_DISPOSITION,
                    &format!("attachment; filename=\"{}\"", filename),
                ),
            ],
            docx_bytes,
        )
            .into_response()
    }
}
