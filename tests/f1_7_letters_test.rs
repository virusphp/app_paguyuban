use app_paguyuban::app::controllers::LetterController;
use app_paguyuban::app::services::{AuthContext, LetterPdfService, ViewService};
use chrono::{Local, TimeZone};
use lumina::core::blade::BladeContext;
use uuid::Uuid;

#[test]
fn test_render_letter_views_native_blade() {
    // 1. Render letters/index.blade.rs
    let mut ctx_index = BladeContext::new();
    ctx_index.insert("title", "Surat & Undangan — Paguyuban");
    ctx_index.insert("area", "admin");
    ctx_index.insert("active", "surat");
    ctx_index.insert("org_name", "Paguyuban Kanfer 2");
    ctx_index.insert("role_label", "SEKRETARIS");
    ctx_index.insert("can_manage", true);
    ctx_index.insert("total_count", 4);
    ctx_index.insert("surat_count", 2);
    ctx_index.insert("undangan_count", 2);
    ctx_index.insert("sent_count", 3);
    ctx_index.insert("filter_kind", "");
    ctx_index.insert("search_query", "");

    #[derive(serde::Serialize, Clone)]
    struct DummyLetterItem {
        id: Uuid,
        kind: String,
        number: String,
        subject: String,
        recipient: String,
        status: String,
        date_formatted: String,
    }

    let dummy_letters = vec![
        DummyLetterItem {
            id: Uuid::new_v4(),
            kind: "surat".to_string(),
            number: "001/PGY-KNF2/XI/2026".to_string(),
            subject: "Surat Pengantar Pembuatan KTP".to_string(),
            recipient: "Bpk. Bambang Sudarsono".to_string(),
            status: "terkirim".to_string(),
            date_formatted: "01 Nov 2026".to_string(),
        },
        DummyLetterItem {
            id: Uuid::new_v4(),
            kind: "undangan".to_string(),
            number: "002/PGY-KNF2/XI/2026".to_string(),
            subject: "Undangan Rapat Musyawarah Warga".to_string(),
            recipient: "Seluruh Warga RT 03".to_string(),
            status: "draft".to_string(),
            date_formatted: "05 Nov 2026".to_string(),
        },
    ];
    ctx_index.insert("letters", dummy_letters.clone());

    let rendered_index = ViewService::render_blade("letters/index.blade.rs", &ctx_index);
    assert!(rendered_index.contains("Surat & Undangan Warga"));
    assert!(rendered_index.contains("Surat Pengantar Pembuatan KTP"));
    assert!(rendered_index.contains("001/PGY-KNF2/XI/2026"));
    assert!(rendered_index.contains("Undangan Rapat Musyawarah Warga"));
    assert!(rendered_index.contains("+ Buat Surat Baru"));

    // 2. Render letters/form.blade.rs (Mode Buat Baru)
    let mut ctx_form = BladeContext::new();
    ctx_form.insert("title", "Buat Surat Baru — Paguyuban");
    ctx_form.insert("area", "admin");
    ctx_form.insert("active", "surat");
    ctx_form.insert("org_name", "Paguyuban Kanfer 2");
    ctx_form.insert("role_label", "SEKRETARIS");
    ctx_form.insert("is_edit", false);
    ctx_form.insert("auto_number", "003/PGY-KNF2/XI/2026");

    #[derive(serde::Serialize)]
    struct DummyCustomTmpl {
        id: Uuid,
        name: String,
        body: String,
        created_at_formatted: String,
    }

    let dummy_templates = vec![DummyCustomTmpl {
        id: Uuid::new_v4(),
        name: "Surat Izin Penggunaan Balai".to_string(),
        body: "Diberikan izin kepada...".to_string(),
        created_at_formatted: "01/11/2026".to_string(),
    }];
    ctx_form.insert("custom_templates", dummy_templates);

    #[derive(serde::Serialize)]
    struct DummyEmptyLetter {
        kind: String,
        number: String,
        recipient: String,
        subject: String,
        body: String,
        status: String,
    }

    ctx_form.insert(
        "letter",
        DummyEmptyLetter {
            kind: "surat".to_string(),
            number: "003/PGY-KNF2/XI/2026".to_string(),
            recipient: String::new(),
            subject: String::new(),
            body: String::new(),
            status: "draft".to_string(),
        },
    );

    let rendered_form = ViewService::render_blade("letters/form.blade.rs", &ctx_form);
    assert!(rendered_form.contains("Buat Surat & Undangan Baru"));
    assert!(rendered_form.contains("Nomor Registrasi Surat"));
    assert!(rendered_form.contains("003/PGY-KNF2/XI/2026"));
    assert!(rendered_form.contains("Gunakan Template Cepat"));
    assert!(rendered_form.contains("Terbitkan Surat"));

    // 3. Render letters/detail.blade.rs
    let mut ctx_detail = BladeContext::new();
    ctx_detail.insert("title", "Surat 001/PGY-KNF2/XI/2026 — Paguyuban");
    ctx_detail.insert("area", "admin");
    ctx_detail.insert("active", "surat");
    ctx_detail.insert("org_name", "Paguyuban Kanfer 2");
    ctx_detail.insert("role_label", "SEKRETARIS");
    ctx_detail.insert("can_manage", true);

    #[derive(serde::Serialize)]
    struct DummyDetailLetter {
        id: Uuid,
        kind: String,
        number: String,
        subject: String,
        body: String,
        recipient: String,
        status: String,
        date_formatted: String,
    }

    ctx_detail.insert(
        "letter",
        DummyDetailLetter {
            id: Uuid::new_v4(),
            kind: "surat".to_string(),
            number: "001/PGY-KNF2/XI/2026".to_string(),
            subject: "Surat Pengantar Pembuatan KTP".to_string(),
            body: "Dengan hormat,\nYang bertanda tangan di bawah ini...".to_string(),
            recipient: "Ketua RW 05 / Petugas Kelurahan Pedalangan".to_string(),
            status: "terkirim".to_string(),
            date_formatted: "01 November 2026".to_string(),
        },
    );

    let rendered_detail = ViewService::render_blade("letters/detail.blade.rs", &ctx_detail);
    assert!(rendered_detail.contains("Surat: 001/PGY-KNF2/XI/2026"));
    assert!(rendered_detail.contains("PENGURUS LINGKUNGAN WARGA"));
    assert!(rendered_detail.contains("Paguyuban Kanfer 2"));
    assert!(rendered_detail.contains("Surat Pengantar Pembuatan KTP"));
    assert!(rendered_detail.contains("Unduh PDF Resmi"));

    // 4. Render letters/templates.blade.rs
    let mut ctx_tmpl = BladeContext::new();
    ctx_tmpl.insert("title", "Template Surat — Paguyuban");
    ctx_tmpl.insert("area", "admin");
    ctx_tmpl.insert("active", "surat");
    ctx_tmpl.insert("org_name", "Paguyuban Kanfer 2");
    ctx_tmpl.insert("role_label", "SEKRETARIS");

    #[derive(serde::Serialize)]
    struct DummyTmplItem {
        id: Uuid,
        name: String,
        body: String,
        created_at_formatted: String,
    }

    let tmpls = vec![DummyTmplItem {
        id: Uuid::new_v4(),
        name: "Surat Izin Penggunaan Balai Warga".to_string(),
        body: "Format izin pemakaian tempat balai warga untuk keperluan acara keluarga."
            .to_string(),
        created_at_formatted: "01/11/2026".to_string(),
    }];
    ctx_tmpl.insert("templates", tmpls);

    let rendered_tmpl = ViewService::render_blade("letters/templates.blade.rs", &ctx_tmpl);
    assert!(rendered_tmpl.contains("Kelola Template Surat & Undangan"));
    assert!(rendered_tmpl.contains("Daftar Template Kustom Organisasi"));
    assert!(rendered_tmpl.contains("Surat Izin Penggunaan Balai Warga"));
    assert!(rendered_tmpl.contains("+ Tambah Template Baru"));
}

#[test]
fn test_letter_auto_number_generation() {
    let test_date = Local.with_ymd_and_hms(2026, 11, 6, 10, 0, 0).unwrap();
    let num1 = LetterController::generate_auto_number(1, "Paguyuban Kanfer 2", test_date);
    assert_eq!(num1, "001/PGY-KNF2/XI/2026");

    let num15 = LetterController::generate_auto_number(15, "Paguyuban Kanfer 2", test_date);
    assert_eq!(num15, "015/PGY-KNF2/XI/2026");

    let test_date_jan = Local.with_ymd_and_hms(2027, 1, 15, 8, 0, 0).unwrap();
    let num_jan = LetterController::generate_auto_number(3, "Rukun Tetangga 03", test_date_jan);
    assert_eq!(num_jan, "003/PGY-RT/I/2027");
}

#[test]
fn test_letter_authorization_matrix() {
    let sekretaris = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Ahmad Fauzi".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Paguyuban Kanfer 2".to_string()),
        current_role: Some("sekretaris".to_string()),
        is_api: false,
    };
    assert!(sekretaris.require_role(&["admin", "sekretaris"]).is_ok());

    let admin = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Bambang Sudarsono".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Paguyuban Kanfer 2".to_string()),
        current_role: Some("admin".to_string()),
        is_api: false,
    };
    assert!(admin.require_role(&["admin", "sekretaris"]).is_ok());

    let bendahara = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Siti Rahayu".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Paguyuban Kanfer 2".to_string()),
        current_role: Some("bendahara".to_string()),
        is_api: false,
    };
    assert!(bendahara.require_role(&["admin", "sekretaris"]).is_err());
}

#[test]
fn test_letter_pdf_export_binary() {
    let pdf_bytes = LetterPdfService::generate_pdf(
        "Paguyuban Kanfer 2",
        "001/PGY-KNF2/XI/2026",
        "Surat Pengantar Pembuatan KTP",
        "Bpk. Bambang Sudarsono",
        "06 November 2026",
        "Dengan hormat, Yang bertanda tangan di bawah ini menerangkan bahwa...",
    );

    assert!(!pdf_bytes.is_empty(), "PDF bytes must not be empty");
    assert!(
        pdf_bytes.starts_with(b"%PDF-"),
        "Generated binary must be a valid PDF document format"
    );
}
