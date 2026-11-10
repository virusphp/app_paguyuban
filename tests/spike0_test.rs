use dotenv::dotenv;

#[test]
fn test_auth_user_and_session_contract() {
    use lumina::prelude::Request;

    // Verifikasi kontrak Request.user dan Request.session
    fn verify_auth_types(req: &Request) {
        if let Some(user) = &req.user {
            assert!(!user.sub.is_empty());
            assert!(!user.role.is_empty());
        }
        let _ = &req.session;
    }

    let _ = verify_auth_types;
    println!("AuthUser & Session contract test: SUCCESS!");
}

#[test]
fn test_native_blade_engine_rendering() {
    use lumina::core::blade::{BladeContext, BladeEngine};

    let engine = BladeEngine::new();
    let mut ctx = BladeContext::new();
    ctx.insert("org_name", "Paguyuban Kanfer 2");
    ctx.insert("area", "admin");
    ctx.insert("active", "kas");
    ctx.insert("role_label", "Bendahara");

    let rendered = engine.render("dashboard/index.blade.rs", &ctx);
    assert!(rendered.contains("Paguyuban Kanfer 2"));
    assert!(rendered.contains("is-active"));
    println!("Native Blade templates test: SUCCESS!");
}

#[test]
fn test_spike0_export_docx_lpj() {
    use docx_rs::*;

    let path = std::path::Path::new("target/test_lpj_tirakatan.docx");
    let file = std::fs::File::create(path).expect("Failed to create file");

    let docx = Docx::new()
        .add_paragraph(
            Paragraph::new().align(AlignmentType::Center).add_run(
                Run::new()
                    .size(32)
                    .bold()
                    .add_text("LAPORAN PERTANGGUNGJAWABAN (LPJ)"),
            ),
        )
        .add_paragraph(
            Paragraph::new().align(AlignmentType::Center).add_run(
                Run::new()
                    .size(26)
                    .add_text("KEGIATAN TIRAKATAN & LOMBA HUT RI"),
            ),
        )
        .add_paragraph(
            Paragraph::new().align(AlignmentType::Center).add_run(
                Run::new()
                    .size(22)
                    .italic()
                    .add_text("PAGUYUBAN KANFER 2 - RW 09 SLAMARAN"),
            ),
        )
        .add_paragraph(Paragraph::new()) // Jarak kosong
        .add_paragraph(
            Paragraph::new().add_run(Run::new().bold().add_text("A. Ringkasan Keuangan")),
        )
        .add_table(Table::new(vec![
            TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().bold().add_text("Keterangan")),
                ),
                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(Run::new().bold().add_text("Jumlah"))),
            ]),
            TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text("Total Pemasukan")),
                ),
                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Rp8.450.000"))),
            ]),
            TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().add_text("Total Pengeluaran")),
                ),
                TableCell::new()
                    .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Rp6.120.000"))),
            ]),
            TableRow::new(vec![
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().bold().add_text("Sisa Saldo Kas")),
                ),
                TableCell::new().add_paragraph(
                    Paragraph::new().add_run(Run::new().bold().add_text("Rp2.330.000")),
                ),
            ]),
        ]));

    docx.build()
        .pack(file)
        .expect("Failed to build and write docx");
    assert!(path.exists());
    let metadata = std::fs::metadata(path).expect("Failed to read metadata");
    assert!(metadata.len() > 0);
    println!(
        "DOCX LPJ export test: SUCCESS! File size: {} bytes",
        metadata.len()
    );
}

#[test]
fn test_spike0_export_pdf_lpj() {
    use printpdf::*;

    let mut doc = PdfDocument::new("LPJ Paguyuban Kanfer 2");
    let page = PdfPage::new(Mm(210.0), Mm(297.0), vec![]);
    doc.pages.push(page);

    let output_path = "target/test_lpj_tirakatan.pdf";
    let bytes = doc.save(&PdfSaveOptions::default(), &mut Vec::new());
    std::fs::write(output_path, &bytes).expect("Failed to write PDF");

    assert!(std::path::Path::new(output_path).exists());
    assert!(!bytes.is_empty());
    println!(
        "PDF LPJ export test: SUCCESS! File size: {} bytes",
        bytes.len()
    );
}

#[tokio::test]
async fn test_postgres_spike0_crud_and_rls() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", db_url);

    let pool = sqlx::PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to PostgreSQL with sqlx::PgPool");

    // 1. Uji Schema Migration & DDL di PostgreSQL
    sqlx::query("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";")
        .execute(&pool)
        .await
        .expect("Failed to create extension uuid-ossp");
    sqlx::query("CREATE EXTENSION IF NOT EXISTS \"pgcrypto\";")
        .execute(&pool)
        .await
        .expect("Failed to create extension pgcrypto");

    // Buat tabel uji organizations dan invoices untuk membuktikan RLS dan CRUD
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS test_organizations (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create test_organizations table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS test_invoices (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            org_id UUID NOT NULL REFERENCES test_organizations(id) ON DELETE CASCADE,
            title TEXT NOT NULL,
            amount_idr BIGINT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
        );
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create test_invoices table");

    // Bersihkan data uji sebelumnya
    sqlx::query("DELETE FROM test_invoices;")
        .execute(&pool)
        .await
        .expect("Failed to clean test_invoices");
    sqlx::query("DELETE FROM test_organizations;")
        .execute(&pool)
        .await
        .expect("Failed to clean test_organizations");

    // 2. Uji CRUD (Create, Read, Update, Delete) dengan tipe UUID dan BIGINT rupiah
    let org_id: uuid::Uuid =
        sqlx::query_scalar("INSERT INTO test_organizations (name) VALUES ($1) RETURNING id")
            .bind("Paguyuban Kanfer 2")
            .fetch_one(&pool)
            .await
            .expect("Failed to insert organization");

    let invoice_id: uuid::Uuid = sqlx::query_scalar(
        "INSERT INTO test_invoices (org_id, title, amount_idr) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(org_id)
    .bind("Iuran Bulanan Oktober 2026")
    .bind(25000_i64)
    .fetch_one(&pool)
    .await
    .expect("Failed to insert invoice");

    // Read
    let (fetched_title, fetched_amount): (String, i64) =
        sqlx::query_as("SELECT title, amount_idr FROM test_invoices WHERE id = $1")
            .bind(invoice_id)
            .fetch_one(&pool)
            .await
            .expect("Failed to fetch invoice");

    assert_eq!(fetched_title, "Iuran Bulanan Oktober 2026");
    assert_eq!(fetched_amount, 25000);

    // Update
    sqlx::query("UPDATE test_invoices SET amount_idr = $1 WHERE id = $2")
        .bind(30000_i64)
        .bind(invoice_id)
        .execute(&pool)
        .await
        .expect("Failed to update invoice");

    let updated_amount: i64 =
        sqlx::query_scalar("SELECT amount_idr FROM test_invoices WHERE id = $1")
            .bind(invoice_id)
            .fetch_one(&pool)
            .await
            .expect("Failed to fetch updated invoice");
    assert_eq!(updated_amount, 30000);

    println!("CRUD test on PostgreSQL: SUCCESS!");

    // 3. Uji RLS (Row Level Security) dengan `SET LOCAL app.org_id = '...'`
    // Aktifkan RLS di test_invoices
    sqlx::query("ALTER TABLE test_invoices ENABLE ROW LEVEL SECURITY;")
        .execute(&pool)
        .await
        .expect("Failed to enable RLS");
    sqlx::query("DROP POLICY IF EXISTS org_isolation_policy ON test_invoices;")
        .execute(&pool)
        .await
        .expect("Failed to drop old policy");
    sqlx::query(
        "CREATE POLICY org_isolation_policy ON test_invoices USING (org_id = NULLIF(current_setting('app.org_id', true), '')::uuid);",
    )
    .execute(&pool)
    .await
    .expect("Failed to create RLS policy");

    // Buat organisasi kedua (Org B)
    let org_b_id: uuid::Uuid =
        sqlx::query_scalar("INSERT INTO test_organizations (name) VALUES ($1) RETURNING id")
            .bind("Paguyuban Kenari RW 05")
            .fetch_one(&pool)
            .await
            .expect("Failed to insert org B");

    let _invoice_b_id: uuid::Uuid = sqlx::query_scalar(
        "INSERT INTO test_invoices (org_id, title, amount_idr) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(org_b_id)
    .bind("Iuran Kas RW 05")
    .bind(50000_i64)
    .fetch_one(&pool)
    .await
    .expect("Failed to insert invoice for Org B");

    // Buat non-superuser role untuk menguji RLS
    // (karena superuser PostgreSQL bypass RLS secara default kecuali FORCE ROW LEVEL SECURITY)
    sqlx::query(
        r#"
        DO $$
        BEGIN
            IF NOT EXISTS (SELECT FROM pg_roles WHERE rolname = 'app_user') THEN
                CREATE ROLE app_user WITH LOGIN PASSWORD 'app_secret';
            END IF;
        END
        $$;
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to setup app_user role");

    sqlx::query("GRANT USAGE ON SCHEMA public TO app_user;")
        .execute(&pool)
        .await
        .expect("Failed to grant usage");
    sqlx::query("GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO app_user;")
        .execute(&pool)
        .await
        .expect("Failed to grant privileges");

    // Koneksi sebagai app_user (non-owner)
    let app_user_url = format!(
        "postgres://app_user:app_secret@{}:{}/db_paguyuban",
        std::env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        std::env::var("DB_PORT").unwrap_or_else(|_| "54322".to_string()),
    );
    let app_pool = sqlx::PgPool::connect(&app_user_url)
        .await
        .expect("Failed to connect as app_user");

    // Test Transaksi untuk Org A:
    // BEGIN; SET LOCAL app.org_id = '<org_id>'; SELECT COUNT(*) ...; COMMIT;
    let mut tx_a = app_pool
        .begin()
        .await
        .expect("Failed to begin transaction A");
    sqlx::query(&format!("SET LOCAL app.org_id = '{}'", org_id))
        .execute(&mut *tx_a)
        .await
        .expect("Failed to set app.org_id for Org A");

    let count_a: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM test_invoices")
        .fetch_one(&mut *tx_a)
        .await
        .expect("Failed to query invoices in Org A tx");
    tx_a.commit().await.expect("Failed to commit tx A");

    // Harus HANYA melihat invoice dari Org A (count = 1), BUKAN invoice Org B!
    assert_eq!(count_a, 1, "Org A must only see its own invoices via RLS!");

    // Test Transaksi untuk Org B:
    let mut tx_b = app_pool
        .begin()
        .await
        .expect("Failed to begin transaction B");
    sqlx::query(&format!("SET LOCAL app.org_id = '{}'", org_b_id))
        .execute(&mut *tx_b)
        .await
        .expect("Failed to set app.org_id for Org B");

    let count_b: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM test_invoices")
        .fetch_one(&mut *tx_b)
        .await
        .expect("Failed to query invoices in Org B tx");
    tx_b.commit().await.expect("Failed to commit tx B");

    // Harus HANYA melihat invoice dari Org B (count = 1)
    assert_eq!(count_b, 1, "Org B must only see its own invoices via RLS!");

    // Test tanpa `app.org_id` (atau app.org_id kosong)
    let mut tx_none = app_pool
        .begin()
        .await
        .expect("Failed to begin transaction none");
    let count_none: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM test_invoices")
        .fetch_one(&mut *tx_none)
        .await
        .expect("Failed to query invoices without org_id");
    tx_none.commit().await.expect("Failed to commit tx none");

    assert_eq!(
        count_none, 0,
        "Without org_id set, RLS must block all rows!"
    );

    println!("RLS multi-tenant isolation test on PostgreSQL: SUCCESS!");
}
