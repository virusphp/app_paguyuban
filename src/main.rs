mod app;
mod routes;

use lumina::prelude::*;

//                  LUMINA FRAMEWORK APPLICATION ENTRYPOINT
// High-performance, elegant Rust web backend & API engine.

#[tokio::main]
async fn main() {
    // 1. Load configuration from .env file
    dotenv::dotenv().ok();

    // 2. Initialize the application engine with Web & API routes
    let app = Application::new()
        .with_web(routes::web::router())
        .with_api(routes::api::router());

    // 2b. Ensure database schema expansion (atribut kependudukan lengkap)
    app::services::ensure_schema_expanded().await;

    println!("⚡ Welcome to Lumina Framework");
    println!("🚀 Server running at http://127.0.0.1:8000");
    println!("📖 API Endpoints ready at http://127.0.0.1:8000/api");

    // 3. Launch HTTP server
    app.serve("127.0.0.1:8000").await;
}
