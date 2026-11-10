use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use lumina::core::blade::BladeContext;
use lumina::prelude::Request;

use crate::app::services::{ApiResponse, ViewService};

pub struct ErrorController;

impl ErrorController {
    /// Handler untuk HTTP 404 Not Found (mendukung Web HTML & REST API JSON)
    pub async fn not_found(req: Request) -> Response {
        let is_api = req.uri.path().starts_with("/api/")
            || req
                .headers
                .get("accept")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.contains("application/json"))
                .unwrap_or(false);

        if is_api {
            ApiResponse::error(
                StatusCode::NOT_FOUND,
                &format!("Endpoint API '{}' tidak ditemukan.", req.uri.path()),
            )
        } else {
            let ctx = BladeContext::new();
            let html = ViewService::render_blade("errors/404.blade.rs", &ctx);
            (StatusCode::NOT_FOUND, Html(html)).into_response()
        }
    }

    /// Handler untuk HTTP 403 Forbidden (akses ditolak karena izin tidak cukup)
    pub async fn forbidden(req: Request) -> Response {
        let is_api = req.uri.path().starts_with("/api/")
            || req
                .headers
                .get("accept")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.contains("application/json"))
                .unwrap_or(false);

        if is_api {
            ApiResponse::error(
                StatusCode::FORBIDDEN,
                "Akses ditolak: Anda tidak memiliki wewenang untuk sumber daya ini.",
            )
        } else {
            let ctx = BladeContext::new();
            let html = ViewService::render_blade("errors/403.blade.rs", &ctx);
            (StatusCode::FORBIDDEN, Html(html)).into_response()
        }
    }

    /// Handler untuk HTTP 500 Internal Server Error
    pub async fn server_error(req: Request) -> Response {
        let is_api = req.uri.path().starts_with("/api/")
            || req
                .headers
                .get("accept")
                .and_then(|v| v.to_str().ok())
                .map(|v| v.contains("application/json"))
                .unwrap_or(false);

        if is_api {
            ApiResponse::error(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Terjadi kendala internal pada server. Silakan coba beberapa saat lagi.",
            )
        } else {
            let ctx = BladeContext::new();
            let html = ViewService::render_blade("errors/500.blade.rs", &ctx);
            (StatusCode::INTERNAL_SERVER_ERROR, Html(html)).into_response()
        }
    }
}
