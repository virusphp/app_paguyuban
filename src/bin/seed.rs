use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@127.0.0.1:54322/db_paguyuban".to_string()
    });

    println!("🌱 Menjalankan Seeder Lingkungan Dev Paguyuban...");
    println!("📡 URL Target: {}", db_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Baca berkas seeder dari database/seeders/0001_initial_seed.sql
    println!("📄 Membaca berkas database/seeders/0001_initial_seed.sql...");
    let seed_sql = std::fs::read_to_string("database/seeders/0001_initial_seed.sql")?;

    let default_password = "password123";
    let password_hash = bcrypt::hash(default_password, bcrypt::DEFAULT_COST)?;
    println!("🔐 Generated valid bcrypt hash for dev: {}", password_hash);

    let final_sql = seed_sql.replace("__BCRYPT_PASSWORD_HASH__", &password_hash);

    // Jalankan seluruh statement dalam seeder
    sqlx::raw_sql(&final_sql).execute(&pool).await?;

    println!("\n✨ Seeding berhasil diterapkan ke database!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔑 Kredensial Login Default (Semua Sandi: password123)");
    println!("   1. Superadmin : 081111111111 (Akses /sa/...)");
    println!("   2. Ketua Org  : 081234567890 (Admin Paguyuban Kanfer 2)");
    println!("   3. Bendahara  : 081234567891 (Bendahara Paguyuban Kanfer 2)");
    println!("   4. Sekretaris : 081234567892 (Sekretaris Paguyuban Kanfer 2)");
    println!("   5. Ketua RT05 : 081300000001 (Admin RT 05 RW 09)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}
