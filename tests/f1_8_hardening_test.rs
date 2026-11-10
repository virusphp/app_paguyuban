use app_paguyuban::app::services::{RateLimiter, ViewService};
use lumina::core::blade::BladeContext;
use std::path::Path;
use std::time::Duration;

#[tokio::test]
async fn test_rate_limiter_logic() {
    let limiter = RateLimiter::new();
    let key = "081299990001";
    let window = Duration::from_secs(60);
    let max_attempts = 3;

    // Awal: belum ada kegagalan, harus diizinkan
    assert!(limiter.check_limit(key, max_attempts, window).is_ok());

    // Percobaan 1 gagal
    limiter.record_failure(key, window);
    assert!(limiter.check_limit(key, max_attempts, window).is_ok());

    // Percobaan 2 gagal
    limiter.record_failure(key, window);
    assert!(limiter.check_limit(key, max_attempts, window).is_ok());

    // Percobaan 3 gagal -> sekarang sudah mencapai 3 kegagalan
    limiter.record_failure(key, window);
    let check_res = limiter.check_limit(key, max_attempts, window);
    assert!(
        check_res.is_err(),
        "Harus terkunci setelah mencapai max attempts"
    );
    let remaining = check_res.unwrap_err();
    assert!(remaining > 0 && remaining <= 60);

    // Kunci lain tidak boleh terpengaruh (isolasi per user/nomor HP)
    let other_key = "081299990002";
    assert!(limiter.check_limit(other_key, max_attempts, window).is_ok());

    // Reset setelah berhasil login
    limiter.reset(key);
    assert!(
        limiter.check_limit(key, max_attempts, window).is_ok(),
        "Setelah direset, key harus kembali diizinkan"
    );
}

#[tokio::test]
async fn test_render_error_views_native_blade() {
    let ctx = BladeContext::new();

    // 1. Uji View 404
    let html_404 = ViewService::render_blade("errors/404.blade.rs", &ctx);
    assert!(
        html_404.contains("404"),
        "Halaman 404 harus memuat kode status 404"
    );
    assert!(
        html_404.contains("Halaman Tidak Ditemukan"),
        "Halaman 404 harus memuat teks judul yang ramah"
    );
    assert!(
        html_404.contains("/dashboard"),
        "Halaman 404 harus menyediakan tautan kembali ke dashboard"
    );

    // 2. Uji View 403
    let html_403 = ViewService::render_blade("errors/403.blade.rs", &ctx);
    assert!(
        html_403.contains("403"),
        "Halaman 403 harus memuat kode status 403"
    );
    assert!(
        html_403.contains("Akses Ditolak"),
        "Halaman 403 harus memuat judul Akses Ditolak"
    );
    assert!(
        html_403.contains("/dashboard"),
        "Halaman 403 harus menyediakan tautan kembali ke dashboard"
    );

    // 3. Uji View 500
    let html_500 = ViewService::render_blade("errors/500.blade.rs", &ctx);
    assert!(
        html_500.contains("500"),
        "Halaman 500 harus memuat kode status 500"
    );
    assert!(
        html_500.contains("Kendala Sistem"),
        "Halaman 500 harus memuat judul kendala sistem"
    );
    assert!(
        html_500.contains("/dashboard"),
        "Halaman 500 harus menyediakan tautan kembali ke dashboard"
    );
}

#[test]
fn test_backup_and_restore_scripts_exist() {
    let sh_backup = Path::new("scripts/backup_db.sh");
    let ps_backup = Path::new("scripts/backup_db.ps1");
    let sh_restore = Path::new("scripts/restore_db.sh");
    let ps_restore = Path::new("scripts/restore_db.ps1");

    assert!(sh_backup.exists(), "scripts/backup_db.sh harus ada");
    assert!(ps_backup.exists(), "scripts/backup_db.ps1 harus ada");
    assert!(sh_restore.exists(), "scripts/restore_db.sh harus ada");
    assert!(ps_restore.exists(), "scripts/restore_db.ps1 harus ada");

    let sh_backup_content = std::fs::read_to_string(sh_backup).unwrap();
    assert!(
        sh_backup_content.contains("pg_dump"),
        "Skrip backup bash harus memuat instruksi pg_dump"
    );
    assert!(
        sh_backup_content.contains("RETENTION_DAYS"),
        "Skrip backup bash harus memuat rotasi retensi"
    );

    let ps_backup_content = std::fs::read_to_string(ps_backup).unwrap();
    assert!(
        ps_backup_content.contains("pg_dump"),
        "Skrip backup powershell harus memuat instruksi pg_dump"
    );
}
