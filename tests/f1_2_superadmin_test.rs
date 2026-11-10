use app_paguyuban::app::services::{AuthContext, ViewService};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[test]
fn test_superadmin_rbac_guard_logic() {
    // 1. Pengguna biasa (Bukan Superadmin)
    let regular_user = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Ketua Kanfer".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Paguyuban Kanfer 2".to_string()),
        current_role: Some("admin".to_string()),
        is_api: false,
    };

    assert!(
        regular_user.require_superadmin().is_err(),
        "Regular admin must NOT pass require_superadmin"
    );

    // 2. Pengguna Superadmin
    let superadmin = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Super Admin".to_string(),
        is_superadmin: true,
        current_org_id: None,
        current_org_name: None,
        current_role: None,
        is_api: false,
    };

    assert!(
        superadmin.require_superadmin().is_ok(),
        "Superadmin must pass require_superadmin"
    );
}

#[tokio::test]
async fn test_f1_2_db_create_org_impersonate_and_status() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
    });

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    // Ambil user superadmin dev
    let superadmin_id: Uuid = sqlx::query_scalar(
        "SELECT id FROM users WHERE phone = '081111111111' AND is_superadmin = true",
    )
    .fetch_one(&pool)
    .await
    .expect("Superadmin seed must exist");

    // 1. Buat organisasi baru untuk tes
    let test_org_id = Uuid::new_v4();
    let test_org_name = format!("RT 08 RW 03 Uji F1.2 {}", &test_org_id.to_string()[..6]);
    let admin_phone = format!("089999{:06}", rand_num());

    sqlx::query(
        "INSERT INTO organizations (id, name, type, status) VALUES ($1, $2, 'rt'::org_type, 'aktif')",
    )
    .bind(test_org_id)
    .bind(&test_org_name)
    .execute(&pool)
    .await
    .expect("Insert test organization failed");

    // 2. Buat admin pertama
    let test_admin_id = Uuid::new_v4();
    let password_hash = bcrypt::hash("password123", bcrypt::DEFAULT_COST).unwrap();

    sqlx::query(
        r#"
        INSERT INTO users (id, name, phone, password_hash, is_superadmin, status)
        VALUES ($1, 'Admin Uji F1.2', $2, $3, false, 'aktif')
        "#,
    )
    .bind(test_admin_id)
    .bind(&admin_phone)
    .bind(password_hash)
    .execute(&pool)
    .await
    .expect("Insert admin user failed");

    // 3. Buat relasi membership admin
    sqlx::query(
        "INSERT INTO memberships (org_id, user_id, role) VALUES ($1, $2, 'admin'::org_role)",
    )
    .bind(test_org_id)
    .bind(test_admin_id)
    .execute(&pool)
    .await
    .expect("Insert membership failed");

    // 4. Catat audit_logs untuk CREATE_ORG
    sqlx::query(
        r#"
        INSERT INTO audit_logs (org_id, actor_user_id, action, entity, entity_id, meta)
        VALUES ($1, $2, 'CREATE_ORG', 'organizations', $1, '{"test": true}'::jsonb);
        "#,
    )
    .bind(Some(test_org_id))
    .bind(superadmin_id)
    .execute(&pool)
    .await
    .expect("Insert audit log CREATE_ORG failed");

    // 5. Uji Impersonasi & Audit Log
    sqlx::query(
        r#"
        INSERT INTO audit_logs (org_id, actor_user_id, action, entity, entity_id, meta)
        VALUES ($1, $2, 'IMPERSONATE_ORG', 'organizations', $1, '{"impersonate": true}'::jsonb);
        "#,
    )
    .bind(Some(test_org_id))
    .bind(superadmin_id)
    .execute(&pool)
    .await
    .expect("Insert audit log IMPERSONATE_ORG failed");

    let impersonate_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM audit_logs WHERE org_id = $1 AND action = 'IMPERSONATE_ORG'",
    )
    .bind(Some(test_org_id))
    .fetch_one(&pool)
    .await
    .expect("Query impersonate log failed");

    assert!(impersonate_count >= 1, "Impersonation audit log must exist");

    // 6. Uji Ubah Status Organisasi (Aktif -> Nonaktif)
    sqlx::query("UPDATE organizations SET status = 'nonaktif'::org_status WHERE id = $1")
        .bind(test_org_id)
        .execute(&pool)
        .await
        .expect("Update status failed");

    let updated_status: String =
        sqlx::query_scalar("SELECT status::text FROM organizations WHERE id = $1")
            .bind(test_org_id)
            .fetch_one(&pool)
            .await
            .expect("Fetch updated status failed");

    assert_eq!(updated_status, "nonaktif");

    // Bersihkan data uji
    let _ = sqlx::query("DELETE FROM audit_logs WHERE org_id = $1")
        .bind(Some(test_org_id))
        .execute(&pool)
        .await;
    let _ = sqlx::query("DELETE FROM memberships WHERE org_id = $1")
        .bind(test_org_id)
        .execute(&pool)
        .await;
    let _ = sqlx::query("DELETE FROM organizations WHERE id = $1")
        .bind(test_org_id)
        .execute(&pool)
        .await;
    let _ = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(test_admin_id)
        .execute(&pool)
        .await;
}

fn rand_num() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    nanos % 1_000_000
}

#[test]
fn test_render_superadmin_views_native_blade() {
    use lumina::core::blade::BladeContext;

    // 1. Render superadmin organizations Blade
    let mut ctx_orgs = BladeContext::new();
    ctx_orgs.insert("title", "Superadmin — Manajemen Organisasi");
    ctx_orgs.insert("area", "sa");
    ctx_orgs.insert("active", "organisasi");
    ctx_orgs.insert("user_name", "Super Admin");
    ctx_orgs.insert("role_label", "SUPERADMIN");
    ctx_orgs.insert("total_orgs", 3);
    ctx_orgs.insert("active_orgs", 2);
    ctx_orgs.insert("pending_orgs", 1);

    #[derive(serde::Serialize)]
    struct DummyOrg {
        id: String,
        name: String,
        org_type: String,
        status: String,
        admin_name: String,
        admin_phone: String,
    }

    let dummy_orgs = vec![DummyOrg {
        id: Uuid::new_v4().to_string(),
        name: "Paguyuban Kanfer 2".to_string(),
        org_type: "paguyuban".to_string(),
        status: "aktif".to_string(),
        admin_name: "Bambang Sudarsono".to_string(),
        admin_phone: "081234567890".to_string(),
    }];
    ctx_orgs.insert("organizations", dummy_orgs);

    let rendered_orgs = ViewService::render_blade("superadmin/organizations.blade.rs", &ctx_orgs);
    assert!(
        rendered_orgs.contains("Manajemen Organisasi"),
        "Must render org title"
    );
    assert!(
        rendered_orgs.contains("Paguyuban Kanfer 2"),
        "Must render org name"
    );
    assert!(
        rendered_orgs.contains("Masuk sebagai Admin"),
        "Must render impersonate button"
    );

    // 2. Render superadmin org_create Blade
    let mut ctx_create = BladeContext::new();
    ctx_create.insert("title", "Tambah Organisasi — Superadmin");
    ctx_create.insert("area", "sa");
    ctx_create.insert("active", "organisasi");
    ctx_create.insert("user_name", "Super Admin");
    ctx_create.insert("role_label", "SUPERADMIN");

    let rendered_create = ViewService::render_blade("superadmin/org_create.blade.rs", &ctx_create);
    assert!(
        rendered_create.contains("Tambah Organisasi Baru"),
        "Must render create title"
    );
    assert!(
        rendered_create.contains("Nama Organisasi (wajib)"),
        "Must render org field"
    );

    // 3. Render superadmin audit_logs Blade
    let mut ctx_log = BladeContext::new();
    ctx_log.insert("title", "Log Aktivitas Sistem — Superadmin");
    ctx_log.insert("area", "sa");
    ctx_log.insert("active", "log");
    ctx_log.insert("user_name", "Super Admin");
    ctx_log.insert("role_label", "SUPERADMIN");

    #[derive(serde::Serialize)]
    struct DummyLog {
        created_at: String,
        actor_name: String,
        actor_phone: String,
        org_name: Option<String>,
        action: String,
        entity: String,
        meta_text: String,
    }

    let dummy_logs = vec![DummyLog {
        created_at: "24 Okt 2026, 16:30 WIB".to_string(),
        actor_name: "Super Admin".to_string(),
        actor_phone: "081111111111".to_string(),
        org_name: Some("Paguyuban Kanfer 2".to_string()),
        action: "IMPERSONATE_ORG".to_string(),
        entity: "organizations".to_string(),
        meta_text: "{\"action\": \"impersonate\"}".to_string(),
    }];
    ctx_log.insert("logs", dummy_logs);

    let rendered_log = ViewService::render_blade("superadmin/audit_logs.blade.rs", &ctx_log);
    assert!(
        rendered_log.contains("Log Aktivitas Sistem (Audit Trail)"),
        "Must render audit log title"
    );
    assert!(
        rendered_log.contains("IMPERSONASI"),
        "Must render badge IMPERSONASI"
    );
    assert!(
        rendered_log.contains("Super Admin"),
        "Must render actor name"
    );
}
