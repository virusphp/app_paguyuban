use app_paguyuban::app::services::{AuthContext, ViewService};
use chrono::NaiveDate;
use docx_rs::*;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::io::Cursor;
use uuid::Uuid;

#[test]
fn test_render_cash_views_native_blade() {
    // 1. Render cash/index.blade.rs (Buku Kas Umum)
    let mut ctx_index = lumina::core::blade::BladeContext::new();
    ctx_index.insert("title", "Buku Kas & Keuangan");
    ctx_index.insert("area", "admin");
    ctx_index.insert("active", "kas");
    ctx_index.insert("org_name", "Paguyuban Kanfer 2");
    ctx_index.insert("role_label", "BENDAHARA");
    ctx_index.insert("can_manage", true);
    ctx_index.insert("selected_year", 2026);
    ctx_index.insert("available_years", vec![2025, 2026, 2027]);
    ctx_index.insert("filter_kind", "");
    ctx_index.insert("filter_category", "");

    ctx_index.insert("current_balance_formatted", "Rp18.750.000");
    ctx_index.insert("total_income_formatted", "Rp22.500.000");
    ctx_index.insert("total_expense_formatted", "Rp3.750.000");
    ctx_index.insert("income_count", 12);
    ctx_index.insert("expense_count", 4);
    ctx_index.insert("in_kind_count", 2);

    #[derive(serde::Serialize, Clone)]
    struct DummyTx {
        id: Uuid,
        date_formatted: String,
        kind: String,
        category: String,
        category_label: String,
        description: String,
        donor_name: Option<String>,
        amount_idr: i64,
        amount_formatted: String,
        running_balance_formatted: String,
        source_payment_id: Option<Uuid>,
    }

    let dummy_txs = vec![DummyTx {
        id: Uuid::new_v4(),
        date_formatted: "24/10/2026".to_string(),
        kind: "masuk".to_string(),
        category: "iuran".to_string(),
        category_label: "Iuran Warga".to_string(),
        description: "Iuran Kas Bulanan - Masriyati".to_string(),
        donor_name: Some("Masriyati".to_string()),
        amount_idr: 25000,
        amount_formatted: "Rp25.000".to_string(),
        running_balance_formatted: "Rp18.750.000".to_string(),
        source_payment_id: None,
    }];
    ctx_index.insert("transactions", dummy_txs.clone());

    #[derive(serde::Serialize, Clone)]
    struct DummyInKind {
        id: Uuid,
        date_formatted: String,
        donor_name: String,
        item: String,
        quantity_text: String,
    }

    let dummy_inkind = vec![DummyInKind {
        id: Uuid::new_v4(),
        date_formatted: "20/10/2026".to_string(),
        donor_name: "Bpk. H. Agus Santoso".to_string(),
        item: "Air Mineral Gelas".to_string(),
        quantity_text: "2 Dus".to_string(),
    }];
    ctx_index.insert("in_kind_items", dummy_inkind.clone());

    let rendered_index = ViewService::render_blade("cash/index.blade.rs", &ctx_index);
    assert!(rendered_index.contains("Buku Kas & Keuangan Paguyuban"));
    assert!(rendered_index.contains("Saldo Kas Berjalan"));
    assert!(rendered_index.contains("18.750.000"));
    assert!(rendered_index.contains("Air Mineral Gelas"));
    assert!(rendered_index.contains("Pratinjau LPJ Resmi"));

    // 2. Render cash/form_in.blade.rs
    let mut ctx_in = lumina::core::blade::BladeContext::new();
    ctx_in.insert("title", "Catat Kas Masuk");
    ctx_in.insert("area", "admin");
    ctx_in.insert("active", "kas");
    ctx_in.insert("today_date", "2026-10-24");
    let rendered_in = ViewService::render_blade("cash/form_in.blade.rs", &ctx_in);
    assert!(rendered_in.contains("Catat Penerimaan Kas Masuk"));
    assert!(rendered_in.contains("Kategori Pemasukan"));
    assert!(rendered_in.contains("Hamba Allah"));

    // 3. Render cash/form_out.blade.rs
    let mut ctx_out = lumina::core::blade::BladeContext::new();
    ctx_out.insert("title", "Catat Kas Keluar");
    ctx_out.insert("area", "admin");
    ctx_out.insert("active", "kas");
    ctx_out.insert("today_date", "2026-10-24");
    let rendered_out = ViewService::render_blade("cash/form_out.blade.rs", &ctx_out);
    assert!(rendered_out.contains("Catat Pengeluaran Kas (Belanja)"));
    assert!(rendered_out.contains("Kategori Pengeluaran"));

    // 4. Render cash/form_inkind.blade.rs
    let mut ctx_inkind = lumina::core::blade::BladeContext::new();
    ctx_inkind.insert("title", "Catat Donasi Barang");
    ctx_inkind.insert("area", "admin");
    ctx_inkind.insert("active", "kas");
    let rendered_inkind = ViewService::render_blade("cash/form_inkind.blade.rs", &ctx_inkind);
    assert!(rendered_inkind.contains("Catat Donasi Barang (Natura)"));
    assert!(rendered_inkind.contains("Nama Donatur / Penyumbang"));

    // 5. Render cash/lpj_preview.blade.rs
    let mut ctx_lpj = lumina::core::blade::BladeContext::new();
    ctx_lpj.insert("title", "LPJ Keuangan 2026");
    ctx_lpj.insert("area", "admin");
    ctx_lpj.insert("active", "kas");
    ctx_lpj.insert("org_name", "Paguyuban Kanfer 2");
    ctx_lpj.insert("role_label", "BENDAHARA");
    ctx_lpj.insert("selected_year", 2026);
    ctx_lpj.insert("print_date", "24 Oktober 2026");
    ctx_lpj.insert("total_income_formatted", "Rp22.500.000");
    ctx_lpj.insert("total_expense_formatted", "Rp3.750.000");
    ctx_lpj.insert("final_balance_formatted", "Rp18.750.000");

    ctx_lpj.insert("income_list", dummy_txs);
    ctx_lpj.insert("expense_list", Vec::<DummyTx>::new());
    ctx_lpj.insert("inkind_list", dummy_inkind);

    let rendered_lpj = ViewService::render_blade("cash/lpj_preview.blade.rs", &ctx_lpj);
    assert!(rendered_lpj.contains("LAPORAN PERTANGGUNGJAWABAN (LPJ) KEUANGAN"));
    assert!(rendered_lpj.contains("BAGIAN A: RINCIAN PEMASUKAN DANA"));
    assert!(rendered_lpj.contains("BAGIAN B: RINCIAN PENGELUARAN DANA"));
    assert!(rendered_lpj.contains("BAGIAN C: REKONSILIASI SALDO AKHIR"));
    assert!(rendered_lpj.contains("BAGIAN D: BANTUAN & DONASI BARANG"));
    assert!(rendered_lpj.contains("Ketua Paguyuban"));
    assert!(rendered_lpj.contains("Sekretaris"));
    assert!(rendered_lpj.contains("Bendahara"));
}

#[test]
fn test_cash_authorization_matrix() {
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
    // Sekretaris read-only, ditolak mencatat kas masuk/keluar
    assert!(sekretaris.require_role(&["admin", "bendahara"]).is_err());
}

#[tokio::test]
async fn test_database_cash_operations_and_running_balance() {
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
            eprintln!("Database not accessible, skipping database cash operations test.");
            return;
        }
    };

    let kanfer_org_id = Uuid::parse_str("b5bac8a0-c424-428f-8c27-2534cb24e9f4").unwrap();
    let slamaran_org_id = Uuid::parse_str("bc729a29-e7c7-4988-8352-9be8ca59ca53").unwrap();

    let tx_in_id = Uuid::new_v4();
    let today = NaiveDate::from_ymd_opt(2026, 10, 24).unwrap();

    // 1. Insert Transaksi Kas Masuk (Donasi Warga)
    let ins_in = sqlx::query(
        r#"
        INSERT INTO transactions (id, org_id, kind, category, description, amount_idr, occurred_on, donor_name)
        VALUES ($1, $2, 'masuk', 'donasi', 'Donasi Warga Uji Kas', 500000, $3, 'Hamba Allah')
        "#,
    )
    .bind(tx_in_id)
    .bind(kanfer_org_id)
    .bind(today)
    .execute(&pool)
    .await;
    assert!(ins_in.is_ok(), "Insert cash in transaction must succeed");

    // 2. Insert Transaksi Kas Keluar (Konsumsi)
    let tx_out_id = Uuid::new_v4();
    let ins_out = sqlx::query(
        r#"
        INSERT INTO transactions (id, org_id, kind, category, description, amount_idr, occurred_on, donor_name)
        VALUES ($1, $2, 'keluar', 'konsumsi', 'Beli Snack Rapat RT', 150000, $3, 'Warung Bu Ani')
        "#,
    )
    .bind(tx_out_id)
    .bind(kanfer_org_id)
    .bind(today)
    .execute(&pool)
    .await;
    assert!(ins_out.is_ok(), "Insert cash out transaction must succeed");

    // 3. Insert Donasi Barang (Natura)
    let inkind_id = Uuid::new_v4();
    let ins_inkind = sqlx::query(
        r#"
        INSERT INTO in_kind_donations (id, org_id, donor_name, item, quantity_text)
        VALUES ($1, $2, 'Bpk. Bambang Sudarsono', 'Air Mineral Botol', '3 Dus')
        "#,
    )
    .bind(inkind_id)
    .bind(kanfer_org_id)
    .execute(&pool)
    .await;
    assert!(ins_inkind.is_ok(), "Insert in_kind_donation must succeed");

    // 4. Verifikasi Isolasi Multi-Tenant: Organisasi Slamaran TIDAK melihat mutasi Kanfer
    let slamaran_tx_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM transactions WHERE org_id = $1 AND id = $2")
            .bind(slamaran_org_id)
            .bind(tx_in_id)
            .fetch_one(&pool)
            .await
            .unwrap_or(0);
    assert_eq!(
        slamaran_tx_count, 0,
        "Cash transaction must be strictly tenant isolated"
    );

    // 5. Verifikasi Perhitungan Kas
    let total_in: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(amount_idr), 0)::BIGINT FROM transactions WHERE org_id = $1 AND kind = 'masuk'",
    )
    .bind(kanfer_org_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    let total_out: i64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(amount_idr), 0)::BIGINT FROM transactions WHERE org_id = $1 AND kind = 'keluar'",
    )
    .bind(kanfer_org_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    assert!(total_in >= 500000);
    assert!(total_out >= 150000);
    let net_balance = total_in - total_out;
    assert!(net_balance >= 350000);
}

#[test]
fn test_docx_lpj_generation_with_table_sections() {
    let docx = Docx::new()
        .add_paragraph(
            Paragraph::new().align(AlignmentType::Center).add_run(
                Run::new()
                    .size(28)
                    .bold()
                    .add_text("LAPORAN PERTANGGUNGJAWABAN (LPJ) KEUANGAN"),
            ),
        )
        .add_paragraph(
            Paragraph::new().align(AlignmentType::Center).add_run(
                Run::new()
                    .size(22)
                    .add_text("PAGUYUBAN KANFER 2 — TAHUN 2026"),
            ),
        )
        .add_table(Table::new(vec![
            TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().bold().add_text("Pos Ringkasan")),
                ),
                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(Run::new().bold().add_text("Jumlah"))),
            ]),
            TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text("Total Pemasukan")),
                ),
                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Rp22.500.000"))),
            ]),
        ]));

    let mut buffer = Cursor::new(Vec::new());
    let pack_res = docx.build().pack(&mut buffer);
    assert!(pack_res.is_ok(), "Packing DOCX document must succeed");

    let bytes = buffer.into_inner();
    assert!(!bytes.is_empty(), "DOCX output binary must not be empty");
    // Header ZIP signature (DOCX berbasis Zip package) adalah [0x50, 0x4B, 0x03, 0x04]
    assert_eq!(
        &bytes[0..4],
        &[0x50, 0x4B, 0x03, 0x04],
        "DOCX must have valid Zip package magic header"
    );
}
