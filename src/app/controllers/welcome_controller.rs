use lumina::prelude::*;
use serde_json::json;

//                        WELCOME CONTROLLER
// Controller halaman utama aplikasi Lumina.
// Mendukung Dual-Mode Response:
// 1. Browser Request (Accept: text/html) -> Mengembalikan Dashboard HTML modern ala Laravel.
// 2. API / cURL Request (Accept: application/json atau ?format=json) -> Mengembalikan JSON ala Lumen!

#[allow(dead_code)]
pub struct WelcomeController;

#[allow(dead_code)]
impl WelcomeController {
    /// Halaman Utama ("/") - Dual Mode Response
    pub async fn index(req: Request) -> impl IntoResponse {
        // Cek informasi koneksi database aktif
        let db_info = if let Some(pool) = &req.state.db {
            let driver = format!("{:?}", pool.kind);
            let is_ok = sqlx::query("SELECT 1").execute(&pool.pool).await.is_ok();
            (driver, if is_ok { "connected" } else { "disconnected" })
        } else {
            ("SQLite".to_string(), "not connected")
        };

        // 1. Jika request meminta JSON (via Header Accept: application/json atau ?format=json)
        // Mengembalikan spesifikasi framework ala Lumen microframework!
        let wants_json = req.wants_json()
            || req
                .header("accept")
                .map(|a| a.contains("json"))
                .unwrap_or(false);

        if wants_json {
            return req.json(json!({
                "name": "Lumina Framework",
                "tagline": "The High-Performance Rust Backend for Modern APIs",
                "version": "0.1.6",
                "rust_edition": "2021",
                "engine": "Axum 0.7 + Tokio Async",
                "environment": "local",
                "database": {
                    "driver": db_info.0,
                    "status": db_info.1
                },
                "documentation": "https://github.com/lumina-java/framework",
                "endpoints": {
                    "home": "/",
                    "health": "/health",
                    "api_index": "/api",
                    "api_health": "/api/health"
                }
            }));
        }

        // 2. Jika diakses via Web Browser biasa, render Blade HTML Dashboard
        Ok(req
            .view("welcome")
            .with("title", "Lumina Framework - High Performance Rust Backend")
            .with("app_name", "Lumina Framework")
            .with("version", "0.1.6")
            .with("rust_edition", "2021")
            .with("db_driver", &db_info.0)
            .with("db_status", db_info.1)
            .with("docs_url", "https://github.com/lumina-java/framework")
            .render(&req)
            .await
            .into_response())
    }

    /// Menampilkan Halaman Login ("/login")
    #[allow(dead_code)]
    pub async fn login(req: Request) -> impl IntoResponse {
        req.view("auth.login")
            .with("title", "Login - Lumina")
            .render(&req)
            .await
            .into_response()
    }

    /// Menampilkan Halaman Register ("/register")
    #[allow(dead_code)]
    pub async fn register(req: Request) -> impl IntoResponse {
        req.view("auth.register")
            .with("title", "Register - Lumina")
            .render(&req)
            .await
            .into_response()
    }
}
