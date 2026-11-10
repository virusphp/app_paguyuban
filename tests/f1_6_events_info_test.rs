use app_paguyuban::app::services::{AuthContext, ViewService};
use chrono::Utc;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[test]
fn test_render_events_and_announcements_views_native_blade() {
    // 1. Render events/index.blade.rs
    let mut ctx_events = lumina::core::blade::BladeContext::new();
    ctx_events.insert("title", "Agenda Kegiatan — Paguyuban");
    ctx_events.insert("area", "admin");
    ctx_events.insert("active", "kegiatan");
    ctx_events.insert("org_name", "Paguyuban Kanfer 2");
    ctx_events.insert("role_label", "SEKRETARIS");
    ctx_events.insert("can_manage", true);
    ctx_events.insert("upcoming_count", 2);
    ctx_events.insert("total_count", 5);
    ctx_events.insert("average_rsvp_percentage", 85);

    #[derive(serde::Serialize)]
    struct DummyEventItem {
        id: Uuid,
        title: String,
        date_formatted: String,
        time_formatted: String,
        location: Option<String>,
        description: Option<String>,
        status: String,
        attending_count: i64,
        absent_count: i64,
    }

    let dummy_events = vec![
        DummyEventItem {
            id: Uuid::new_v4(),
            title: "Kerja Bakti Saluran Air".to_string(),
            date_formatted: "25 Oktober 2026".to_string(),
            time_formatted: "07:00".to_string(),
            location: Some("Balai RT 03 / Lapangan Utama".to_string()),
            description: Some("Kerja bakti pembersihan selokan menjelang musim hujan".to_string()),
            status: "aktif".to_string(),
            attending_count: 24,
            absent_count: 3,
        },
        DummyEventItem {
            id: Uuid::new_v4(),
            title: "Senam Pagi Bersama Lansia".to_string(),
            date_formatted: "01 November 2026".to_string(),
            time_formatted: "06:30".to_string(),
            location: Some("Taman Kanfer".to_string()),
            description: None,
            status: "selesai".to_string(),
            attending_count: 18,
            absent_count: 2,
        },
    ];
    ctx_events.insert("events", dummy_events);

    let rendered_events = ViewService::render_blade("events/index.blade.rs", &ctx_events);
    assert!(rendered_events.contains("Agenda Kegiatan & Acara Warga"));
    assert!(rendered_events.contains("Kerja Bakti Saluran Air"));
    assert!(rendered_events.contains("Balai RT 03"));
    assert!(rendered_events.contains("24 Hadir"));
    assert!(rendered_events.contains("+ Tambah Kegiatan Baru"));

    // 2. Render events/form.blade.rs (Create Mode)
    let mut ctx_form = lumina::core::blade::BladeContext::new();
    ctx_form.insert("title", "Jadwalkan Acara Baru — Paguyuban");
    ctx_form.insert("area", "admin");
    ctx_form.insert("active", "kegiatan");
    ctx_form.insert("org_name", "Paguyuban Kanfer 2");
    ctx_form.insert("role_label", "SEKRETARIS");
    ctx_form.insert("is_edit", false);
    ctx_form.insert("default_starts_at", "2026-10-25T19:30");

    let rendered_form = ViewService::render_blade("events/form.blade.rs", &ctx_form);
    assert!(rendered_form.contains("Tambah Kegiatan Baru"));
    assert!(rendered_form.contains("Nama Kegiatan / Agenda Acara (wajib)"));
    assert!(rendered_form.contains("Waktu & Tanggal Mulai (wajib)"));
    assert!(rendered_form.contains("Terbitkan Kegiatan"));

    // 3. Render events/detail.blade.rs
    let mut ctx_detail = lumina::core::blade::BladeContext::new();
    ctx_detail.insert("title", "Detail Kegiatan — Paguyuban");
    ctx_detail.insert("area", "admin");
    ctx_detail.insert("active", "kegiatan");
    ctx_detail.insert("org_name", "Paguyuban Kanfer 2");
    ctx_detail.insert("role_label", "SEKRETARIS");
    ctx_detail.insert("can_manage", true);

    #[derive(serde::Serialize)]
    struct DummyDetail {
        id: Uuid,
        title: String,
        datetime_formatted: String,
        location: Option<String>,
        description: Option<String>,
        status: String,
    }

    let detail_obj = DummyDetail {
        id: Uuid::new_v4(),
        title: "Pertemuan Rutin Pengurus & Warga".to_string(),
        datetime_formatted: "Minggu, 25 Oktober 2026 pukul 19:30 WIB".to_string(),
        location: Some("Rumah Ketua RW 05".to_string()),
        description: Some(
            "Pembahasan rencana renovasi pos kamling dan laporan keuangan".to_string(),
        ),
        status: "aktif".to_string(),
    };
    ctx_detail.insert("event", detail_obj);
    ctx_detail.insert("attending_count", 15);
    ctx_detail.insert("absent_count", 3);
    ctx_detail.insert("pending_count", 10);

    #[derive(serde::Serialize)]
    struct DummyRsvp {
        id: Uuid,
        full_name: String,
        house_no: String,
        phone: Option<String>,
        answer: String,
        confirmed_at: Option<String>,
    }

    let dummy_rsvps = vec![DummyRsvp {
        id: Uuid::new_v4(),
        full_name: "Budi Santoso".to_string(),
        house_no: "Blok A1 No. 5".to_string(),
        phone: Some("08123456789".to_string()),
        answer: "hadir".to_string(),
        confirmed_at: Some("24 Okt 2026 10:15 WIB".to_string()),
    }];
    ctx_detail.insert("rsvps", dummy_rsvps);

    #[derive(serde::Serialize)]
    struct DummyMemberOption {
        id: Uuid,
        full_name: String,
        house_no: String,
    }
    let dummy_members = vec![DummyMemberOption {
        id: Uuid::new_v4(),
        full_name: "Siti Rahmawati".to_string(),
        house_no: "Blok B2 No. 8".to_string(),
    }];
    ctx_detail.insert("members_options", dummy_members);

    let rendered_detail = ViewService::render_blade("events/detail.blade.rs", &ctx_detail);
    assert!(rendered_detail.contains("Pertemuan Rutin"));
    assert!(rendered_detail.contains("Rekapitulasi Kehadiran Warga"));
    assert!(rendered_detail.contains("Budi Santoso"));
    assert!(rendered_detail.contains("Catat Respon Kehadiran Warga"));

    // 4. Render announcements/index.blade.rs
    let mut ctx_announcements = lumina::core::blade::BladeContext::new();
    ctx_announcements.insert("title", "Papan Informasi — Paguyuban");
    ctx_announcements.insert("area", "admin");
    ctx_announcements.insert("active", "info");
    ctx_announcements.insert("org_name", "Paguyuban Kanfer 2");
    ctx_announcements.insert("role_label", "ADMIN");
    ctx_announcements.insert("can_manage", true);

    #[derive(serde::Serialize)]
    struct DummyAnnouncementItem {
        id: Uuid,
        title: String,
        body: String,
        pinned: bool,
        published_at_formatted: String,
        author_name: Option<String>,
    }

    let dummy_announcements = vec![
        DummyAnnouncementItem {
            id: Uuid::new_v4(),
            title: "Jadwal Pengambilan Bansos Sembako".to_string(),
            body: "Pengambilan dapat dilakukan mulai besok pagi di Balai RT setempat.".to_string(),
            pinned: true,
            published_at_formatted: "24 Okt 2026 09:00".to_string(),
            author_name: Some("Budi Santoso (Ketua)".to_string()),
        },
        DummyAnnouncementItem {
            id: Uuid::new_v4(),
            title: "Pemberitahuan Pemadaman Listrik Sementara".to_string(),
            body: "Akan ada perbaikan gardu listrik oleh PLN pada hari Selasa mendatang."
                .to_string(),
            pinned: false,
            published_at_formatted: "23 Okt 2026 14:00".to_string(),
            author_name: Some("Ahmad Sekretaris".to_string()),
        },
    ];
    ctx_announcements.insert("announcements", dummy_announcements);

    let rendered_announcements =
        ViewService::render_blade("announcements/index.blade.rs", &ctx_announcements);
    assert!(rendered_announcements.contains("Papan Informasi & Pengumuman Warga"));
    assert!(rendered_announcements.contains("📌 PENGUMUMAN TERSEMAT"));
    assert!(rendered_announcements.contains("Jadwal Pengambilan Bansos Sembako"));
    assert!(rendered_announcements.contains("Pemberitahuan Pemadaman Listrik Sementara"));

    // 5. Render announcements/form.blade.rs
    let mut ctx_anno_form = lumina::core::blade::BladeContext::new();
    ctx_anno_form.insert("title", "Terbitkan Pengumuman Baru");
    ctx_anno_form.insert("area", "admin");
    ctx_anno_form.insert("active", "info");
    ctx_anno_form.insert("org_name", "Paguyuban Kanfer 2");
    ctx_anno_form.insert("role_label", "ADMIN");
    ctx_anno_form.insert("is_edit", false);

    let rendered_anno_form =
        ViewService::render_blade("announcements/form.blade.rs", &ctx_anno_form);
    assert!(rendered_anno_form.contains("Terbitkan Pengumuman Baru"));
    assert!(rendered_anno_form.contains("Judul Pengumuman (wajib)"));
    assert!(rendered_anno_form.contains("📌 Sematkan Pengumuman Ini"));
    assert!(rendered_anno_form.contains("Terbitkan Sekarang"));
}

#[tokio::test]
async fn test_events_and_rsvp_database_flow() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
    });

    let pool = match PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await
    {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Melewati test_events_and_rsvp_database_flow: {}", e);
            return;
        }
    };

    // Ambil org Kanfer
    let org_id: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM organizations WHERE name ILIKE '%Kanfer%' LIMIT 1")
            .fetch_optional(&pool)
            .await
            .unwrap_or(None);

    let org_id = match org_id {
        Some(id) => id,
        None => {
            eprintln!("Organisasi Kanfer tidak ditemukan, lewati test DB");
            return;
        }
    };

    // Ambil dua anggota untuk tes RSVP
    #[derive(sqlx::FromRow)]
    struct MemberRow {
        id: Uuid,
    }
    let members: Vec<MemberRow> =
        sqlx::query_as("SELECT id FROM members WHERE org_id = $1 LIMIT 2")
            .bind(org_id)
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

    if members.len() < 2 {
        eprintln!("Minimal butuh 2 member untuk tes RSVP");
        return;
    }

    let member_1 = members[0].id;
    let member_2 = members[1].id;

    // 1. Buat Acara Baru
    let event_id = Uuid::new_v4();
    let event_title = format!("Uji Kerja Bakti Bersama {}", Uuid::new_v4().simple());
    let starts_at = Utc::now() + chrono::Duration::days(3);

    let ins_ev = sqlx::query(
        r#"
        INSERT INTO events (id, org_id, title, starts_at, location, description, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(event_id)
    .bind(org_id)
    .bind(&event_title)
    .bind(starts_at)
    .bind("Lapangan Kanfer RT 02")
    .bind("Deskripsi kerja bakti lingkungan")
    .bind("aktif")
    .execute(&pool)
    .await;

    assert!(ins_ev.is_ok(), "Insert event harus berhasil");

    // 2. Catat RSVP Member 1: 'hadir'
    let rsvp_1_id = Uuid::new_v4();
    let rsvp_1 = sqlx::query(
        r#"
        INSERT INTO event_rsvps (id, org_id, event_id, member_id, answer)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (event_id, member_id) DO UPDATE SET answer = EXCLUDED.answer
        "#,
    )
    .bind(rsvp_1_id)
    .bind(org_id)
    .bind(event_id)
    .bind(member_1)
    .bind("hadir")
    .execute(&pool)
    .await;
    assert!(rsvp_1.is_ok(), "RSVP 1 harus berhasil dicatat");

    // 3. Catat RSVP Member 2: 'tidak'
    let rsvp_2_id = Uuid::new_v4();
    let rsvp_2 = sqlx::query(
        r#"
        INSERT INTO event_rsvps (id, org_id, event_id, member_id, answer)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (event_id, member_id) DO UPDATE SET answer = EXCLUDED.answer
        "#,
    )
    .bind(rsvp_2_id)
    .bind(org_id)
    .bind(event_id)
    .bind(member_2)
    .bind("tidak")
    .execute(&pool)
    .await;
    assert!(rsvp_2.is_ok(), "RSVP 2 harus berhasil dicatat");

    // 4. Verifikasi Agregasi RSVP
    let hadir_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM event_rsvps WHERE event_id = $1 AND answer = 'hadir'",
    )
    .bind(event_id)
    .fetch_one(&pool)
    .await
    .unwrap_or(0);
    assert_eq!(hadir_count, 1, "Harus ada 1 warga hadir");

    let tidak_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM event_rsvps WHERE event_id = $1 AND answer = 'tidak'",
    )
    .bind(event_id)
    .fetch_one(&pool)
    .await
    .unwrap_or(0);
    assert_eq!(tidak_count, 1, "Harus ada 1 warga tidak hadir");

    // 5. Uji Idempotensi: Member 1 mengubah jawaban menjadi 'tidak'
    let rsvp_update = sqlx::query(
        r#"
        INSERT INTO event_rsvps (id, org_id, event_id, member_id, answer)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (event_id, member_id) DO UPDATE SET answer = EXCLUDED.answer
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(org_id)
    .bind(event_id)
    .bind(member_1)
    .bind("tidak")
    .execute(&pool)
    .await;
    assert!(rsvp_update.is_ok(), "Update RSVP in-place harus berhasil");

    let total_rsvps: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM event_rsvps WHERE event_id = $1")
            .bind(event_id)
            .fetch_one(&pool)
            .await
            .unwrap_or(0);
    assert_eq!(
        total_rsvps, 2,
        "Total baris RSVP tidak boleh bertambah (tetap 2)"
    );

    let baru_tidak_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM event_rsvps WHERE event_id = $1 AND answer = 'tidak'",
    )
    .bind(event_id)
    .fetch_one(&pool)
    .await
    .unwrap_or(0);
    assert_eq!(baru_tidak_count, 2, "Sekarang ada 2 warga tidak hadir");
}

#[tokio::test]
async fn test_announcements_pinned_ordering_and_isolation() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
    });

    let pool = match PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await
    {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Melewati test_announcements: {}", e);
            return;
        }
    };

    // Ambil org Kanfer
    let org_id: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM organizations WHERE name ILIKE '%Kanfer%' LIMIT 1")
            .fetch_optional(&pool)
            .await
            .unwrap_or(None);

    let org_id = match org_id {
        Some(id) => id,
        None => return,
    };

    // 1. Buat pengumuman unpinned (terbit lebih baru)
    let unpinned_id = Uuid::new_v4();
    let unpinned_title = format!("Info Biasa {}", Uuid::new_v4().simple());
    let _ = sqlx::query(
        r#"
        INSERT INTO announcements (id, org_id, title, body, pinned, published_at)
        VALUES ($1, $2, $3, $4, FALSE, $5)
        "#,
    )
    .bind(unpinned_id)
    .bind(org_id)
    .bind(&unpinned_title)
    .bind("Isi info biasa")
    .bind(Utc::now())
    .execute(&pool)
    .await;

    // 2. Buat pengumuman pinned (terbit lebih lama / 2 hari lalu)
    let pinned_id = Uuid::new_v4();
    let pinned_title = format!("Info Tersemat {}", Uuid::new_v4().simple());
    let _ = sqlx::query(
        r#"
        INSERT INTO announcements (id, org_id, title, body, pinned, published_at)
        VALUES ($1, $2, $3, $4, TRUE, $5)
        "#,
    )
    .bind(pinned_id)
    .bind(org_id)
    .bind(&pinned_title)
    .bind("Isi info tersemat")
    .bind(Utc::now() - chrono::Duration::days(2))
    .execute(&pool)
    .await;

    // 3. Query dengan ORDER BY pinned DESC, published_at DESC
    #[derive(sqlx::FromRow)]
    #[allow(dead_code)]
    struct AnnoRow {
        id: Uuid,
        title: String,
        pinned: bool,
    }

    let rows: Vec<AnnoRow> = sqlx::query_as(
        r#"
        SELECT id, title, pinned
        FROM announcements
        WHERE org_id = $1 AND id IN ($2, $3)
        ORDER BY pinned DESC, published_at DESC
        "#,
    )
    .bind(org_id)
    .bind(unpinned_id)
    .bind(pinned_id)
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    assert_eq!(rows.len(), 2);
    assert_eq!(
        rows[0].id, pinned_id,
        "Item pinned harus selalu di urutan pertama"
    );
    assert!(rows[0].pinned);
    assert_eq!(
        rows[1].id, unpinned_id,
        "Item unpinned berada di urutan setelahnya"
    );
    assert!(!rows[1].pinned);

    // 4. Multi-tenant isolation check
    let dummy_other_org = Uuid::new_v4();
    let other_org_items: Vec<AnnoRow> =
        sqlx::query_as("SELECT id, title, pinned FROM announcements WHERE org_id = $1 AND id = $2")
            .bind(dummy_other_org)
            .bind(pinned_id)
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

    assert!(
        other_org_items.is_empty(),
        "Pengumuman tidak boleh bocor ke org lain"
    );
}

#[test]
fn test_events_and_announcements_rbac_matrix() {
    let admin_auth = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Pak RW".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Kanfer".to_string()),
        current_role: Some("admin".to_string()),
        is_api: false,
    };
    assert!(admin_auth.require_role(&["admin", "sekretaris"]).is_ok());

    let sekretaris_auth = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Mas Sekretaris".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Kanfer".to_string()),
        current_role: Some("sekretaris".to_string()),
        is_api: false,
    };
    assert!(sekretaris_auth
        .require_role(&["admin", "sekretaris"])
        .is_ok());

    let bendahara_auth = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Bu Bendahara".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Kanfer".to_string()),
        current_role: Some("bendahara".to_string()),
        is_api: false,
    };
    // Bendahara hanya boleh melihat, tidak boleh mengelola / memodifikasi
    assert!(bendahara_auth
        .require_role(&["admin", "sekretaris"])
        .is_err());

    let warga_auth = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Warga Biasa".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Kanfer".to_string()),
        current_role: Some("warga".to_string()),
        is_api: false,
    };
    assert!(warga_auth.require_role(&["admin", "sekretaris"]).is_err());
}

#[test]
fn test_mobile_api_events_and_announcements_contracts() {
    use app_paguyuban::app::controllers::announcement_api_controller::ApiAnnouncementDto;
    use app_paguyuban::app::controllers::event_api_controller::{ApiEventDto, SendRsvpPayload};

    let ev_dto = ApiEventDto {
        id: Uuid::new_v4(),
        title: "Pemeriksaan Kesehatan Gratis".to_string(),
        starts_at: "2026-10-30T08:00:00Z".to_string(),
        location: Some("Balai RT 01".to_string()),
        description: Some("Cek tensi dan gula darah".to_string()),
        status: "aktif".to_string(),
        attending_count: 25,
        absent_count: 2,
        my_rsvp: Some("hadir".to_string()),
    };

    let json_ev = serde_json::to_string(&ev_dto).expect("Serialisasi ApiEventDto harus sukses");
    assert!(json_ev.contains("Pemeriksaan Kesehatan Gratis"));
    assert!(json_ev.contains("25"));
    assert!(json_ev.contains("hadir"));

    let rsvp_payload: SendRsvpPayload = serde_json::from_str(
        r#"{"member_id": "11111111-1111-1111-1111-111111111111", "answer": "hadir"}"#,
    )
    .expect("Deserialisasi SendRsvpPayload harus sukses");
    assert_eq!(rsvp_payload.answer, "hadir");
    assert!(rsvp_payload.member_id.is_some());

    let anno_dto = ApiAnnouncementDto {
        id: Uuid::new_v4(),
        title: "Kerja Bakti Akbar".to_string(),
        body: "Dimohon seluruh warga membawa alat kebersihan".to_string(),
        pinned: true,
        published_at: "2026-10-24T12:00:00Z".to_string(),
        author_name: Some("Ketua Paguyuban".to_string()),
    };
    let json_anno =
        serde_json::to_string(&anno_dto).expect("Serialisasi ApiAnnouncementDto harus sukses");
    assert!(json_anno.contains("Kerja Bakti Akbar"));
    assert!(json_anno.contains("pinned\":true"));
}
