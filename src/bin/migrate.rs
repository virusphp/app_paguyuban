use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
    });

    println!("⚡ Menjalankan migrasi database PostgreSQL...");
    println!("📡 URL Target: {}", db_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    println!("📦 Membaca direktori ./database/migrations...");
    sqlx::migrate!("./database/migrations").run(&pool).await?;

    println!("✅ Seluruh migrasi database berhasil diterapkan!");
    Ok(())
}
