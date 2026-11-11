use axum::response::{Html, IntoResponse, Response};
use lumina::core::blade::BladeContext;
use lumina::prelude::Request;
use uuid::Uuid;

use crate::app::services::{AuthContext, ViewService};

pub struct DashboardController;

impl DashboardController {
    /// Halaman Utama Dashboard PC (GET /dashboard)
    pub async fn index(req: Request) -> Response {
        // 1. Guard autentikasi
        let auth = match AuthContext::from_request(&req).await {
            Ok(ctx) => ctx,
            Err(res) => return res,
        };

        let user_id = auth.user_id;
        let user_name = auth.user_name.clone();
        let is_superadmin = auth.is_superadmin;

        // Jika belum ada organisasi aktif yang dipilih, arahkan ke halaman pilih organisasi
        if auth.current_org_id.is_none() {
            return axum::response::Redirect::to("/pilih-organisasi").into_response();
        }

        let current_org_id_str = auth.current_org_id.map(|u| u.to_string());
        let current_org_name = auth
            .current_org_name
            .unwrap_or_else(|| "Paguyuban".to_string());
        let current_role = auth.current_role.unwrap_or_else(|| "Pengurus".to_string());

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

        // 2. Ambil daftar organisasi yang dimiliki pengguna untuk dropdown switcher
        // Jika Superadmin, ambil seluruh organisasi aktif di sistem
        #[derive(sqlx::FromRow, serde::Serialize)]
        struct OrgItem {
            id: String,
            name: String,
        }

        let user_orgs_raw: Vec<(Uuid, String)> = if is_superadmin {
            sqlx::query_as(
                r#"
                SELECT id, name
                FROM organizations
                WHERE status = 'aktif'
                ORDER BY name ASC;
                "#,
            )
            .fetch_all(&pool)
            .await
            .unwrap_or_default()
        } else {
            sqlx::query_as(
                r#"
                SELECT o.id, o.name
                FROM memberships m
                JOIN organizations o ON m.org_id = o.id
                WHERE m.user_id = $1 AND o.status = 'aktif'
                ORDER BY o.name ASC;
                "#,
            )
            .bind(user_id)
            .fetch_all(&pool)
            .await
            .unwrap_or_default()
        };

        let user_orgs: Vec<OrgItem> = user_orgs_raw
            .into_iter()
            .map(|(id, name)| OrgItem {
                id: id.to_string(),
                name,
            })
            .collect();

        // 3. Query data statistik jika ada organisasi aktif
        let mut total_members: i64 = 0;
        let mut total_households: i64 = 0;
        let mut current_balance: i64 = 0;
        let mut fee_collected: i64 = 0;
        let mut total_arrears: i64 = 0;
        let mut arrears_count: i64 = 0;

        #[derive(serde::Serialize)]
        struct TxView {
            date: String,
            description: String,
            category: String,
            kind: String,
            amount: String,
            tone: String,
        }

        #[derive(serde::Serialize)]
        struct EventSummary {
            id: String,
            name: String,
            date_time: String,
            location: String,
        }

        let mut recent_transactions: Vec<TxView> = Vec::new();
        let mut upcoming_events: Vec<EventSummary> = Vec::new();

        if let Some(org_id_str) = &current_org_id_str {
            if let Ok(org_uuid) = Uuid::parse_str(org_id_str) {
                // Total anggota
                total_members =
                    sqlx::query_scalar("SELECT COUNT(*) FROM members WHERE org_id = $1")
                        .bind(org_uuid)
                        .fetch_one(&pool)
                        .await
                        .unwrap_or(0);

                // Total rumah / KK
                total_households =
                    sqlx::query_scalar("SELECT COUNT(*) FROM households WHERE org_id = $1")
                        .bind(org_uuid)
                        .fetch_one(&pool)
                        .await
                        .unwrap_or(0);

                // Saldo kas saat ini
                current_balance = sqlx::query_scalar(
                    r#"
                    SELECT COALESCE(SUM(CASE WHEN kind = 'masuk' THEN amount_idr ELSE -amount_idr END), 0)
                    FROM transactions WHERE org_id = $1;
                    "#,
                )
                .bind(org_uuid)
                .fetch_one(&pool)
                .await
                .unwrap_or(0);

                // Iuran terkumpul
                fee_collected = sqlx::query_scalar(
                    "SELECT COALESCE(SUM(amount_idr), 0) FROM payments WHERE org_id = $1",
                )
                .bind(org_uuid)
                .fetch_one(&pool)
                .await
                .unwrap_or(0);

                // Tunggakan
                total_arrears = sqlx::query_scalar(
                    "SELECT COALESCE(SUM(amount_idr), 0) FROM invoices WHERE org_id = $1 AND status = 'belum'",
                )
                .bind(org_uuid)
                .fetch_one(&pool)
                .await
                .unwrap_or(0);

                arrears_count = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM invoices WHERE org_id = $1 AND status = 'belum'",
                )
                .bind(org_uuid)
                .fetch_one(&pool)
                .await
                .unwrap_or(0);

                // Transaksi terakhir
                #[derive(sqlx::FromRow)]
                struct TxRow {
                    occurred_on: chrono::NaiveDate,
                    description: String,
                    category: String,
                    kind: String,
                    amount_idr: i64,
                }

                let rows: Vec<TxRow> = sqlx::query_as(
                    r#"
                    SELECT occurred_on, description, category, kind, amount_idr
                    FROM transactions
                    WHERE org_id = $1
                    ORDER BY occurred_on DESC, created_at DESC
                    LIMIT 5;
                    "#,
                )
                .bind(org_uuid)
                .fetch_all(&pool)
                .await
                .unwrap_or_default();

                for r in rows {
                    let tone = if r.kind == "masuk" { "ok" } else { "bad" };
                    recent_transactions.push(TxView {
                        date: r.occurred_on.format("%d %b %Y").to_string(),
                        description: r.description,
                        category: r.category.replace('_', " ").to_uppercase(),
                        kind: r.kind,
                        amount: ViewService::format_rupiah(r.amount_idr).replace("Rp", ""),
                        tone: tone.to_string(),
                    });
                }

                // Agenda kegiatan mendatang (maksimal 3)
                #[derive(sqlx::FromRow)]
                struct EventRow {
                    id: Uuid,
                    name: String,
                    held_at: chrono::DateTime<chrono::Utc>,
                    location: String,
                }

                let ev_rows: Vec<EventRow> = sqlx::query_as(
                    r#"
                    SELECT id, name, held_at, location
                    FROM events
                    WHERE org_id = $1 AND status != 'batal' AND held_at >= CURRENT_TIMESTAMP - INTERVAL '1 day'
                    ORDER BY held_at ASC
                    LIMIT 3;
                    "#,
                )
                .bind(org_uuid)
                .fetch_all(&pool)
                .await
                .unwrap_or_default();

                let wib = chrono::FixedOffset::east_opt(7 * 3600).unwrap();
                for e in ev_rows {
                    upcoming_events.push(EventSummary {
                        id: e.id.to_string(),
                        name: e.name,
                        date_time: e
                            .held_at
                            .with_timezone(&wib)
                            .format("%d %b %Y, %H:%M WIB")
                            .to_string(),
                        location: e.location,
                    });
                }
            }
        }

        // 4. Hitung persentase penarikan iuran
        let total_billable = fee_collected + total_arrears;
        let collection_percent = if total_billable > 0 {
            (fee_collected * 100) / total_billable
        } else {
            100
        };

        let can_manage = matches!(
            current_role.to_lowercase().as_str(),
            "admin" | "bendahara" | "sekretaris" | "superadmin"
        );

        // 5. Susun Context Blade Native
        let mut ctx = BladeContext::new();
        ctx.insert("title", format!("Dashboard — {}", current_org_name));
        ctx.insert("active", "dashboard");
        ctx.insert("area", "admin");
        ctx.insert("user_name", &user_name);
        ctx.insert("org_name", &current_org_name);
        ctx.insert("current_org_id", current_org_id_str.unwrap_or_default());
        ctx.insert("role_label", current_role.to_uppercase());
        ctx.insert("can_manage", can_manage);
        ctx.insert("is_superadmin", is_superadmin);
        ctx.insert("user_orgs", user_orgs);

        ctx.insert("total_members", total_members);
        ctx.insert("total_households", total_households);
        ctx.insert("current_balance", current_balance);
        ctx.insert("fee_collected", fee_collected);
        ctx.insert("total_arrears", total_arrears);
        ctx.insert("arrears_count", arrears_count);
        ctx.insert("collection_percent", collection_percent);
        ctx.insert("recent_transactions", recent_transactions);
        ctx.insert("upcoming_events", upcoming_events);

        let html = ViewService::render_blade("dashboard/index.blade.rs", &ctx);
        Html(html).into_response()
    }
}
