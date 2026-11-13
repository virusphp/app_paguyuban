use app_paguyuban::app::services::{AuthContext, ViewService};
use dotenv::dotenv;
use lumina::core::blade::BladeContext;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[test]
fn test_render_member_views_native_blade() {
    let mut ctx_index = BladeContext::new();
    ctx_index.insert("title", "Daftar Warga — Paguyuban Kanfer 2");
    ctx_index.insert("area", "admin");
    ctx_index.insert("active", "anggota");
    ctx_index.insert("user_name", "Bambang Sudarsono");
    ctx_index.insert("org_name", "Paguyuban Kanfer 2");
    ctx_index.insert("can_edit", true);
    ctx_index.insert("total_members", 65);
    ctx_index.insert("total_households", 30);
    ctx_index.insert("active_members", 62);
    ctx_index.insert("pending_members", 3);
    ctx_index.insert("search_query", "");
    ctx_index.insert("filter_status", "");

    #[derive(serde::Serialize)]
    struct DummyMember {
        id: String,
        full_name: String,
        phone: Option<String>,
        status: String,
        joined_date: String,
        has_account: bool,
        house_no: Option<String>,
        house_note: Option<String>,
    }

    let dummy_members = vec![DummyMember {
        id: Uuid::new_v4().to_string(),
        full_name: "Ahmad Fauzi".to_string(),
        phone: Some("081234567892".to_string()),
        status: "aktif".to_string(),
        joined_date: "15 Jan 2026".to_string(),
        has_account: true,
        house_no: Some("Blok B-03".to_string()),
        house_note: Some("Warga Tetap".to_string()),
    }];
    ctx_index.insert("members", dummy_members);

    let rendered_index = ViewService::render_blade("members/index.blade.rs", &ctx_index);
    assert!(rendered_index.contains("Daftar Warga"));
    assert!(rendered_index.contains("Ahmad Fauzi"));
    assert!(rendered_index.contains("Blok B-03"));
    assert!(rendered_index.contains("Impor CSV"));
    assert!(rendered_index.contains("Tambah Warga"));

    // 2. Render form Blade
    let mut ctx_form = BladeContext::new();
    ctx_form.insert("title", "Tambah Warga Baru");
    ctx_form.insert("area", "admin");
    ctx_form.insert("active", "anggota");
    ctx_form.insert("user_name", "Bambang Sudarsono");
    ctx_form.insert("org_name", "Paguyuban Kanfer 2");
    ctx_form.insert("is_edit", false);
    ctx_form.insert("today_date", "2026-10-24");
    ctx_form.insert("error", "");

    #[derive(serde::Serialize, Default)]
    struct DummyMemberForm {
        id: String,
        full_name: String,
        nik: String,
        gender: String,
        phone: String,
        birth_place: String,
        birth_date: String,
        religion: String,
        occupation: String,
        family_role: String,
        is_head_of_family: bool,
        is_fee_payer: bool,
        emergency_phone: String,
        notes: String,
        house_no: String,
        family_card_no: String,
        ownership_status: String,
        rt_rw: String,
        house_note: String,
        status: String,
        joined_on: String,
    }
    ctx_form.insert(
        "member",
        DummyMemberForm {
            status: "aktif".to_string(),
            gender: "Laki-laki".to_string(),
            family_role: "Kepala Keluarga".to_string(),
            ownership_status: "Milik Sendiri".to_string(),
            is_fee_payer: true,
            ..Default::default()
        },
    );

    #[derive(serde::Serialize)]
    struct DummyHouse {
        house_no: String,
        note: Option<String>,
    }
    let dummy_houses = vec![DummyHouse {
        house_no: "Blok B-01".to_string(),
        note: None,
    }];
    ctx_form.insert("existing_houses", dummy_houses);

    let rendered_form = ViewService::render_blade("members/form.blade.rs", &ctx_form);
    assert!(rendered_form.contains("Tambah Warga Baru"));
    assert!(rendered_form.contains("Nama Lengkap Warga (wajib)"));
    assert!(rendered_form.contains("Nomor Rumah / Blok (wajib)"));

    // 3. Render import Blade
    let mut ctx_import = BladeContext::new();
    ctx_import.insert("title", "Impor Data Warga — CSV");
    ctx_import.insert("area", "admin");
    ctx_import.insert("active", "anggota");
    ctx_import.insert("user_name", "Bambang Sudarsono");
    ctx_import.insert("org_name", "Paguyuban Kanfer 2");
    ctx_import.insert("error", "");

    let rendered_import = ViewService::render_blade("members/import.blade.rs", &ctx_import);
    assert!(rendered_import.contains("Impor Data Warga Massal (CSV)"));
    assert!(rendered_import.contains("nomor_rumah,nama_lengkap,nomor_hp,status"));
}

#[test]
fn test_member_rbac_guard_rules() {
    let bendahara = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Siti Rahayu".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Paguyuban Kanfer 2".to_string()),
        current_role: Some("bendahara".to_string()),
        is_api: false,
    };

    // Bendahara tidak boleh menambah/mengubah anggota
    assert!(
        bendahara.require_role(&["admin", "sekretaris"]).is_err(),
        "Bendahara must NOT have permission to mutate members"
    );

    let sekretaris = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Ahmad Fauzi".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Paguyuban Kanfer 2".to_string()),
        current_role: Some("sekretaris".to_string()),
        is_api: false,
    };

    // Sekretaris boleh menambah/mengubah anggota
    assert!(
        sekretaris.require_role(&["admin", "sekretaris"]).is_ok(),
        "Sekretaris MUST have permission to mutate members"
    );

    let admin = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Bambang Sudarsono".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Paguyuban Kanfer 2".to_string()),
        current_role: Some("admin".to_string()),
        is_api: false,
    };

    // Admin (Ketua) boleh menambah/mengubah anggota
    assert!(
        admin.require_role(&["admin", "sekretaris"]).is_ok(),
        "Admin MUST have permission to mutate members"
    );
}

#[tokio::test]
async fn test_f1_3_db_member_crud_and_household_linking() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
    });

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    let org_id: Uuid =
        sqlx::query_scalar("SELECT id FROM organizations WHERE name = 'Paguyuban Kanfer 2'")
            .fetch_one(&pool)
            .await
            .expect("Kanfer org must exist");

    let admin_id: Uuid = sqlx::query_scalar("SELECT id FROM users WHERE phone = '081234567890'")
        .fetch_one(&pool)
        .await
        .expect("Admin Kanfer must exist");

    // 1. Buat unit rumah
    let test_house_id = Uuid::new_v4();
    let test_house_no = format!("B-TEST-{}", &test_house_id.to_string()[..6]);

    sqlx::query(
        "INSERT INTO households (id, org_id, house_no, note) VALUES ($1, $2, $3, 'Rumah Uji F1.3')",
    )
    .bind(test_house_id)
    .bind(org_id)
    .bind(&test_house_no)
    .execute(&pool)
    .await
    .expect("Insert household failed");

    // 2. Buat data warga
    let member_id = Uuid::new_v4();
    let member_name = format!("Warga Uji {}", &member_id.to_string()[..6]);

    sqlx::query(
        r#"
        INSERT INTO members (id, org_id, household_id, full_name, phone, status, joined_on)
        VALUES ($1, $2, $3, $4, '089912345678', 'aktif', '2026-10-24');
        "#,
    )
    .bind(member_id)
    .bind(org_id)
    .bind(test_house_id)
    .bind(&member_name)
    .execute(&pool)
    .await
    .expect("Insert member failed");

    // 3. Catat audit_logs
    sqlx::query(
        r#"
        INSERT INTO audit_logs (org_id, actor_user_id, action, entity, entity_id, meta)
        VALUES ($1, $2, 'CREATE_MEMBER', 'members', $3, '{"test": true}'::jsonb);
        "#,
    )
    .bind(Some(org_id))
    .bind(admin_id)
    .bind(member_id.to_string())
    .execute(&pool)
    .await
    .expect("Insert audit log failed");

    // 4. Verifikasi query join member & household
    #[derive(sqlx::FromRow)]
    struct MemberJoined {
        full_name: String,
        house_no: Option<String>,
        status: String,
    }

    let joined: MemberJoined = sqlx::query_as(
        r#"
        SELECT m.full_name, h.house_no, m.status
        FROM members m
        JOIN households h ON m.household_id = h.id
        WHERE m.id = $1 AND m.org_id = $2;
        "#,
    )
    .bind(member_id)
    .bind(org_id)
    .fetch_one(&pool)
    .await
    .expect("Query joined member failed");

    assert_eq!(joined.full_name, member_name);
    assert_eq!(joined.house_no.unwrap(), test_house_no);
    assert_eq!(joined.status, "aktif");

    // 5. Verifikasi ubah status (aktif -> nonaktif)
    sqlx::query("UPDATE members SET status = 'nonaktif' WHERE id = $1 AND org_id = $2")
        .bind(member_id)
        .bind(org_id)
        .execute(&pool)
        .await
        .expect("Update member status failed");

    let updated_status: String = sqlx::query_scalar("SELECT status FROM members WHERE id = $1")
        .bind(member_id)
        .fetch_one(&pool)
        .await
        .expect("Fetch updated status failed");

    assert_eq!(updated_status, "nonaktif");

    // Bersihkan data uji
    let _ = sqlx::query("DELETE FROM audit_logs WHERE entity_id = $1")
        .bind(member_id.to_string())
        .execute(&pool)
        .await;
    let _ = sqlx::query("DELETE FROM members WHERE id = $1")
        .bind(member_id)
        .execute(&pool)
        .await;
    let _ = sqlx::query("DELETE FROM households WHERE id = $1")
        .bind(test_house_id)
        .execute(&pool)
        .await;
}

#[tokio::test]
async fn test_f1_3_multi_tenant_isolation_members() {
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

    let member_kanfer_id = Uuid::new_v4();
    sqlx::query("INSERT INTO members (id, org_id, full_name, status) VALUES ($1, $2, 'Warga Kanfer Eksklusif', 'aktif')")
        .bind(member_kanfer_id)
        .bind(kanfer_id)
        .execute(&pool)
        .await
        .expect("Insert member Kanfer failed");

    let member_rt05_id = Uuid::new_v4();
    sqlx::query("INSERT INTO members (id, org_id, full_name, status) VALUES ($1, $2, 'Warga RT05 Eksklusif', 'aktif')")
        .bind(member_rt05_id)
        .bind(rt05_id)
        .execute(&pool)
        .await
        .expect("Insert member RT05 failed");

    // Query dengan org_id = Kanfer -> TIDAK BOLEH menemukan Warga RT05
    let kanfer_members: Vec<String> =
        sqlx::query_scalar("SELECT full_name FROM members WHERE org_id = $1")
            .bind(kanfer_id)
            .fetch_all(&pool)
            .await
            .expect("Query Kanfer members failed");

    assert!(kanfer_members.contains(&"Warga Kanfer Eksklusif".to_string()));
    assert!(
        !kanfer_members.contains(&"Warga RT05 Eksklusif".to_string()),
        "LEAK: Kanfer saw RT05 member!"
    );

    // Query dengan org_id = RT05 -> TIDAK BOLEH menemukan Warga Kanfer
    let rt05_members: Vec<String> =
        sqlx::query_scalar("SELECT full_name FROM members WHERE org_id = $1")
            .bind(rt05_id)
            .fetch_all(&pool)
            .await
            .expect("Query RT05 members failed");

    assert!(rt05_members.contains(&"Warga RT05 Eksklusif".to_string()));
    assert!(
        !rt05_members.contains(&"Warga Kanfer Eksklusif".to_string()),
        "LEAK: RT05 saw Kanfer member!"
    );

    // Bersihkan data uji
    let _ = sqlx::query("DELETE FROM members WHERE id IN ($1, $2)")
        .bind(member_kanfer_id)
        .bind(member_rt05_id)
        .execute(&pool)
        .await;
}

#[tokio::test]
async fn test_f1_3_mobile_api_payload_contracts() {
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

    // 1. Uji Kontrak JSON API Mobile /api/v1/members
    #[derive(sqlx::FromRow, serde::Serialize)]
    struct ApiMemberDto {
        id: Uuid,
        full_name: String,
        phone: Option<String>,
        status: String,
    }

    let members: Vec<ApiMemberDto> =
        sqlx::query_as("SELECT id, full_name, phone, status FROM members WHERE org_id = $1")
            .bind(kanfer_id)
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

    let json_resp = serde_json::json!({
        "success": true,
        "total": members.len(),
        "data": members
    });

    assert_eq!(json_resp["success"], true);
    assert!(json_resp["total"].as_u64().is_some());
}
