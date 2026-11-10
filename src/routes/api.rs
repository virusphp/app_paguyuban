use crate::app::controllers::{
    AnnouncementApiController, AuthApiController, CashApiController, EventApiController,
    FeeApiController, LetterApiController, MemberApiController,
};
use lumina::prelude::*;
use serde_json::json;

//                            API ROUTING
// Rute di sini otomatis diprefix dengan "/api" oleh Application engine.
// Sangat ideal untuk membangun RESTful API, Microservices, dan Webhooks.

pub fn router() -> Router<AppState> {
    Router::new()
        .get("/", api_index)
        .get("/health", api_health)
        .get("/v1", api_v1_index)
        // API Mobile SvelteKit PWA — Autentikasi & Sesi Warga (F2.1)
        .post("/v1/auth/login", AuthApiController::login)
        .post("/v1/auth/logout", AuthApiController::logout)
        .get("/v1/auth/me", AuthApiController::me)
        .post("/v1/auth/switch-org", AuthApiController::switch_org)
        // API Mobile SvelteKit PWA — Profil & Anggota (F1.3)
        .get("/v1/me", MemberApiController::get_me)
        .put("/v1/me", MemberApiController::update_me)
        .get("/v1/members", MemberApiController::list_members)
        // API Mobile SvelteKit PWA — Iuran & Transparansi Lingkungan (F1.4)
        .get("/v1/iuran", FeeApiController::my_invoices)
        .get("/v1/iuran/ringkasan", FeeApiController::public_summary)
        // API Mobile SvelteKit PWA — Ringkasan Saldo Kas Umum (F1.5)
        .get("/v1/kas/ringkasan", CashApiController::summary)
        // API Mobile SvelteKit PWA — Agenda Kegiatan & RSVP (F1.6)
        .get("/v1/kegiatan", EventApiController::list_events)
        .post("/v1/kegiatan/:id/rsvp", EventApiController::send_rsvp)
        // API Mobile SvelteKit PWA — Papan Informasi & Pengumuman (F1.6)
        .get("/v1/info", AnnouncementApiController::list_announcements)
        // API Mobile SvelteKit PWA — Surat & Undangan Warga (F2.1.2)
        .get("/v1/surat", LetterApiController::list_letters)
        .get("/v1/surat/:id", LetterApiController::get_letter)
        .get("/v1/surat/:id/pdf", LetterApiController::export_pdf)
        // Fallback untuk endpoint API tak terdaftar & Middleware CORS (F2.1.3)
        .fallback(api_fallback_not_found)
        .layer(axum::middleware::from_fn(
            crate::app::services::cors_middleware,
        ))
}

/// Fallback respons JSON jika rute API tidak ditemukan
async fn api_fallback_not_found() -> impl IntoResponse {
    crate::app::services::ApiResponse::error(
        axum::http::StatusCode::NOT_FOUND,
        "Endpoint API tidak ditemukan",
    )
}

/// Endpoint: GET /api
async fn api_index() -> impl IntoResponse {
    Json(json!({
        "framework": "Lumina Framework",
        "tagline": "The High-Performance Rust Backend for Modern APIs",
        "version": "0.1.6",
        "status": "online",
        "endpoints": {
            "health": "/api/health",
            "v1": "/api/v1"
        }
    }))
}

/// Endpoint: GET /api/health
async fn api_health(State(state): State<AppState>) -> impl IntoResponse {
    let db_status = if let Some(pool) = &state.db {
        if sqlx::query("SELECT 1").execute(&pool.pool).await.is_ok() {
            "connected"
        } else {
            "error"
        }
    } else {
        "disabled"
    };

    Json(json!({
        "status": "ok",
        "database": db_status,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Endpoint: GET /api/v1
async fn api_v1_index() -> impl IntoResponse {
    Json(json!({
        "version": "v1",
        "message": "Lumina API v1 is ready. Register your controllers in src/routes/api.rs!"
    }))
}
