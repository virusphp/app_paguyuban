use app_paguyuban::app::services::ViewService;
use lumina::core::blade::BladeContext;
use uuid::Uuid;

#[test]
fn test_render_dashboard_overview_blade() {
    let mut ctx = BladeContext::new();
    ctx.insert("title", "Dashboard — Paguyuban Kanfer 2");
    ctx.insert("active", "dashboard");
    ctx.insert("area", "admin");
    ctx.insert("user_name", "Bambang Sudarsono");
    ctx.insert("org_name", "Paguyuban Kanfer 2");
    ctx.insert("current_org_id", Uuid::new_v4().to_string());
    ctx.insert("role_label", "ADMIN");
    ctx.insert("can_manage", true);

    ctx.insert("total_members", 125);
    ctx.insert("total_households", 42);
    ctx.insert("current_balance", 8500000);
    ctx.insert("fee_collected", 2100000);
    ctx.insert("total_arrears", 450000);
    ctx.insert("arrears_count", 9);
    ctx.insert("collection_percent", 82);

    #[derive(serde::Serialize)]
    struct DummyTx {
        date: String,
        description: String,
        category: String,
        kind: String,
        amount: String,
        tone: String,
    }

    let txs = vec![
        DummyTx {
            date: "07 Nov 2026".to_string(),
            description: "Iuran Warga Blok B-02".to_string(),
            category: "IURAN BULANAN".to_string(),
            kind: "masuk".to_string(),
            amount: "50.000".to_string(),
            tone: "ok".to_string(),
        },
        DummyTx {
            date: "06 Nov 2026".to_string(),
            description: "Pembelian Lampu Pos Ronda".to_string(),
            category: "PEMELIHARAAN".to_string(),
            kind: "keluar".to_string(),
            amount: "125.000".to_string(),
            tone: "bad".to_string(),
        },
    ];
    ctx.insert("recent_transactions", txs);

    #[derive(serde::Serialize)]
    struct DummyEvent {
        id: String,
        name: String,
        date_time: String,
        location: String,
    }

    let events = vec![DummyEvent {
        id: Uuid::new_v4().to_string(),
        name: "Kerja Bakti Bersama".to_string(),
        date_time: "15 Nov 2026, 07:00 WIB".to_string(),
        location: "Lapangan RT 05".to_string(),
    }];
    ctx.insert("upcoming_events", events);

    let html = ViewService::render_blade("dashboard/index.blade.rs", &ctx);

    // Verifikasi konten penting
    assert!(html.contains("Dashboard Ringkasan"));
    assert!(html.contains("Paguyuban Kanfer 2"));
    assert!(html.contains("125"));
    assert!(html.contains("Jiwa"));
    assert!(html.contains("42 Kepala Keluarga"));
    assert!(html.contains("82%"));
    assert!(html.contains("Aktivitas Kas Terbaru"));
    assert!(html.contains("Iuran Warga Blok B-02"));
    assert!(html.contains("Pembelian Lampu Pos Ronda"));
    assert!(html.contains("Pintasan Aksi Pengurus"));
    assert!(html.contains("Kerja Bakti Bersama"));
}

#[test]
fn test_render_org_settings_blade() {
    let mut ctx = BladeContext::new();
    ctx.insert("title", "Pengaturan — Paguyuban Kanfer 2");
    ctx.insert("active", "pengaturan");
    ctx.insert("area", "admin");
    ctx.insert("user_name", "Bambang Sudarsono");
    ctx.insert("org_name", "Paguyuban Kanfer 2");
    ctx.insert("role_label", "ADMIN");
    ctx.insert("current_org_id", Uuid::new_v4().to_string());
    ctx.insert("can_edit", true);
    ctx.insert("org_id", Uuid::new_v4().to_string());
    ctx.insert("org_type", "PAGUYUBAN");
    ctx.insert("org_status", "aktif");
    ctx.insert("created_date", "24 Oktober 2026");

    #[derive(serde::Serialize)]
    struct DummyOfficer {
        name: String,
        phone: String,
        role: String,
    }

    let officers = vec![
        DummyOfficer {
            name: "Bambang Sudarsono".to_string(),
            phone: "081234567890".to_string(),
            role: "admin".to_string(),
        },
        DummyOfficer {
            name: "Siti Rahayu".to_string(),
            phone: "081234567891".to_string(),
            role: "bendahara".to_string(),
        },
        DummyOfficer {
            name: "Ahmad Fauzi".to_string(),
            phone: "081234567892".to_string(),
            role: "sekretaris".to_string(),
        },
    ];
    ctx.insert("officers", officers);

    let html = ViewService::render_blade("org/settings.blade.rs", &ctx);

    assert!(html.contains("Pengaturan Organisasi"));
    assert!(html.contains("Profil Lingkungan"));
    assert!(html.contains("Susunan Pengurus Lingkungan"));
    assert!(html.contains("Bambang Sudarsono"));
    assert!(html.contains("Ketua (Admin)"));
    assert!(html.contains("Siti Rahayu"));
    assert!(html.contains("Bendahara"));
    assert!(html.contains("Ahmad Fauzi"));
    assert!(html.contains("Sekretaris"));
}

#[test]
fn test_render_superadmin_users_blade() {
    let mut ctx = BladeContext::new();
    ctx.insert("title", "Manajemen Pengguna — Superadmin");
    ctx.insert("area", "sa");
    ctx.insert("active", "pengguna");
    ctx.insert("user_name", "Super Admin");
    ctx.insert("role_label", "SUPERADMIN");
    ctx.insert("total_users", 45);
    ctx.insert("total_superadmins", 2);

    #[derive(serde::Serialize)]
    struct DummyUserView {
        name: String,
        phone: String,
        role_badge: String,
        status: String,
        affiliations: String,
        last_login: String,
    }

    let users = vec![
        DummyUserView {
            name: "Super Admin".to_string(),
            phone: "081111111111".to_string(),
            role_badge: "SUPERADMIN".to_string(),
            status: "aktif".to_string(),
            affiliations: "Semua Organisasi".to_string(),
            last_login: "07 Nov 2026, 21:00".to_string(),
        },
        DummyUserView {
            name: "Ketua RT".to_string(),
            phone: "081234567890".to_string(),
            role_badge: "PENGGUNA".to_string(),
            status: "aktif".to_string(),
            affiliations: "Paguyuban Kanfer 2 (admin)".to_string(),
            last_login: "07 Nov 2026, 18:30".to_string(),
        },
    ];
    ctx.insert("users", users);

    let html = ViewService::render_blade("superadmin/users.blade.rs", &ctx);

    assert!(html.contains("Daftar Pengguna Sistem"));
    assert!(html.contains("Total Pengguna Terdaftar"));
    assert!(html.contains("Super Admin"));
    assert!(html.contains("081111111111"));
    assert!(html.contains("Ketua RT"));
    assert!(html.contains("Paguyuban Kanfer 2 (admin)"));
}

#[test]
fn test_render_superadmin_dashboard_blade() {
    let mut ctx = BladeContext::new();
    ctx.insert("title", "Portal Superadmin & Developer — Sistem Paguyuban");
    ctx.insert("area", "sa");
    ctx.insert("active", "dashboard");
    ctx.insert("user_name", "Super Administrator");
    ctx.insert("role_label", "SUPERADMIN");
    ctx.insert("org_name", "Pusat Kontrol Superadmin");

    ctx.insert("total_orgs", 12);
    ctx.insert("active_orgs", 10);
    ctx.insert("total_members", 480);
    ctx.insert("total_households", 160);
    ctx.insert("total_users", 24);
    ctx.insert("global_cash_balance", 45000000i64);
    ctx.insert("total_invoices_count", 960);
    ctx.insert("paid_invoices_count", 820);
    ctx.insert("global_compliance_percent", 85);

    #[derive(serde::Serialize)]
    struct DummyTopOrg {
        id: Uuid,
        name: String,
        org_type: String,
        status: String,
        members_count: i64,
    }

    let top_orgs = vec![
        DummyTopOrg {
            id: Uuid::new_v4(),
            name: "Paguyuban Kanfer 2".to_string(),
            org_type: "paguyuban".to_string(),
            status: "aktif".to_string(),
            members_count: 125,
        },
        DummyTopOrg {
            id: Uuid::new_v4(),
            name: "RT 05 RW 09 Slamaran".to_string(),
            org_type: "rt".to_string(),
            status: "aktif".to_string(),
            members_count: 48,
        },
    ];
    ctx.insert("top_orgs", top_orgs);

    #[derive(serde::Serialize)]
    struct DummyRecentLog {
        created_at: String,
        actor_name: String,
        action: String,
        org_name: Option<String>,
    }

    let logs = vec![DummyRecentLog {
        created_at: "11 Nov 18:00".to_string(),
        actor_name: "Super Admin".to_string(),
        action: "CREATE_ORG".to_string(),
        org_name: Some("RT 05 RW 09 Slamaran".to_string()),
    }];
    ctx.insert("recent_logs", logs);

    let html = ViewService::render_blade("superadmin/dashboard.blade.rs", &ctx);

    assert!(
        html.contains("PORTAL SUPERADMIN &amp; DEVELOPER")
            || html.contains("PORTAL SUPERADMIN & DEVELOPER")
    );
    assert!(
        html.contains("Developer &amp; Engine Telemetry")
            || html.contains("Developer & Engine Telemetry")
    );
    assert!(html.contains("Lumina Framework v0.6.1 (Rust)"));
    assert!(html.contains("PostgreSQL (SQLx Pool)"));
    assert!(html.contains("Paguyuban Kanfer 2"));
    assert!(html.contains("RT 05 RW 09 Slamaran"));
}

#[test]
fn test_render_choose_org_blade() {
    let mut ctx = BladeContext::new();
    ctx.insert("title", "Pilih Organisasi — Aplikasi Paguyuban");
    ctx.insert("user_name", "Super Administrator");
    ctx.insert("is_superadmin", true);

    #[derive(serde::Serialize)]
    struct DummyChooseOrg {
        id: String,
        name: String,
        org_type: String,
        role: String,
        members_count: i64,
        is_active: bool,
    }

    let orgs = vec![DummyChooseOrg {
        id: Uuid::new_v4().to_string(),
        name: "RT 05 RW 09 Slamaran".to_string(),
        org_type: "rt".to_string(),
        role: "Superadmin (Akses Penuh)".to_string(),
        members_count: 48,
        is_active: true,
    }];
    ctx.insert("orgs", orgs);

    let html = ViewService::render_blade("org/choose.blade.rs", &ctx);

    assert!(html.contains("Pilih Lingkungan / Organisasi"));
    assert!(html.contains("RT 05 RW 09 Slamaran"));
    assert!(html.contains("Superadmin (Akses Penuh)"));
    assert!(html.contains("48 Warga Terdata"));
}
