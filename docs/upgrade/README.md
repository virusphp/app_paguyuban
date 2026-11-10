# Rencana & Status Migrasi Lumina Framework v0.3.0 (Native Blade Engine)

Dokumen ini mencatat ringkasan progres migrasi arsitektur dari engine template lama (Tera) ke **Pure Native Blade Engine** bawaan Lumina Framework v0.3.0, serta panduan langkah demi langkah untuk melanjutkan pekerjaan pada sesi berikutnya.

---

## 1. Ringkasan Upgrade Arsitektur

- **Framework**: `lumina-framework` v0.1.7 ➔ **v0.3.0** (menggunakan dependensi lokal di `D:/FRAMEWORK RUST/framework`).
- **Template Engine**: Crate `tera` telah dihapus 100% dari Lumina dan digantikan dengan **Native Blade Engine** murni Rust (`.blade.rs`).
- **Fitur Baru yang Diadopsi**:
  - Direktif Layout: `@extends('layouts/dashboard.blade.rs')`, `@section('content')`, `@yield('content')`.
  - Sub-views: `@include('components/sidebar.blade.rs')`.
  - Direktif Atribut & Kondisional: `@class(['is-active' => $active == '...'])`, `@selected(...)`, `@if`, `@foreach`, `@forelse`.
  - Form Helpers: `@csrf` otomatis menghasilkan input hidden token CSRF sesi aktif.
  - Pipeline Filter Native: `{{ $amount | rupiah }}` langsung memformat angka integer rupiah tanpa helper manual.
  - Context Engine: `lumina::core::blade::BladeContext`.

---

## 2. Status Progres Pekerjaan

| Tahap | Modul / Cakupan | Status | File yang Diubah / Dibuat |
|---|---|:---:|---|
| **Tahap 0** | **Integrasi Engine & Adapter** | **SELESAI** | [src/app/services/view_service.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/services/view_service.rs) (`ViewService::render_blade`) |
| **Tahap 1** | **Fondasi Layout, Auth & Superadmin** | **SELESAI** | • [resources/views/layouts/dashboard.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/layouts/dashboard.blade.rs)<br>• [resources/views/components/sidebar.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/components/sidebar.blade.rs)<br>• [resources/views/auth/login.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/auth/login.blade.rs)<br>• [resources/views/dashboard/index.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/dashboard/index.blade.rs)<br>• [resources/views/superadmin/organizations.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/superadmin/organizations.blade.rs)<br>• [resources/views/superadmin/org_create.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/superadmin/org_create.blade.rs)<br>• [resources/views/superadmin/audit_logs.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/superadmin/audit_logs.blade.rs)<br>• [src/app/controllers/auth_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/auth_controller.rs)<br>• [src/app/controllers/dashboard_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/dashboard_controller.rs)<br>• [src/app/controllers/superadmin_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/superadmin_controller.rs) |
| **Tahap 2** | **Modul Anggota & Rumah (F1.3)** | **SELESAI** | • [resources/views/members/index.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/members/index.blade.rs)<br>• [resources/views/members/form.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/members/form.blade.rs)<br>• [resources/views/members/import.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/members/import.blade.rs)<br>• [src/app/controllers/member_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/member_controller.rs) |
| **Tahap 3** | **Modul Iuran & Tagihan (F1.4)** | **SELESAI** | • [resources/views/fees/index.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/fees/index.blade.rs)<br>• [resources/views/fees/tariffs.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/fees/tariffs.blade.rs)<br>• [resources/views/fees/generate.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/fees/generate.blade.rs)<br>• [resources/views/fees/pay.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/fees/pay.blade.rs)<br>• [src/app/controllers/fee_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/fee_controller.rs) |
| **Tahap 4** | **Modul Kas & Generator LPJ (F1.5)** | **SELESAI** | • [resources/views/cash/index.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/cash/index.blade.rs)<br>• [resources/views/cash/form_in.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/cash/form_in.blade.rs)<br>• [resources/views/cash/form_out.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/cash/form_out.blade.rs)<br>• [resources/views/cash/form_inkind.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/cash/form_inkind.blade.rs)<br>• [resources/views/cash/lpj_preview.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/cash/lpj_preview.blade.rs)<br>• [src/app/controllers/cash_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/cash_controller.rs) |
| **Tahap 5** | **Modul Kegiatan & Info Penting (F1.6)** | **SELESAI** | • [resources/views/events/index.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/events/index.blade.rs)<br>• [resources/views/events/form.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/events/form.blade.rs)<br>• [resources/views/events/detail.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/events/detail.blade.rs)<br>• [resources/views/announcements/index.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/announcements/index.blade.rs)<br>• [resources/views/announcements/form.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/announcements/form.blade.rs)<br>• [src/app/controllers/event_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/event_controller.rs)<br>• [src/app/controllers/announcement_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/announcement_controller.rs) |
| **Tahap 6** | **Pembersihan & Pelepasan Crate Tera** | **SELESAI** | 25 file `.html` dihapus, `tera = "1.19"` dilepas dari `Cargo.toml`, [ViewService](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/services/view_service.rs) 100% native Blade, test suite & clippy 100% lulus bersih |

---

## 3. Rincian Checklist Tahapan Selanjutnya

### Tahap 2: Modul Anggota & Rumah (F1.3) — Selesai
- [x] Buat `resources/views/members/index.blade.rs` (Daftar warga, filter pencarian & status, chip status kependudukan).
- [x] Buat `resources/views/members/form.blade.rs` (Form tambah/ubah data warga dan penautan rumah `households`).
- [x] Buat `resources/views/members/import.blade.rs` (Panduan dan form impor massal CSV).
- [x] Perbarui [src/app/controllers/member_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/member_controller.rs) dari `tera::Context` ke `BladeContext` dan gunakan `render_blade`.
- [x] Jalankan dan verifikasi suite pengujian: `cargo test --test f1_3_members_test`.

### Tahap 3: Modul Iuran & Tagihan (F1.4) — Selesai
- [x] Buat `resources/views/fees/index.blade.rs` (Matriks 12 bulan berpetak hijau/oranye/abu-abu).
- [x] Buat `resources/views/fees/tariffs.blade.rs` (Daftar tarif pos iuran paguyuban).
- [x] Buat `resources/views/fees/generate.blade.rs` (Form penerbitan tagihan massal).
- [x] Buat `resources/views/fees/pay.blade.rs` (Form pelunasan transaksi atomik).
- [x] Manfaatkan filter `{{ $nominal | rupiah }}` untuk seluruh nilai mata uang.
- [x] Perbarui [src/app/controllers/fee_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/fee_controller.rs).
- [x] Jalankan dan verifikasi: `cargo test --test f1_4_fees_test`.

### Tahap 4: Modul Kas & Generator LPJ (F1.5) — Selesai
- [x] Buat `resources/views/cash/index.blade.rs` (Buku kas umum & running balance).
- [x] Buat `resources/views/cash/form_in.blade.rs`, `form_out.blade.rs`, `form_inkind.blade.rs`.
- [x] Buat `resources/views/cash/lpj_preview.blade.rs` (Lembar formal cetak LPJ Paguyuban Kanfer 2).
- [x] Perbarui [src/app/controllers/cash_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/cash_controller.rs).
- [x] Jalankan dan verifikasi: `cargo test --offline --test f1_5_cash_lpj_test -- test_render_cash_views_native_blade`.

### Tahap 5: Modul Kegiatan & Info Penting (F1.6) — Selesai
- [x] Buat `resources/views/events/index.blade.rs`, `form.blade.rs`, `detail.blade.rs`.
- [x] Buat `resources/views/announcements/index.blade.rs`, `form.blade.rs`.
- [x] Perbarui `EventController` dan `AnnouncementController` ke `BladeContext` bawaan Lumina via `ViewService::render_blade`.
- [x] Jalankan dan verifikasi: `cargo test --offline --test f1_6_events_info_test -- test_render_events_and_announcements_views_native_blade`.

### Tahap 6: Finalisasi & Pembersihan Dependensi — Selesai
- [x] Hapus seluruh file template `.html` lama (25 file) dari `resources/views/`.
- [x] Hapus dependensi `tera = "1.19"` dari [Cargo.toml](file:///d:/PROJECT%20LUMINA/app_paguyuban/Cargo.toml).
- [x] Bersihkan [src/app/services/view_service.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/services/view_service.rs) dari import dan fungsi-fungsi usang Tera.
- [x] Perbarui seluruh test suite untuk pengujian langsung ke Native Blade Engine Lumina v0.6.1.
- [x] Verifikasi seluruh kualitas kode: `cargo check --offline`, `cargo clippy --offline`, `cargo fmt --check`, dan `cargo test --offline` berjalan 100% bersih tanpa warning dan tanpa error.

---

## 4. Status Akhir Upgrade & Langkah Pengembangan Selanjutnya

Seluruh proses upgrade framework ke **Lumina v0.6.1** dan migrasi template engine ke **Pure Native Blade Engine** (`.blade.rs`) dari **Tahap 0 hingga Tahap 6 telah TUNTAS 100%**.

### Perintah Verifikasi Rutin
```bash
# 1. Pastikan kompilasi bersih dari warning dan error
cargo check --offline
cargo clippy --offline

# 2. Pastikan gaya kode rapi
cargo fmt --check

# 3. Jalankan unit test perenderan Native Blade
cargo test --offline --test f1_1_foundation_test -- test_render_login_view_native_blade
cargo test --offline --test f1_2_superadmin_test -- test_render_superadmin_views_native_blade
cargo test --offline --test f1_3_members_test -- test_render_member_views_native_blade
cargo test --offline --test f1_4_fees_test -- test_render_fee_views_native_blade
cargo test --offline --test f1_5_cash_lpj_test -- test_render_cash_views_native_blade
cargo test --offline --test f1_6_events_info_test -- test_render_events_and_announcements_views_native_blade
```

### Langkah Selanjutnya Sesuai Roadmap (`PLAN.md`):
1. **Fitur F1.7: Modul Surat & Undangan Warga (SELESAI 100%)**
   - Rute: `/surat`, `/surat/baru`, `/surat/template`, `/surat/:id`, `/surat/:id/edit`, `/surat/:id/kirim`, `/surat/:id/hapus`, `/surat/:id/pdf`.
   - Template surat & undangan otomatis, penomoran urut resmi (`<no>/<kode>/<bulan>/<tahun>`), ekspor PDF surat via `printpdf`.
   - 4 template Native Blade: `letters/index.blade.rs`, `letters/form.blade.rs`, `letters/detail.blade.rs`, `letters/templates.blade.rs`.
   - Lolos seluruh pengujian unit/integrasi (`tests/f1_7_letters_test.rs`).
2. **Fitur F1.8: Pengerasan Sistem (Hardening) & Kesiapan Produksi (SELESAI 100%)**
   - Backup otomatis database PostgreSQL (`scripts/backup_db.sh` & `.ps1`, `scripts/restore_db.sh` & `.ps1`).
   - Rate limiting dan pencegahan brute-force login di [src/app/services/rate_limiter.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/services/rate_limiter.rs) dan [AuthController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/auth_controller.rs).
   - Halaman error ramah pengguna Native Blade: `errors/404.blade.rs`, `errors/403.blade.rs`, `errors/500.blade.rs` dan fallback routing Axum/Lumina.
   - Panduan pengurus paguyuban di [docs/PANDUAN_PENGURUS.md](file:///d:/PROJECT%20LUMINA/app_paguyuban/docs/PANDUAN_PENGURUS.md).
   - Lolos seluruh pengujian otomatis (`tests/f1_8_hardening_test.rs`).

---

## 5. Kesimpulan Kesiapan Fase 1

Seluruh fitur **Fase 0 (Spike & Setup)** dan **Fase 1 (Dashboard PC Pengurus: F1.1 s.d. F1.8)** telah **SELESAI 100%** dengan:
- **Zero compiler & clippy warnings** (`cargo check --offline`, `cargo clippy --offline`).
- **Zero formatting discrepancies** (`cargo fmt --check`).
- **100% test passing** pada seluruh suite test unit & integrasi.
- Seluruh template menggunakan **Pure Native Blade Engine (`.blade.rs`)** tanpa dependensi Tera.
- Siap melangkah ke **Fase 2: Aplikasi Mobile Anggota (PWA SvelteKit) & API JSON**.
