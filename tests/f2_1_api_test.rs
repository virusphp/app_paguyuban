use app_paguyuban::app::services::{login_rate_limiter, AuthContext};
use dotenv::dotenv;
use lumina::core::auth::AuthUser;
use std::time::Duration;
use uuid::Uuid;

#[tokio::test]
async fn test_auth_api_jwt_token_generation_and_validation() {
    dotenv().ok();
    if std::env::var("APP_KEY").is_err() {
        std::env::set_var("APP_KEY", "test-secret-key-12345");
    }

    let user_id = Uuid::new_v4();
    let org_id = Uuid::new_v4();
    let user_name = "Ahmad Dahlan (Warga)";
    let phone = "081299998888";
    let role = "anggota".to_string();

    let permissions = vec![
        format!("org_id:{}", org_id),
        "org_name:Paguyuban Kanfer 2".to_string(),
        "org_type:paguyuban".to_string(),
    ];

    let mut auth_user = AuthUser::with_name(
        1,
        user_name,
        phone.to_string(),
        role.clone(),
        permissions.clone(),
        24 * 30, // 30 hari
    );
    auth_user.sub = user_id.to_string();

    // 1. Generate token JWT
    let token =
        lumina::http::auth::generate_token(&auth_user).expect("JWT Token generation must succeed");
    assert!(!token.is_empty(), "Token JWT tidak boleh kosong");

    // 2. Decode dan validasi token
    let decoded =
        lumina::http::auth::validate_token(&token).expect("JWT Token validation must succeed");

    assert_eq!(decoded.sub, user_id.to_string());
    assert_eq!(decoded.name, user_name);
    assert_eq!(decoded.email, phone);
    assert_eq!(decoded.role, role);
    assert_eq!(decoded.permissions.len(), 3);

    // 3. Ekstrak data organisasi dari permission claims
    let parsed_org_id = decoded
        .permissions
        .iter()
        .find_map(|p| p.strip_prefix("org_id:"))
        .and_then(|id_str| Uuid::parse_str(id_str).ok());

    assert_eq!(parsed_org_id, Some(org_id));

    let parsed_org_name = decoded
        .permissions
        .iter()
        .find_map(|p| p.strip_prefix("org_name:"));

    assert_eq!(parsed_org_name, Some("Paguyuban Kanfer 2"));
}

#[tokio::test]
async fn test_auth_api_token_revocation_blacklist() {
    dotenv().ok();
    if std::env::var("APP_KEY").is_err() {
        std::env::set_var("APP_KEY", "test-secret-key-12345");
    }

    let user_id = Uuid::new_v4();
    let mut auth_user = AuthUser::with_name(
        1,
        "Uji Pembatalan",
        "081200000001".to_string(),
        "anggota".to_string(),
        vec![],
        1,
    );
    auth_user.sub = user_id.to_string();

    let token = lumina::http::auth::generate_token(&auth_user).unwrap();

    // Pastikan belum masuk blacklist
    assert!(
        !lumina::core::auth::TokenBlacklist::is_revoked(&token).await,
        "Token baru tidak boleh masuk blacklist"
    );

    // Batalkan token (Logout)
    lumina::core::auth::TokenBlacklist::revoke(&token).await;

    // Pastikan token sekarang terdeteksi telah dibatalkan
    assert!(
        lumina::core::auth::TokenBlacklist::is_revoked(&token).await,
        "Token yang telah dicabut harus terdeteksi di blacklist"
    );
}

#[tokio::test]
async fn test_auth_api_rate_limiter_integration() {
    let phone = "081288887777";
    let window = Duration::from_secs(60);
    let max_attempts = 5;

    // Bersihkan state awal
    login_rate_limiter().reset(phone);

    // 5x percobaan gagal berurutan
    for _ in 0..5 {
        assert!(login_rate_limiter()
            .check_limit(phone, max_attempts, window)
            .is_ok());
        login_rate_limiter().record_failure(phone, window);
    }

    // Percobaan ke-6 harus ditolak oleh Rate Limiter
    let check = login_rate_limiter().check_limit(phone, max_attempts, window);
    assert!(
        check.is_err(),
        "Percobaan setelah 5x kegagalan harus diblokir oleh Rate Limiter"
    );

    // Reset setelah pengguna berhasil login atau jeda waktu berakhir
    login_rate_limiter().reset(phone);
    assert!(
        login_rate_limiter()
            .check_limit(phone, max_attempts, window)
            .is_ok(),
        "Setelah direset, pengguna harus diizinkan mencoba kembali"
    );
}

#[tokio::test]
async fn test_auth_context_role_and_org_helpers() {
    let user_id = Uuid::new_v4();
    let org_id = Uuid::new_v4();

    let auth_ctx = AuthContext {
        user_id,
        user_name: "Budi Santoso".to_string(),
        is_superadmin: false,
        current_org_id: Some(org_id),
        current_org_name: Some("Paguyuban Kanfer 2".to_string()),
        current_role: Some("anggota".to_string()),
        is_api: true,
    };

    // require_org harus berhasil mengembalikan Uuid
    assert_eq!(auth_ctx.require_org().unwrap(), org_id);

    // require_role anggota harus lolos
    assert!(auth_ctx.require_role(&["anggota", "admin"]).is_ok());

    // require_role admin harus ditolak karena perannya adalah anggota
    assert!(auth_ctx.require_role(&["admin", "bendahara"]).is_err());

    // role display
    assert_eq!(auth_ctx.role_display(), "ANGGOTA");
}

#[tokio::test]
async fn test_letter_api_pdf_export_binary() {
    let pdf_bytes = app_paguyuban::app::services::LetterPdfService::generate_pdf(
        "Paguyuban Kanfer 2",
        "001/PGY-KNF2/XI/2026",
        "Undangan Kerja Bakti Lingkungan",
        "Seluruh Warga RT 05",
        "08 November 2026",
        "Diharapkan kehadiran seluruh bapak/ibu warga pada acara kerja bakti lingkungan.",
    );

    assert!(
        !pdf_bytes.is_empty(),
        "Keluaran PDF biner tidak boleh kosong"
    );
    assert!(
        pdf_bytes.starts_with(b"%PDF-"),
        "Header biner harus berformat PDF resmi (%PDF-)"
    );
}

#[tokio::test]
async fn test_letter_api_access_control_rules() {
    // 1. Kasus Warga Biasa (role: anggota)
    let member_ctx = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Warga Biasa".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Paguyuban Kanfer 2".to_string()),
        current_role: Some("anggota".to_string()),
        is_api: true,
    };

    let can_view_drafts_member = member_ctx.is_superadmin
        || matches!(
            member_ctx.current_role.as_deref(),
            Some("admin") | Some("sekretaris")
        );

    assert!(
        !can_view_drafts_member,
        "Warga biasa tidak boleh memiliki izin melihat surat draft"
    );

    // 2. Kasus Sekretaris Pengurus
    let sekretaris_ctx = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Sekretaris Paguyuban".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Paguyuban Kanfer 2".to_string()),
        current_role: Some("sekretaris".to_string()),
        is_api: true,
    };

    let can_view_drafts_sekretaris = sekretaris_ctx.is_superadmin
        || matches!(
            sekretaris_ctx.current_role.as_deref(),
            Some("admin") | Some("sekretaris")
        );

    assert!(
        can_view_drafts_sekretaris,
        "Sekretaris harus memiliki izin melihat surat draft"
    );

    // 3. Kasus Ketua (Admin)
    let admin_ctx = AuthContext {
        user_id: Uuid::new_v4(),
        user_name: "Ketua Paguyuban".to_string(),
        is_superadmin: false,
        current_org_id: Some(Uuid::new_v4()),
        current_org_name: Some("Paguyuban Kanfer 2".to_string()),
        current_role: Some("admin".to_string()),
        is_api: true,
    };

    let can_view_drafts_admin = admin_ctx.is_superadmin
        || matches!(
            admin_ctx.current_role.as_deref(),
            Some("admin") | Some("sekretaris")
        );

    assert!(
        can_view_drafts_admin,
        "Ketua (admin) harus memiliki izin melihat surat draft"
    );
}

#[tokio::test]
async fn test_letter_api_dto_serialization() {
    use app_paguyuban::app::controllers::{ApiLetterDetailDto, ApiLetterSummaryDto};

    let letter_id = Uuid::new_v4();
    let org_id = Uuid::new_v4();

    let summary = ApiLetterSummaryDto {
        id: letter_id,
        kind: "undangan".to_string(),
        number: "005/PGY-KNF2/XI/2026".to_string(),
        subject: "Rapat Warga Bulanan".to_string(),
        recipient: "Warga RT 05".to_string(),
        status: "terkirim".to_string(),
        date_formatted: "08 Nov 2026".to_string(),
        author_name: Some("Budi (Sekretaris)".to_string()),
        sent_at: Some("08 Nov 2026 09:00".to_string()),
    };

    let json_val = serde_json::to_value(&summary).expect("Serialization summary must succeed");
    assert_eq!(json_val["kind"], "undangan");
    assert_eq!(json_val["status"], "terkirim");
    assert_eq!(json_val["number"], "005/PGY-KNF2/XI/2026");

    let detail = ApiLetterDetailDto {
        id: letter_id,
        org_id,
        org_name: "Paguyuban Kanfer 2".to_string(),
        kind: "undangan".to_string(),
        number: "005/PGY-KNF2/XI/2026".to_string(),
        subject: "Rapat Warga Bulanan".to_string(),
        recipient: "Warga RT 05".to_string(),
        body: "Isi teks undangan rapat...".to_string(),
        status: "terkirim".to_string(),
        date_formatted: "08 November 2026".to_string(),
        author_name: Some("Budi (Sekretaris)".to_string()),
        sent_at: Some("08 Nov 2026 09:00".to_string()),
    };

    let json_detail = serde_json::to_value(&detail).expect("Serialization detail must succeed");
    assert_eq!(json_detail["body"], "Isi teks undangan rapat...");
    assert_eq!(json_detail["org_name"], "Paguyuban Kanfer 2");
}

#[tokio::test]
async fn test_api_response_formatting_helpers() {
    use app_paguyuban::app::services::ApiResponse;
    use axum::body::to_bytes;
    use axum::http::StatusCode;

    // 1. Format respons sukses dengan payload data
    let res = ApiResponse::success(serde_json::json!({ "id": 123, "name": "Warga Test" }));
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX)
        .await
        .expect("Reading body bytes must succeed");
    let body: serde_json::Value = serde_json::from_slice(&bytes).expect("Body must be valid JSON");
    assert_eq!(body["success"], true);
    assert_eq!(body["code"], 200);
    assert_eq!(body["data"]["id"], 123);
    assert_eq!(body["data"]["name"], "Warga Test");

    // 2. Format respons sukses dengan pesan teks
    let res = ApiResponse::success_msg("Operasi berhasil disimpan");
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = to_bytes(res.into_body(), usize::MAX)
        .await
        .expect("Reading body bytes must succeed");
    let body: serde_json::Value = serde_json::from_slice(&bytes).expect("Body must be valid JSON");
    assert_eq!(body["success"], true);
    assert_eq!(body["message"], "Operasi berhasil disimpan");
    assert_eq!(body["code"], 200);

    // 3. Format respons error dengan status code tertentu
    let res = ApiResponse::error(StatusCode::NOT_FOUND, "Resource tidak ditemukan");
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    let bytes = to_bytes(res.into_body(), usize::MAX)
        .await
        .expect("Reading body bytes must succeed");
    let body: serde_json::Value = serde_json::from_slice(&bytes).expect("Body must be valid JSON");
    assert_eq!(body["success"], false);
    assert_eq!(body["code"], 404);
    assert_eq!(body["error"], "Resource tidak ditemukan");
}

#[tokio::test]
async fn test_cors_middleware_preflight_and_headers() {
    use axum::{
        body::Body,
        http::{header, Method, Request, StatusCode},
        routing::get,
        Router,
    };
    use tower::util::ServiceExt;

    let app =
        Router::new()
            .route("/test", get(|| async { "ok" }))
            .layer(axum::middleware::from_fn(
                app_paguyuban::app::services::cors_middleware,
            ));

    // 1. Preflight OPTIONS request
    let preflight_req = Request::builder()
        .method(Method::OPTIONS)
        .uri("/test")
        .header(header::ORIGIN, "http://localhost:5173")
        .header(header::ACCESS_CONTROL_REQUEST_METHOD, "POST")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(preflight_req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    assert_eq!(
        response
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .unwrap(),
        "http://localhost:5173"
    );
    assert!(response
        .headers()
        .contains_key(header::ACCESS_CONTROL_ALLOW_METHODS));
    assert!(response
        .headers()
        .contains_key(header::ACCESS_CONTROL_ALLOW_HEADERS));
    assert_eq!(
        response
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_CREDENTIALS)
            .unwrap(),
        "true"
    );
    assert_eq!(
        response
            .headers()
            .get(header::ACCESS_CONTROL_MAX_AGE)
            .unwrap(),
        "86400"
    );

    // 2. Actual GET request dengan Origin SvelteKit
    let get_req = Request::builder()
        .method(Method::GET)
        .uri("/test")
        .header(header::ORIGIN, "http://localhost:5173")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(get_req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .unwrap(),
        "http://localhost:5173"
    );
    assert_eq!(
        response
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_CREDENTIALS)
            .unwrap(),
        "true"
    );
}
