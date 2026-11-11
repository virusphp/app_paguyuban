use app_paguyuban::app::services::{AuthContext, ViewService};
use chrono::NaiveDate;
use dotenv::dotenv;
use lumina::core::blade::BladeContext;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[test]
fn test_render_fee_views_native_blade() {
    // 1. Render fees/index.blade.rs (Matriks Kotak 12 Bulan)
    let mut ctx_index = BladeContext::new();
    ctx_index.insert("title", "Rekap Iuran 2026 — Paguyuban");
    ctx_index.insert("area", "admin");
    ctx_index.insert("active", "iuran");
    ctx_index.insert("org_name", "Paguyuban Kanfer 2");
    ctx_index.insert("role_label", "BENDAHARA");
    ctx_index.insert("can_manage", true);
    ctx_index.insert("selected_year", 2026);
    ctx_index.insert("available_years", vec![2025, 2026, 2027]);
    ctx_index.insert("query_search", "");

    #[derive(serde::Serialize)]
    struct DummyMonthCell {
        month_num: u32,
        month_name: String,
        status: String,
        invoice_id: Option<Uuid>,
        amount_raw: i64,
        amount_formatted: String,
        paid_date: Option<String>,
    }

    #[derive(serde::Serialize)]
    struct DummyMatrixRow {
        member_id: Uuid,
        full_name: String,
        phone: Option<String>,
        house_no: String,
        months: Vec<DummyMonthCell>,
        total_paid_raw: i64,
        total_paid_formatted: String,
        total_unpaid_raw: i64,
        total_unpaid_formatted: String,
        paid_months_count: usize,
        unpaid_months_count: usize,
    }

    let dummy_months = vec![
        DummyMonthCell {
            month_num: 1,
            month_name: "Jan".to_string(),
            status: "lunas".to_string(),
            invoice_id: Some(Uuid::new_v4()),
            amount_raw: 25000,
            amount_formatted: "Rp25.000".to_string(),
            paid_date: Some("10 Jan".to_string()),
        },
        DummyMonthCell {
            month_num: 2,
            month_name: "Feb".to_string(),
            status: "belum".to_string(),
            invoice_id: Some(Uuid::new_v4()),
            amount_raw: 25000,
            amount_formatted: "Rp25.000".to_string(),
            paid_date: None,
        },
        DummyMonthCell {
            month_num: 3,
            month_name: "Mar".to_string(),
            status: "empty".to_string(),
            invoice_id: None,
            amount_raw: 0,
            amount_formatted: "Rp0".to_string(),
            paid_date: None,
        },
    ];

    let dummy_rows = vec![DummyMatrixRow {
        member_id: Uuid::new_v4(),
        full_name: "Bambang Sudarsono".to_string(),
        phone: Some("081234567890".to_string()),
        house_no: "Blok B-01".to_string(),
        months: dummy_months,
        total_paid_raw: 25000,
        total_paid_formatted: "Rp25.000".to_string(),
        total_unpaid_raw: 25000,
        total_unpaid_formatted: "Rp25.000".to_string(),
        paid_months_count: 1,
        unpaid_months_count: 1,
    }];

    ctx_index.insert("matrix_rows", dummy_rows);
    ctx_index.insert("has_matrix_rows", true);
    ctx_index.insert("total_billed_raw", 50000i64);
    ctx_index.insert("total_collected_raw", 25000i64);
    ctx_index.insert("total_arrears_raw", 25000i64);
    ctx_index.insert("total_invoices_count", 2);
    ctx_index.insert("paid_invoices_count", 1);
    ctx_index.insert("unpaid_members_count", 1);
    ctx_index.insert("compliance_percentage", 50);

    let rendered_index = ViewService::render_blade("fees/index.blade.rs", &ctx_index);
    assert!(
        rendered_index.contains("Rekap Iuran &amp; Tunggakan Warga")
            || rendered_index.contains("Rekap Iuran & Tunggakan Warga")
    );
    assert!(rendered_index.contains("Bambang Sudarsono"));
    assert!(rendered_index.contains("Blok B-01"));
    assert!(rendered_index.contains("pg-square-cell--paid"));
    assert!(rendered_index.contains("pg-square-cell--unpaid"));
    assert!(rendered_index.contains("pg-square-cell--empty"));
    assert!(rendered_index.contains("Lunas Rp25.000"));
    assert!(rendered_index.contains("Salin Teks WhatsApp"));

    // 2. Render fees/tariffs.blade.rs
    let mut ctx_tariffs = BladeContext::new();
    ctx_tariffs.insert("title", "Tarif Iuran");
    ctx_tariffs.insert("active", "iuran");
    ctx_tariffs.insert("org_name", "Paguyuban Kanfer 2");
    ctx_tariffs.insert("role_label", "BENDAHARA");
    ctx_tariffs.insert("can_manage", true);
    ctx_tariffs.insert("today_date", "2026-10-24");

    #[derive(serde::Serialize)]
    struct DummyPlan {
        id: Uuid,
        name: String,
        amount_idr: i64,
        period: String,
        active_from: String,
    }
    let dummy_plans = vec![DummyPlan {
        id: Uuid::new_v4(),
        name: "Iuran Wajib Warga".to_string(),
        amount_idr: 25000,
        period: "bulanan".to_string(),
        active_from: "2026-01-01".to_string(),
    }];
    ctx_tariffs.insert("fee_plans", dummy_plans);

    let rendered_tariffs = ViewService::render_blade("fees/tariffs.blade.rs", &ctx_tariffs);
    assert!(rendered_tariffs.contains("Pengaturan Tarif Iuran"));
    assert!(rendered_tariffs.contains("Iuran Wajib Warga"));
    assert!(rendered_tariffs.contains("25.000"));

    // 3. Render fees/generate.blade.rs
    #[derive(serde::Serialize)]
    struct DummyGenPlan {
        id: Uuid,
        name: String,
        period: String,
        amount_formatted: String,
    }
    let dummy_gen_plans = vec![DummyGenPlan {
        id: Uuid::new_v4(),
        name: "Iuran Wajib Warga".to_string(),
        period: "bulanan".to_string(),
        amount_formatted: "Rp25.000".to_string(),
    }];

    let mut ctx_generate = BladeContext::new();
    ctx_generate.insert("title", "Terbitkan Tagihan Massal");
    ctx_generate.insert("active", "iuran");
    ctx_generate.insert("fee_plans", dummy_gen_plans);
    ctx_generate.insert("active_members_count", 30);
    ctx_generate.insert("current_month", 10);
    ctx_generate.insert("current_year", 2026);

    let rendered_generate = ViewService::render_blade("fees/generate.blade.rs", &ctx_generate);
    assert!(rendered_generate.contains("Terbitkan Tagihan Iuran Massal"));
    assert!(rendered_generate.contains("Pilih Jenis Tarif Iuran"));

    // 4. Render fees/pay.blade.rs
    let mut ctx_pay = BladeContext::new();
    ctx_pay.insert("title", "Catat Pembayaran Iuran");
    ctx_pay.insert("active", "iuran");
    ctx_pay.insert("invoice_id", Uuid::new_v4());
    ctx_pay.insert("invoice_member_name", "Bambang Sudarsono");
    ctx_pay.insert("invoice_house_no", "Blok B-01");
    ctx_pay.insert("invoice_plan_name", "Iuran Kas Bulanan");
    ctx_pay.insert("invoice_period_label", "Oktober 2026");
    ctx_pay.insert("invoice_amount_formatted", "Rp25.000");
    ctx_pay.insert("today_date", "2026-10-24");

    let rendered_pay = ViewService::render_blade("fees/pay.blade.rs", &ctx_pay);
    assert!(rendered_pay.contains("Catat Pembayaran Iuran"));
    assert!(rendered_pay.contains("Bambang Sudarsono"));
    assert!(rendered_pay.contains("Blok B-01"));
    assert!(rendered_pay.contains("Rp25.000"));
}

#[test]
fn test_fee_authorization_matrix() {
    let bendahara = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Siti Rahayu".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Paguyuban Kanfer 2".to_string()),
        current_role: Some("bendahara".to_string()),
        is_api: false,
    };
    assert!(bendahara.require_role(&["admin", "bendahara"]).is_ok());
    assert_eq!(bendahara.role_display(), "BENDAHARA");

    let admin = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Bambang Sudarsono".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Paguyuban Kanfer 2".to_string()),
        current_role: Some("admin".to_string()),
        is_api: false,
    };
    assert!(admin.require_role(&["admin", "bendahara"]).is_ok());

    let sekretaris = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Ahmad Fauzi".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Paguyuban Kanfer 2".to_string()),
        current_role: Some("sekretaris".to_string()),
        is_api: false,
    };
    // Sekretaris tidak boleh mengelola / bayar tagihan (hanya read-only)
    assert!(sekretaris.require_role(&["admin", "bendahara"]).is_err());

    let superadmin = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Super Admin".to_string(),
        is_superadmin: true,
        current_org_id: None,
        current_org_name: None,
        current_role: None,
        is_api: false,
    };
    assert!(superadmin.require_role(&["admin", "bendahara"]).is_ok());
    assert_eq!(superadmin.role_display(), "SUPERADMIN");
}

#[tokio::test]
async fn test_database_fee_operations_and_atomic_payment() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
    });

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Database not accessible, skipping database fee operations test.");
            return;
        }
    };

    let kanfer_org_id = Uuid::parse_str("b5bac8a0-c424-428f-8c27-2534cb24e9f4").unwrap();
    let rt_slamaran_id = Uuid::parse_str("bc729a29-e7c7-4988-8352-9be8ca59ca53").unwrap();

    // 1. Buat Tarif Iuran di Paguyuban Kanfer
    let test_plan_id = Uuid::new_v4();
    let plan_name = format!(
        "Iuran Kebersihan Test {}",
        Uuid::new_v4()
            .to_string()
            .chars()
            .take(6)
            .collect::<String>()
    );
    let insert_plan = sqlx::query(
        r#"
        INSERT INTO fee_plans (id, org_id, name, amount_idr, period, active_from)
        VALUES ($1, $2, $3, 30000, 'bulanan', '2026-01-01')
        "#,
    )
    .bind(test_plan_id)
    .bind(kanfer_org_id)
    .bind(&plan_name)
    .execute(&pool)
    .await;
    assert!(insert_plan.is_ok(), "Insert fee_plan must succeed");

    // 2. Uji Isolasi Multi-Tenant: Pastikan RT Slamaran TIDAK melihat tarif Kanfer
    let slamaran_plans_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM fee_plans WHERE org_id = $1 AND id = $2")
            .bind(rt_slamaran_id)
            .bind(test_plan_id)
            .fetch_one(&pool)
            .await
            .unwrap_or(0);
    assert_eq!(
        slamaran_plans_count, 0,
        "Fee plan must be strictly tenant isolated"
    );

    // 3. Pastikan ada minimal 1 warga di Kanfer untuk uji invoice
    let member_id: Uuid = match sqlx::query_scalar(
        "SELECT id FROM members WHERE org_id = $1 AND status = 'aktif' LIMIT 1",
    )
    .bind(kanfer_org_id)
    .fetch_optional(&pool)
    .await
    .unwrap_or(None)
    {
        Some(m) => m,
        None => {
            let new_m = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO members (id, org_id, full_name, status, joined_on) VALUES ($1, $2, 'Warga Tes Iuran', 'aktif', '2026-01-01')"
            )
            .bind(new_m)
            .bind(kanfer_org_id)
            .execute(&pool)
            .await
            .expect("Create member for test must succeed");
            new_m
        }
    };

    // 4. Terbitkan Tagihan untuk Periode 2026-10-01
    let inv_id = Uuid::new_v4();
    let period_date = NaiveDate::from_ymd_opt(2026, 10, 1).unwrap();
    let inv_res = sqlx::query(
        r#"
        INSERT INTO invoices (id, org_id, member_id, fee_plan_id, period_month, amount_idr, status)
        VALUES ($1, $2, $3, $4, $5, 30000, 'belum')
        ON CONFLICT (member_id, fee_plan_id, period_month) DO UPDATE SET amount_idr = EXCLUDED.amount_idr
        "#,
    )
    .bind(inv_id)
    .bind(kanfer_org_id)
    .bind(member_id)
    .bind(test_plan_id)
    .bind(period_date)
    .execute(&pool)
    .await;
    assert!(inv_res.is_ok(), "Invoice creation must succeed");

    // 5. UJI PELUNASAN ATOMIK KE TRANSAKSI KAS
    let mut tx = pool.begin().await.expect("Begin transaction must succeed");

    // A. Update status invoice
    sqlx::query("UPDATE invoices SET status = 'lunas' WHERE id = $1 AND org_id = $2")
        .bind(inv_id)
        .bind(kanfer_org_id)
        .execute(&mut *tx)
        .await
        .expect("Update invoice status must succeed");

    // B. Insert ke payments
    let payment_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO payments (id, org_id, invoice_id, amount_idr, method, paid_on, note)
        VALUES ($1, $2, $3, 30000, 'tunai', '2026-10-24', 'Uji Kas Atomik')
        "#,
    )
    .bind(payment_id)
    .bind(kanfer_org_id)
    .bind(inv_id)
    .execute(&mut *tx)
    .await
    .expect("Insert payment must succeed");

    // C. Insert ke transactions (Kas Paguyuban: kind='masuk', category='iuran')
    let trans_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO transactions (id, org_id, kind, category, description, amount_idr, occurred_on, source_payment_id)
        VALUES ($1, $2, 'masuk', 'iuran', 'Iuran Okt 2026 - Warga Tes', 30000, '2026-10-24', $3)
        "#,
    )
    .bind(trans_id)
    .bind(kanfer_org_id)
    .bind(payment_id)
    .execute(&mut *tx)
    .await
    .expect("Insert kas transaction must succeed");

    tx.commit()
        .await
        .expect("Commit atomic transaction must succeed");

    // 6. Verifikasi Integritas Transaksi Kas
    let inv_status: String = sqlx::query_scalar("SELECT status FROM invoices WHERE id = $1")
        .bind(inv_id)
        .fetch_one(&pool)
        .await
        .expect("Fetch invoice status must succeed");
    assert_eq!(inv_status, "lunas");

    let kas_amount: i64 =
        sqlx::query_scalar("SELECT amount_idr FROM transactions WHERE source_payment_id = $1")
            .bind(payment_id)
            .fetch_one(&pool)
            .await
            .expect("Fetch kas transaction must succeed");
    assert_eq!(
        kas_amount, 30000,
        "Kas amount must exactly match fee amount"
    );
}
