use app_paguyuban::app::services::ViewService;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[test]
fn test_currency_formatting() {
    assert_eq!(ViewService::format_rupiah(0), "Rp0");
    assert_eq!(ViewService::format_rupiah(20000), "Rp20.000");
    assert_eq!(ViewService::format_rupiah(1250000), "Rp1.250.000");
    assert_eq!(ViewService::format_rupiah(15000000), "Rp15.000.000");
    assert_eq!(ViewService::format_rupiah(-50000), "-Rp50.000");
}

#[tokio::test]
async fn test_f1_1_auth_membership_and_audit_log_flow() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
    });

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    // 1. Verifikasi kredensial login dev via bcrypt
    #[derive(sqlx::FromRow)]
    struct UserAuth {
        id: Uuid,
        name: String,
        password_hash: String,
        is_superadmin: bool,
    }

    let admin: UserAuth =
        sqlx::query_as("SELECT id, name, password_hash, is_superadmin FROM users WHERE phone = $1")
            .bind("081234567890")
            .fetch_one(&pool)
            .await
            .expect("Seeded admin must exist");

    assert_eq!(admin.name, "Bambang Sudarsono (Ketua)");
    assert!(!admin.is_superadmin);
    assert!(
        bcrypt::verify("password123", &admin.password_hash).unwrap_or(false),
        "Password verification must succeed"
    );

    // 2. Verifikasi membership di Paguyuban Kanfer 2
    let (org_name, role): (String, String) = sqlx::query_as(
        r#"
        SELECT o.name, m.role::text
        FROM memberships m
        JOIN organizations o ON m.org_id = o.id
        WHERE m.user_id = $1;
        "#,
    )
    .bind(admin.id)
    .fetch_one(&pool)
    .await
    .expect("Membership must exist");

    assert_eq!(org_name, "Paguyuban Kanfer 2");
    assert_eq!(role, "admin");

    // 3. Verifikasi pencatatan audit_logs
    sqlx::query(
        r#"
        INSERT INTO audit_logs (org_id, actor_user_id, action, entity, meta)
        VALUES (
            (SELECT id FROM organizations WHERE name = 'Paguyuban Kanfer 2'),
            $1,
            'TEST_ACTION',
            'dashboard',
            '{"status": "ok"}'::jsonb
        );
        "#,
    )
    .bind(admin.id)
    .execute(&pool)
    .await
    .expect("Audit log insertion must succeed");

    let audit_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM audit_logs WHERE actor_user_id = $1 AND action = 'TEST_ACTION'",
    )
    .bind(admin.id)
    .fetch_one(&pool)
    .await
    .expect("Audit count failed");

    assert!(audit_count >= 1, "Audit log record must be queryable");
}

#[tokio::test]
async fn test_f1_1_multi_tenant_isolation_between_organizations() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
    });

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    let kanfer_id: Uuid =
        sqlx::query_scalar("SELECT id FROM organizations WHERE name = 'Paguyuban Kanfer 2'")
            .fetch_one(&pool)
            .await
            .expect("Kanfer org must exist");

    let rt05_id: Uuid =
        sqlx::query_scalar("SELECT id FROM organizations WHERE name = 'RT 05 RW 09 Slamaran'")
            .fetch_one(&pool)
            .await
            .expect("RT05 org must exist");

    assert_ne!(kanfer_id, rt05_id, "Both orgs must have distinct UUIDs");

    // 1. Masukkan data rumah di masing-masing organisasi
    let house_kanfer_id = Uuid::new_v4();
    sqlx::query("INSERT INTO households (id, org_id, house_no, note) VALUES ($1, $2, 'K-99', 'Blok Kanfer')")
        .bind(house_kanfer_id)
        .bind(kanfer_id)
        .execute(&pool)
        .await
        .expect("Insert household Kanfer failed");

    let house_rt05_id = Uuid::new_v4();
    sqlx::query("INSERT INTO households (id, org_id, house_no, note) VALUES ($1, $2, 'RT-88', 'RT 05 Slamaran')")
        .bind(house_rt05_id)
        .bind(rt05_id)
        .execute(&pool)
        .await
        .expect("Insert household RT05 failed");

    // 2. Query data dengan batasan org_id Kanfer -> TIDAK BOLEH menemukan rumah RT-88
    let kanfer_houses: Vec<String> =
        sqlx::query_scalar("SELECT house_no FROM households WHERE org_id = $1")
            .bind(kanfer_id)
            .fetch_all(&pool)
            .await
            .expect("Query Kanfer houses failed");

    assert!(kanfer_houses.contains(&"K-99".to_string()));
    assert!(
        !kanfer_houses.contains(&"RT-88".to_string()),
        "ISOLATION BREACH: Kanfer saw RT05 house data!"
    );

    // 3. Query data dengan batasan org_id RT05 -> TIDAK BOLEH menemukan rumah K-99
    let rt05_houses: Vec<String> =
        sqlx::query_scalar("SELECT house_no FROM households WHERE org_id = $1")
            .bind(rt05_id)
            .fetch_all(&pool)
            .await
            .expect("Query RT05 houses failed");

    assert!(rt05_houses.contains(&"RT-88".to_string()));
    assert!(
        !rt05_houses.contains(&"K-99".to_string()),
        "ISOLATION BREACH: RT05 saw Kanfer house data!"
    );

    // Bersihkan data uji
    let _ = sqlx::query("DELETE FROM households WHERE id IN ($1, $2)")
        .bind(house_kanfer_id)
        .bind(house_rt05_id)
        .execute(&pool)
        .await;
}

#[test]
fn test_render_login_view_native_blade() {
    use lumina::core::blade::BladeContext;

    let mut ctx = BladeContext::new();
    ctx.insert("title", "Masuk — Aplikasi Manajemen Paguyuban");
    ctx.insert("csrf_token", "test-token-blade-999");
    ctx.insert("error", "Nomor HP atau kata sandi salah.");
    ctx.insert("old_phone", "08123456789");

    let rendered = ViewService::render_blade("auth/login.blade.rs", &ctx);
    println!("DEBUG NATIVE BLADE LOGIN RENDERED:\n{}", rendered);
    assert!(rendered.contains("Masuk Pengurus"), "Must render title");
    assert!(
        rendered.contains("name=\"csrf_token\" value=\"test-token-blade-999\""),
        "Must auto replace @csrf"
    );
    assert!(
        rendered.contains("Nomor HP atau kata sandi salah."),
        "Must render @if error"
    );
    assert!(rendered.contains("08123456789"), "Must render old_phone");
}

#[test]
fn test_render_dashboard_view_native_blade() {
    use lumina::core::blade::BladeContext;

    let mut ctx = BladeContext::new();
    ctx.insert("title", "Dashboard — Paguyuban Kanfer 2");
    ctx.insert("active", "dashboard");
    ctx.insert("area", "admin");
    ctx.insert("user_name", "Bambang Sudarsono");
    ctx.insert("org_name", "Paguyuban Kanfer 2");
    ctx.insert("role_label", "Ketua Paguyuban");
    ctx.insert("total_members", 65);
    ctx.insert("total_households", 30);
    ctx.insert("current_balance", 8450000);
    ctx.insert("fee_collected", 3200000);
    ctx.insert("total_arrears", 750000);
    ctx.insert("arrears_count", 3);
    ctx.insert("collection_percent", 81);

    let rendered = ViewService::render_blade("dashboard/index.blade.rs", &ctx);
    println!("DEBUG NATIVE BLADE DASHBOARD RENDERED:\n{}", rendered);
    assert!(
        rendered.contains("Dashboard Ringkasan"),
        "Must render page heading"
    );
    assert!(
        rendered.contains("Paguyuban Kanfer 2"),
        "Must render org name in layout & sidebar"
    );
    assert!(rendered.contains("65 Jiwa"), "Must render members stat");
    assert!(
        rendered.contains("Rp 8.450.000") || rendered.contains("Rp8.450.000"),
        "Must render rupiah filter output"
    );
    assert!(
        rendered.contains("is-active"),
        "Must mark active sidebar item"
    );
}
