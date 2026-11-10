# Catatan Perubahan (CHANGELOG)

Semua perubahan pada proyek dicatat secara singkat di sini sesuai aturan `AGENTS.md`.

## [2026-11-10] - Otomatisasi Penerbitan Tagihan Iuran Warga & Sinkronisasi Bulan Berjalan

### Ditambahkan & Ditingkatkan
- **Layanan Sinkronisasi Iuran (`FeeService`)**:
  - Dibuat di [src/app/services/fee_service.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/services/fee_service.rs):
    - `sync_invoices_for_member`: Otomatis menerbitkan tagihan bulanan dari awal tarif (`active_from` Januari) atau tanggal bergabung warga hingga bulan kalender berjalan saat ini.
    - `sync_invoices_for_all_active_members`: Melakukan sinkronisasi tagihan untuk seluruh warga aktif di suatu organisasi secara idempoten (`ON CONFLICT DO NOTHING`).
    - `generate_invoices_batch`: Menerbitkan tagihan secara batch dalam rentang bulan tertentu.
- **Hook Otomatis pada Manajemen Warga (`MemberController`)**:
  - Pendaftaran warga baru (`POST /anggota/baru`): Langsung otomatis menerbitkan tagihan dari awal berlakunya tarif s/d bulan saat ini.
  - Perubahan data warga (`POST /anggota/:id` & `POST /anggota/:id/status`): Otomatis memvalidasi dan menerbitkan tagihan ketika warga berstatus `aktif`.
  - Impor massal CSV (`POST /anggota/impor`): Otomatis menerbitkan tagihan untuk seluruh warga aktif yang baru diimpor.
- **Peningkatan Formulir Terbitkan Tagihan (`/iuran/generate`)**:
  - Menambahkan pilihan cakupan periode di [resources/views/fees/generate.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/fees/generate.blade.rs):
    1. *Otomatis Hingga Bulan Berjalan Saat Ini* (Januari s/d Bulan Ini) [Direkomendasikan].
    2. *Hanya 1 Bulan Tertentu Saja*.
    3. *Satu Tahun Penuh Sekaligus* (12 Bulan / Januari s/d Desember).
- **Tombol Aksi Cepat Sinkronisasi di Rekap Iuran (`/iuran`)**:
  - Menambahkan tombol form `⚡ Sinkronkan Tagihan Bulan Ini` pada toolbar [resources/views/fees/index.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/fees/index.blade.rs) dengan endpoint `POST /iuran/sinkronkan` di [FeeController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/fee_controller.rs) dan rute [src/routes/web.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/web.rs).


## [2026-11-10] - Audit Komprehensif Antarmuka Responsif PC & Smartphone (Mobile)

### Diperbaiki & Ditingkatkan
- **Perbaikan Kartu Data Keanggotaan/Warga (`/anggota`)**:
  - Memperbaiki kartu statistik ringkasan warga di [resources/views/members/index.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/members/index.blade.rs) yang sebelumnya pecah akibat inline grid tidak lengkap dan class yang belum terdefinisi.
  - Mengintegrasikan kelas baku design system `.pg-card`, `.pg-stat-card`, dan `.pg-grid-3`.
- **Audit & Penyelarasan Layout Seluruh Menu Web Dashboard**:
  - **Navigasi Drawer Mobile**: Menambahkan tombol hamburger `☰` pada topbar mobile, backdrop overlay, tombol close `✕` di header sidebar, serta script drawer toggle otomatis di [layouts/dashboard.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/layouts/dashboard.blade.rs) & [components/sidebar.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/components/sidebar.blade.rs).
  - **Tabel Responsif**: Memastikan seluruh tabel (Buku Kas, Donasi Barang, LPJ, Matriks Iuran 12 Bulan, Tarif, Agenda Acara, RSVP Warga, Surat Masuk/Keluar, Audit Logs, Data Organisasi, Pengguna) dibungkus dengan wrapper kontainer `.pg-table-wrap` / `.pg-table-responsive` sehingga dapat digulir horizontal mulus tanpa merusak viewport smartphone.
  - **Grid Adaptif**: Menghapus seluruh inline `grid-template-columns: repeat(...)` kaku di seluruh view Blade, menggantinya dengan kelas CSS responsif `.pg-grid-4`, `.pg-grid-3`, `.pg-stats-grid`, dan `.pg-grid-2` yang bertransformasi otomatis ke 2 kolom di tablet (max 992px) dan 1 kolom di smartphone (max 640px).
  - **Penyelarasan Formulir & Tombol Aksi**:
    - Menambahkan kelas `.pg-form-actions` di [public/css/components.css](file:///d:/PROJECT%20LUMINA/app_paguyuban/public/css/components.css) yang otomatis menyusun tombol submit dan batal secara vertikal (full-width) di smartphone dengan `flex-direction: column-reverse`.
    - Menerapkan `.pg-form-actions` dan `flex-wrap: wrap` pada seluruh modul form (`members/form`, `members/import`, `cash/form_in`, `cash/form_out`, `cash/form_inkind`, `fees/pay`, `fees/generate`, `events/form`, `letters/form`, `announcements/form`, `superadmin/org_create`).
  - **Pratinjau Surat Resmi Responsif**:
    - Menambahkan `.pg-letter-sheet`, `.pg-letter-header`, dan `.pg-letter-signatures` di [resources/views/letters/detail.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/letters/detail.blade.rs) untuk memastikan lembar kop surat dan tanda tangan pengurus tetap proporsional dan tidak terpotong di layar sentuh ponsel.
- **Verifikasi Kualitas Kode**:
  - `cargo fmt --check`: 100% rapi dan konsisten.
  - `cargo clippy --offline`: 100% bersih tanpa warning.
  - `dashboard_health_test`: 3/3 pengujian lolos validasi rendering.
  - `f2_1_api_test`: 9/9 pengujian lolos validasi endpoint & middleware REST API.


## [2026-11-08] - Peningkatan & Perbaikan Dashboard PC Pengurus & Navigasi Sistem

### Diperbaiki & Ditingkatkan
- **Penyempurnaan Halaman Ringkasan Dashboard (`/dashboard`)**:
  - Mengoreksi tautan tombol aksi yang sebelumnya mengarah ke route tidak valid `/kas/baru` menjadi aksi terarah `/kas/masuk` dan `/kas/keluar`.
  - Mengimplementasikan tata letak 2 kolom (1.6fr / 1fr) sesuai standar Pola Halaman A pada Design System Lumina:
    - Menampilkan tabel 5 Transaksi Kas Terbaru lengkap dengan kategori, jenis mutasi, tanggal, dan format rupiah.
    - Menampilkan progress bar dan rekapitulasi realisasi iuran warga.
    - Menambahkan kartu **Pintasan Aksi Pengurus** (Tambah Warga, Terbitkan Tagihan, Kas Masuk, Kas Keluar, Buat Agenda, Registrasi Surat).
    - Menampilkan kartu **Agenda Kegiatan Mendatang** untuk memonitor acara terdekat lingkungan.
- **Implementasi Modul Pengaturan Lingkungan (`/pengaturan`)**:
  - Menambahkan endpoint `GET /pengaturan` dan `POST /pengaturan` di [OrgController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/org_controller.rs) serta template [resources/views/org/settings.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/org/settings.blade.rs).
  - Menyajikan informasi profil lingkungan, jenis organisasi, status aktif, tanggal pendaftaran, serta daftar susunan pengurus paguyuban (Ketua, Sekretaris, Bendahara) beserta nomor kontak.
- **Implementasi Modul Manajemen Pengguna Superadmin (`/sa/pengguna`)**:
  - Menambahkan endpoint `GET /sa/pengguna` di [SuperadminController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/superadmin_controller.rs) dan template [resources/views/superadmin/users.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/superadmin/users.blade.rs).
  - Menyajikan daftar seluruh akun pengguna sistem, nomor kontak login, hak akses (Superadmin / Pengguna), asosiasi organisasi & peran, status, serta waktu login terakhir.
- **Penyempurnaan Navigasi Sidebar**:
  - Memperbaiki [components/sidebar.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/components/sidebar.blade.rs) sehingga menu `Pengaturan` dan `Pengguna` (Superadmin) memiliki rute aktif yang valid dan bebas broken link.
- **Pengujian Otomatis Kesehatan Antarmuka**:
  - Menambahkan suite pengujian [tests/dashboard_health_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/dashboard_health_test.rs) untuk memvalidasi rendering Blade Native pada Dashboard Ringkasan, Pengaturan Organisasi, dan Pengguna Superadmin (3 pengujian lolos 100%).

## [2026-11-08] - Fitur F2.1.3 & F2.1.4: Middleware CORS, API Error Formatting & Suite Pengujian RESTful API Selesai

### Ditambahkan
- Middleware CORS di [src/app/services/cors_middleware.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/services/cors_middleware.rs):
  - Menangani request preflight HTTP `OPTIONS` dengan status 204 No Content.
  - Mengatur header respons `Access-Control-Allow-Origin`, `Access-Control-Allow-Methods`, `Access-Control-Allow-Headers` (`Authorization, Content-Type, Accept, X-Requested-With, X-Org-Id`), `Access-Control-Allow-Credentials: true`, dan `Access-Control-Max-Age: 86400`.
  - Mengintegrasikan layer middleware CORS pada seluruh route API di [src/routes/api.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/api.rs).
- Standardisasi Respons API di [src/app/services/api_response.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/services/api_response.rs):
  - Helper respons sukses data: `ApiResponse::success(data)`.
  - Helper respons sukses pesan: `ApiResponse::success_msg(msg)`.
  - Helper respons error: `ApiResponse::error(status, msg)`.
- Fallback Endpoint API 404 JSON di [src/routes/api.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/api.rs):
  - Mencegah error `JSON.parse` pada frontend mobile SvelteKit PWA ketika mengakses endpoint tak terdaftar.
- Suite Pengujian Otomatis RESTful API v1 Lengkap di [tests/f2_1_api_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f2_1_api_test.rs):
  - Menambahkan pengujian `test_cors_middleware_preflight_and_headers` (memvalidasi preflight OPTIONS dan header CORS origin).
  - Menambahkan pengujian `test_api_response_formatting_helpers` (memvalidasi format JSON data, pesan, dan error).
  - Menyelaraskan seluruh test suite API (9 pengujian lolos 100%).

## [2026-11-08] - Fitur F2.1.2: Endpoint REST API Surat & Undangan Warga Selesai

### Ditambahkan
- Controller REST API Surat & Undangan di [src/app/controllers/letter_api_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/letter_api_controller.rs):
  - Endpoint `GET /api/v1/surat`: Mengambil daftar korespondensi surat resmi dan undangan warga dengan filter jenis (`kind=surat|undangan`) dan pencarian (`q=...`).
  - Endpoint `GET /api/v1/surat/:id`: Mengambil rincian detail isi surat, metadata nomor surat, perihal, penerima, dan tanggal pengesahan.
  - Endpoint `GET /api/v1/surat/:id/pdf`: Menghasilkan dan mengunduh berkas biner dokumen PDF resmi ukuran A4 lengkap dengan kop surat paguyuban/RT langsung ke perangkat smartphone warga.
- Penegakan Aturan Hak Akses & Privasi Dokumen:
  - Warga biasa (`role = 'anggota'` atau peran non-sekretaris) **hanya diizinkan melihat surat yang telah berstatus `'terkirim'`**. Surat draft internal pengurus otomatis disaring dari respons API dan diproteksi dengan kode status HTTP 403 / 404.
  - Pengurus (`sekretaris`, `admin`, `superadmin`) tetap dapat meninjau seluruh status surat melalui API.
- Rute-rute API di [src/routes/api.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/api.rs):
  - Mendaftarkan rute `GET /v1/surat`, `GET /v1/surat/:id`, dan `GET /v1/surat/:id/pdf`.
- Suite Pengujian Otomatis REST API di [tests/f2_1_api_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f2_1_api_test.rs):
  - `test_letter_api_pdf_export_binary`: Memverifikasi keluaran biner PDF valid berformat `%PDF-1.x`.
  - `test_letter_api_access_control_rules`: Memvalidasi pembatasan hak akses bahwa warga biasa dilarang mengakses draft surat, sedangkan sekretaris dan ketua memiliki hak akses penuh.
  - `test_letter_api_dto_serialization`: Memvalidasi serialisasi struktur JSON `ApiLetterSummaryDto` dan `ApiLetterDetailDto`.



## [2026-11-07] - Pembaruan Rute: Base URL Root ("/") Mengarah Langsung ke Login / Dashboard

### Diubah
- Rute Web di [src/routes/web.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/web.rs):
  - Mengarahkan `GET /` dan `POST /` langsung ke `AuthController::show_login` dan `AuthController::login`.
  - Jika pengguna belum terautentikasi, peramban langsung menampilkan formulir Masuk Pengurus Paguyuban.
  - Jika sesi login telah aktif, sistem secara otomatis mengalihkan pengguna ke `/dashboard`.
  - Rute `/login` tetap dipertahankan untuk kompatibilitas tautan langsung dan redirect form.

## [2026-11-07] - Fitur F2.1.1: Autentikasi REST API Mobile Warga & JWT Bearer Token Selesai

### Ditambahkan
- Controller Autentikasi REST API di [src/app/controllers/auth_api_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/auth_api_controller.rs):
  - Endpoint `POST /api/v1/auth/login`: Autentikasi warga berbasis nomor ponsel dan kata sandi dengan validasi Rate Limiting (maksimal 5 percobaan gagal per 15 menit), verifikasi bcrypt, pengambilan status aktif, resolusi keanggotaan organisasi, pencatatan `audit_logs`, dan penerbitan token JWT Bearer 30 hari.
  - Endpoint `GET /api/v1/auth/me`: Mengambil data profil warga, peran, dan organisasi aktif pengguna.
  - Endpoint `POST /api/v1/auth/switch-org`: Mengganti konteks organisasi aktif bagi warga/pengurus yang tergabung di banyak organisasi (misal Paguyuban & RT) dan menerbitkan token berlingkup baru.
  - Endpoint `POST /api/v1/auth/logout`: Mencabut dan mendaftarkan token JWT ke blacklist pencabutan server (*instant token revocation*).
- Peningkatan [AuthContext](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/services/auth_guard.rs) dengan Arsitektur Autentikasi Hybrid:
  - Ekstraksi otomatis header `Authorization: Bearer <token>` untuk mobile client PWA dan REST API.
  - Verifikasi validitas tanda tangan kriptografi JWT dan pengecekan blacklist `TokenBlacklist::is_revoked`.
  - Ekstraksi otomatis hak akses `org_id`, `org_name`, dan peran dari token claims atau header `X-Org-Id`.
  - Dukungan fallback ke sesi cookie web jika header Authorization tidak disertakan.
  - Penanganan error API yang ramah: mengembalikan response JSON HTTP 401 Unauthorized / 403 Forbidden alih-alih redirect web.
- Rute-rute API di [src/routes/api.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/api.rs):
  - `/v1/auth/login`, `/v1/auth/logout`, `/v1/auth/me`, `/v1/auth/switch-org`.
- Suite Pengujian Otomatis REST API di [tests/f2_1_api_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f2_1_api_test.rs):
  - `test_auth_api_jwt_token_generation_and_validation`: Memvalidasi penerbitan, klaim sub UUID, ekstraksi `org_id` dan `org_name`.
  - `test_auth_api_token_revocation_blacklist`: Memvalidasi mekanisme pencabutan token logout.
  - `test_auth_api_rate_limiter_integration`: Memvalidasi penguncian brute force pada login API.
  - `test_auth_context_role_and_org_helpers`: Memvalidasi helper otorisasi `require_org` dan `require_role`.



## [2026-11-07] - Penyelarasan Struktur Framework Lumina v0.6.1 & Pemasangan CLI lumina clean

### Ditambahkan
- Pemasangan dan pemutakhiran Lumina CLI ke versi resmi terbaru **v0.6.1** di biner global `~/.cargo/bin/lumina.exe` (dikompilasi langsung dari repositori framework Lumina lokal).
- Skrip pembungkus CLI lokal di root proyek:
  - [lumina.bat](file:///d:/PROJECT%20LUMINA/app_paguyuban/lumina.bat): Wrapper batch Windows yang memanggil `lumina.exe %*` secara eksplisit guna menghindari *recursive execution trap* Windows CMD.
  - [lumina](file:///d:/PROJECT%20LUMINA/app_paguyuban/lumina): Wrapper Bash Unix/Linux (`lumina "$@"`).
- Penyelarasan folder dan berkas placeholder kerangka resmi Lumina v0.6.1:
  - `storage/logs/.gitkeep` & `storage/cache/.gitkeep`: Direktori log berkas dan penampung cache framework.
  - `database/factories/.gitkeep`: Direktori pembuat data model tiruan untuk pengujian otomatis.
  - `plugins/.gitkeep`: Direktori plugin ekstensi Lumina.
  - `src/core/mod.rs`: Modul internal inti aplikasi.
- Konfigurasi profil build release di [Cargo.toml](file:///d:/PROJECT%20LUMINA/app_paguyuban/Cargo.toml) (`opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`, `strip = true`) dan dependensi `validator = { version = "0.18", features = ["derive"] }`.

### Dioptimalkan
- Eksekusi pembersihan total cache compiler dan data usang dengan perintah `lumina clean`:
  - Berhasil membersihkan **36.93 GiB** cache usang `target/debug/incremental/` tanpa menghapus pre-compiled crate dependencies, mengembalikan ruang penyimpanan dan menjaga kecepatan kompilasi.
- Seluruh suite pengujian lolos 100%, `cargo check`, `cargo fmt`, dan `cargo clippy` bersih tanpa warning.



## [2026-11-07] - Fitur F1.8: Pengerasan Sistem (Hardening) & Kesiapan Produksi Selesai

### Ditambahkan
- Modul Layanan Pelindung Percobaan Masuk (Rate Limiter) di [src/app/services/rate_limiter.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/services/rate_limiter.rs):
  - Sistem sliding-window thread-safe untuk membatasi percobaan masuk yang gagal (maksimal 5 kali kegagalan per 15 menit per nomor HP).
  - Integrasi ke [AuthController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/auth_controller.rs) dengan pesan kunci penundaan otomatis dan reset riwayat percobaan jika autentikasi berhasil.
  - Mempertahankan pesan kesalahan generik netral untuk mencegah pelacakan nomor ponsel aktif (*anti-user enumeration*).
- Halaman Error Ramah Pengguna Berbasis Native Blade di `resources/views/errors/`:
  - [404.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/errors/404.blade.rs): Tampilan Halaman Tidak Ditemukan dengan tautan kembali ke Dashboard atau riwayat peramban sebelumnya.
  - [403.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/errors/403.blade.rs): Tampilan Akses Ditolak bagi pengguna yang tidak memiliki wewenang atau peran yang memadai.
  - [500.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/errors/500.blade.rs): Tampilan Kendala Sistem Sementara dengan opsi muat ulang atau navigasi ke Dashboard.
- Controller Error di [src/app/controllers/error_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/error_controller.rs) dan integrasi rute fallback global pada [src/routes/web.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/web.rs) serta penambahan method `.fallback(handler)` pada Lumina Framework Router.
- Skrip Cadangan (Backup) & Pemulihan (Restore) Otomatis:
  - [scripts/backup_db.sh](file:///d:/PROJECT%20LUMINA/app_paguyuban/scripts/backup_db.sh): Skrip Bash Linux/Docker dengan `pg_dump`, kompresi `gzip`, dan rotasi penghapusan arsip berusia lebih dari 30 hari.
  - [scripts/backup_db.ps1](file:///d:/PROJECT%20LUMINA/app_paguyuban/scripts/backup_db.ps1): Skrip PowerShell untuk lingkungan Windows dengan penanganan Docker/lokal dan rotasi retensi.
  - [scripts/restore_db.sh](file:///d:/PROJECT%20LUMINA/app_paguyuban/scripts/restore_db.sh) & [scripts/restore_db.ps1](file:///d:/PROJECT%20LUMINA/app_paguyuban/scripts/restore_db.ps1): Skrip latihan pemulihan database berkala (*quarterly recovery drill*).
  - [.gitignore](file:///d:/PROJECT%20LUMINA/app_paguyuban/.gitignore): Mencegah berkas arsip `backups/*.gz` dan kredensial ter-commit ke repositori git.
- Buku Panduan Operasional Pengurus di [docs/PANDUAN_PENGURUS.md](file:///d:/PROJECT%20LUMINA/app_paguyuban/docs/PANDUAN_PENGURUS.md):
  - Penjelasan matriks hak akses peran pengurus (Ketua, Sekretaris, Bendahara, Superadmin).
  - Panduan langkah demi langkah siklus data warga, penerbitan tagihan & pelunasan iuran, pembukuan kas & cetak LPJ, agenda kegiatan & papan informasi, hingga korespondensi surat resmi berkop.
  - Panduan prosedur cadangan dan pemulihan rutin basis data.
- Suite Pengujian Otomatis Pengerasan di [tests/f1_8_hardening_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f1_8_hardening_test.rs):
  - `test_rate_limiter_logic`: Memvalidasi batas 3 percobaan, durasi sisa kunci, isolasi key antar pengguna, dan mekanisme reset.
  - `test_render_error_views_native_blade`: Memverifikasi perenderan 3 view error (404, 403, 500) menggunakan Native Blade Engine.
  - `test_backup_and_restore_scripts_exist`: Memverifikasi ketersediaan dan validitas instruksi backup/restore.

### Diubah
- [PLAN.md](file:///d:/PROJECT%20LUMINA/app_paguyuban/PLAN.md): Menandai seluruh tugas Fase 1 (**F1.1 s.d. F1.8**) berstatus selesai (`[x]`).


## [2026-11-06] - Fitur F1.7: Modul Surat & Undangan Warga (Native Blade & PDF) Selesai

### Ditambahkan
- Modul Generator Dokumen PDF Surat Resmi di [src/app/services/letter_pdf_service.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/services/letter_pdf_service.rs):
  - Menggunakan crate `printpdf` untuk menyusun lembar surat ukuran A4 standar.
  - Memuat kop surat resmi organisasi (nama organisasi, alamat/wilayah, garis pemisah kop ganda tebal-tipis).
  - Metadata surat: nomor surat, lampiran, perihal, tanggal terbit, dan tujuan penerima.
  - Pembuka, isi badan surat, paragraf penutup, serta blok tanda tangan pengurus pengesah (Ketua/Sekretaris).
- Controller Surat & Undangan di [src/app/controllers/letter_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/letter_controller.rs):
  - Handler `index`: Daftar seluruh korespondensi dengan tab filter (Semua, Surat Resmi, Undangan Warga) dan kartu metrik statistik.
  - Handler `show_create` & `create`: Pembuatan surat baru dengan generator nomor urut otomatis berstandar `<urut>/<kode_org>/<bulan_romawi>/<tahun>` (contoh: `001/PGY-KNF2/XI/2026`).
  - Handler `show_detail`: Pratinjau surat resmi, ringkasan metadata, dan status pengiriman (Draft vs Terkirim).
  - Handler `show_edit` & `update`: Pengubahan data surat dan pembaruan riwayat.
  - Handler `toggle_sent`: Mengubah status surat antara 'draft' dan 'terkirim'.
  - Handler `delete`: Menghapus surat dengan proteksi CSRF dan otorisasi peran Sekretaris/Ketua/Superadmin.
  - Handler `export_pdf`: Mengunduh berkas binary PDF langsung dari peramban (`inline` Content-Disposition) dengan nama berkas yang aman.
  - Handler `templates` & `store_template`: Mengelola dan menyimpan templat surat siap pakai (contoh: Surat Pengantar RT/RW, Undangan Kerja Bakti, Undangan Rapat Warga).
- Template Native Blade di `resources/views/letters/`:
  - [index.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/letters/index.blade.rs): Daftar surat dengan filter tipe, chip status, aksi unduh PDF, detail, edit, dan hapus.
  - [form.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/letters/form.blade.rs): Formulir pembuatan dan pengubahan surat dengan auto-numbering, pengisian penerima, perihal, isi surat, dan penandatangan.
  - [detail.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/letters/detail.blade.rs): Lembar formal pratinjau surat bergaya kop cetak dokumen fisik dan panel aksi kontrol.
  - [templates.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/letters/templates.blade.rs): Manajemen daftar templat surat cepat dan formulir simpan templat.
- Rute-rute korespondensi di [src/routes/web.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/web.rs):
  - `/surat`, `/surat/baru`, `/surat/template`, `/surat/:id`, `/surat/:id/edit`, `/surat/:id/kirim`, `/surat/:id/hapus`, `/surat/:id/pdf`.
- Pengujian Otomatis di [tests/f1_7_letters_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f1_7_letters_test.rs):
  - `test_letter_authorization_matrix`: Memvalidasi hanya Sekretaris, Ketua, dan Superadmin yang diizinkan mengelola surat; Bendahara dan Warga dibatasi.
  - `test_letter_pdf_export_binary`: Memvalidasi output binary PDF valid `%PDF-1.x`.
  - `test_letter_auto_number_generation`: Memvalidasi format penomoran surat otomatis multi-organisasi dan angka romawi bulan.
  - `test_render_letter_views_native_blade`: Memvalidasi perenderan 4 view surat Native Blade (`letters/index`, `letters/form`, `letters/detail`, `letters/templates`) tanpa error.

### Diubah
- [PLAN.md](file:///d:/PROJECT%20LUMINA/app_paguyuban/PLAN.md): Menandai F1.7 selesai (`[x]`).


## [2026-11-06] - Migrasi Native Blade: Pembersihan & Pelepasan Crate Tera (Tahap 6) Selesai

### Dihapus
- Crate dependensi `tera = "1.19"` dilepas 100% dari [Cargo.toml](file:///d:/PROJECT%20LUMINA/app_paguyuban/Cargo.toml).
- Sebanyak 25 file template lama berformat `.html` di `resources/views/` dihapus secara permanen karena telah digantikan sepenuhnya oleh template Native Blade (`.blade.rs`).
- Fungsi-fungsi render usang Tera (`TERA`, `get_tera`, `load_dir_templates`, `render(..., &Context)`) di [ViewService](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/services/view_service.rs).

### Diubah & Dioptimalkan
- [ViewService](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/services/view_service.rs): Disederhanakan menjadi singleton murni `BladeEngine` dengan antarmuka `render(template_name, &BladeContext)` dan helper mata uang integer `format_rupiah`.
- Seluruh suite test (`spike0_test`, `f1_1_foundation_test`, `f1_2_superadmin_test`, `f1_3_members_test`, `f1_4_fees_test`, `f1_5_cash_lpj_test`, `f1_6_events_info_test`): Dimutakhirkan untuk menguji langsung template Native Blade, membersihkan import `tera::Context`, dan memperbaiki needless borrows sesuai standar Clippy.
- Dokumen pelacakan [docs/upgrade/README.md](file:///d:/PROJECT%20LUMINA/app_paguyuban/docs/upgrade/README.md): Menandai status akhir seluruh proses upgrade arsitektur (Tahap 0 s.d. Tahap 6) menjadi **SELESAI 100%** dan menyusun rencana kerja berikutnya untuk Roadmap F1.7 (Surat & Undangan) dan F1.8 (Pengerasan Sistem).


## [2026-11-06] - Migrasi Native Blade: Modul Kegiatan & Info Penting (F1.6) Selesai

### Ditambahkan
- Template Native Blade Modul Kegiatan di `resources/views/events/`:
  - [index.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/events/index.blade.rs): Daftar agenda kegiatan & acara warga ber-`@extends('layouts/dashboard.blade.rs')`, 3 kartu statistik (Kegiatan Mendatang, Total Kegiatan, Partisipasi Warga), tabel acara dengan badge status kependudukan dan konfirmasi hadir, serta aksi navigasi detail.
  - [form.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/events/form.blade.rs): Formulir tambah dan ubah kegiatan acara warga dengan penanganan waktu pelaksanaan `datetime-local`, deskripsi acara, pemilihan status `@selected(...)`, serta proteksi CSRF `@csrf`.
  - [detail.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/events/detail.blade.rs): Lembar detail acara, kartu rekapitulasi kehadiran warga (Hadir, Tidak Hadir, Belum Respon), formulir cepat pencatatan respon RSVP manual oleh pengurus berproteksi `@csrf`, dan tabel partisipasi warga per nomor rumah.
- Template Native Blade Modul Papan Informasi di `resources/views/announcements/`:
  - [index.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/announcements/index.blade.rs): Papan pengumuman resmi warga dengan penyorotan pengumuman tersemat (`📌 PENGUMUMAN TERSEMAT`), aksi pin/unpin berproteksi `@csrf`, tombol edit, dan hapus pengumuman.
  - [form.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/announcements/form.blade.rs): Formulir penulisan dan pengubahan pengumuman dengan checkbox sematan (`pinned`) dan proteksi `@csrf`.
- Pengujian Otomatis Native Blade di [tests/f1_6_events_info_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f1_6_events_info_test.rs):
  - `test_render_events_and_announcements_views_native_blade`: Memverifikasi keabsahan render seluruh 5 view kegiatan dan pengumuman (`events/index`, `events/form`, `events/detail`, `announcements/index`, `announcements/form`) menggunakan `BladeContext` (100% lolos).

### Diubah
- [EventController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/event_controller.rs): Memigrasikan seluruh handler perender view (`index`, `show_create`, `detail`, `show_edit`) dari `tera::Context` ke `BladeContext` via `ViewService::render_blade`.
- [AnnouncementController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/announcement_controller.rs): Memigrasikan seluruh handler perender view (`index`, `show_create`, `show_edit`) dari `tera::Context` ke `BladeContext` via `ViewService::render_blade`.
- Dokumen pelacakan [docs/upgrade/README.md](file:///d:/PROJECT%20LUMINA/app_paguyuban/docs/upgrade/README.md): Memperbarui status Tahap 5 menjadi **SELESAI** dan mengaktifkan Tahap 6 (Pembersihan & Pelepasan Crate Tera).


## [2026-11-06] - Migrasi Native Blade: Modul Kas & Generator LPJ (F1.5) Selesai

### Ditambahkan
- Template Native Blade Modul Kas & Keuangan di `resources/views/cash/`:
  - [index.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/cash/index.blade.rs): Halaman Buku Kas Umum dengan kartu ringkasan saldo berjalan, total penerimaan & pengeluaran berserta filter filter native `| rupiah`, tab navigasi Pemasukan, Pengeluaran, Donasi Barang (Natura), filter tahun aktif, dan tautan pratinjau LPJ.
  - [form_in.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/cash/form_in.blade.rs): Formulir pencatatan kas masuk/penerimaan dana berproteksi `@csrf`.
  - [form_out.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/cash/form_out.blade.rs): Formulir pencatatan pengeluaran dana/belanja paguyuban berproteksi `@csrf`.
  - [form_inkind.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/cash/form_inkind.blade.rs): Formulir penerimaan donasi barang non-tunai (natura) berproteksi `@csrf`.
  - [lpj_preview.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/cash/lpj_preview.blade.rs): Lembar cetak formal Laporan Pertanggungjawaban (LPJ) Keuangan tahunan dengan Bagian A (Pemasukan), Bagian B (Pengeluaran), Bagian C (Rekonsiliasi Saldo Akhir), Bagian D (Bantuan & Donasi Barang), dan blok tanda tangan tiga pihak (Ketua, Sekretaris, Bendahara) serta tombol unduh berkas resmi `.docx`.
- Pengujian Otomatis Native Blade di [tests/f1_5_cash_lpj_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f1_5_cash_lpj_test.rs):
  - `test_render_cash_views_native_blade`: Memverifikasi keabsahan render seluruh 5 view kas (`index`, `form_in`, `form_out`, `form_inkind`, `lpj_preview`) menggunakan `BladeContext` (100% lolos).

### Diubah
- [CashController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/cash_controller.rs): Memigrasikan seluruh handler perender view (`index`, `show_in`, `show_out`, `show_inkind`, `lpj_preview`) dari `tera::Context` ke `BladeContext` via `ViewService::render_blade`.
- Dokumen pelacakan [docs/upgrade/README.md](file:///d:/PROJECT%20LUMINA/app_paguyuban/docs/upgrade/README.md): Memperbarui status Tahap 4 menjadi **SELESAI** dan mengaktifkan Tahap 5 (Modul Kegiatan & Info Penting).


## [2026-10-24] - Selesai Tugas F0.1 & F0.2 (Spike 0)

### Ditambahkan
- File [docker-compose.yml](file:///d:/PROJECT%20LUMINA/app_paguyuban/docker-compose.yml) untuk menjalankan PostgreSQL 16 di lingkungan Docker.
- Konfigurasi database PostgreSQL pada [.env](file:///d:/PROJECT%20LUMINA/app_paguyuban/.env) dan [.env.example](file:///d:/PROJECT%20LUMINA/app_paguyuban/.env.example).
- Migrasi PostgreSQL di `database/migrations/`:
  - `0001_initial_schema.sql`: Skema lengkap multi-organisasi (`organizations`, `users`, `memberships`, `audit_logs`, `households`, `members`, `fee_plans`, `invoices`, `payments`, `events`, `transactions`, `announcements`, `letters`, `documents`).
  - `0002_enable_rls.sql`: Kebijakan Row Level Security (RLS) PostgreSQL untuk seluruh tabel ber-`org_id`.
- Binary runner migrasi [src/bin/migrate.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/bin/migrate.rs) (`cargo run --bin migrate`).
- Binary seeder pengembang [src/bin/seed.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/bin/seed.rs) (`cargo run --bin seed`) dengan akun default Superadmin, Ketua Paguyuban Kanfer 2, dan Bendahara.
- Model-model data di `src/app/models/`: [User](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/models/user.rs), [Organization](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/models/organization.rs), [Membership](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/models/membership.rs), [AuditLog](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/models/audit_log.rs).
- Suite pengujian Spike 0 di [tests/spike0_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/spike0_test.rs) yang memverifikasi:
  1. Koneksi dan operasi CRUD PostgreSQL dengan tipe `UUID` dan `BIGINT` rupiah.
  2. Isolasi multi-organisasi dengan Row Level Security (RLS) PostgreSQL via `SET LOCAL app.org_id`.
  3. Kontrak integrasi `AuthUser` dan sesi `req.session` untuk multi-tenant org switching.
  4. Render template Tera dengan makro komponen `ui.html` dan `sidebar.html`.
  5. Pembuatan dokumen ekspor DOCX (`docx-rs`) dan PDF (`printpdf`) untuk LPJ.
- Laporan lengkap evaluasi teknologi di [docs/SPIKE-0.md](file:///d:/PROJECT%20LUMINA/app_paguyuban/docs/SPIKE-0.md).

### Diubah
- [Cargo.toml](file:///d:/PROJECT%20LUMINA/app_paguyuban/Cargo.toml): Memperbarui path dependensi `lumina-framework` ke path lokal aktif `D:/FRAMEWORK RUST/framework`, menambahkan fitur `uuid` dan `postgres`, dependensi `bcrypt`, serta menambahkan crate `docx-rs` dan `printpdf`.
- [src/app/controllers/welcome_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/welcome_controller.rs): Memperbaiki tipe return response browser view agar konsisten dengan `IntoResponse`.
- [src/app/mod.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/mod.rs) & [src/app/models/user.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/models/user.rs): Menghilangkan warning unused import dan menyesuaikan struktur model dengan PostgreSQL.

## [2026-10-24] - Selesai Tugas F1.1 (Fondasi Dashboard PC)

### Ditambahkan
- Asset CSS Design System di `public/css/`: [tokens.css](file:///d:/PROJECT%20LUMINA/app_paguyuban/public/css/tokens.css) (variabel `--pg-*`) dan [components.css](file:///d:/PROJECT%20LUMINA/app_paguyuban/public/css/components.css).
- Template & Makro Tera di `resources/views/`:
  - `components/ui.html`: Makro kartu (`card`), statistik (`stat`), lencana status (`chip`), tombol (`btn`), input teks (`input_text`), flash notice (`notice`).
  - `components/sidebar.html`: Makro bilah navigasi kiri responsif sesuai peran dan pemilih organisasi aktif.
  - `layouts/dashboard.html`: Layout induk dashboard PC dengan header ringkas, avatar pengguna, dan dropdown organisasi.
  - `auth/login.html`: Halaman masuk pengurus paguyuban dengan nomor HP dan kata sandi.
  - `dashboard/index.html`: Halaman ringkasan dashboard PC (total warga/KK, kas riil, penarikan iuran, tunggakan, transaksi terkini).
- Layanan & Guard di `src/app/services/`:
  - [ViewService](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/services/view_service.rs): Singleton engine Tera dan helper pemformatan mata uang rupiah (`format_rupiah`).
  - [AuthContext](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/services/auth_guard.rs): Guard autentikasi, ekstraksi sesi pengguna, validasi peran (`require_role`), dan validasi Superadmin (`require_superadmin`).
- Controller di `src/app/controllers/`:
  - [AuthController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/auth_controller.rs): Menangani form `GET /login`, `POST /login` dengan verifikasi nomor HP dan hash bcrypt, serta `GET/POST /logout`.
  - [DashboardController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/dashboard_controller.rs): Menampilkan ringkasan data statistik organisasi aktif.
  - [OrgController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/org_controller.rs): Mengalihkan organisasi aktif (`POST /pilih-organisasi`) dan mencatat log perpindahan ke `audit_logs`.
- Routing web di [src/routes/web.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/web.rs): Mendaftarkan rute web `/login`, `/logout`, `/dashboard`, dan `/pilih-organisasi`.
- Seeder SQL & Binary di [database/seeders/0001_initial_seed.sql](file:///d:/PROJECT%20LUMINA/app_paguyuban/database/seeders/0001_initial_seed.sql) dan [src/bin/seed.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/bin/seed.rs).
- Pengujian Fondasi F1.1 di [tests/f1_1_foundation_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f1_1_foundation_test.rs):
  - Uji pemformatan nominal rupiah (`format_rupiah`).
  - Uji render HTML halaman Login pengurus.
  - Uji render HTML halaman Dashboard PC dengan layout induk dan makro komponen.
  - Uji alur login nomor HP, verifikasi sandi bcrypt, keanggotaan `memberships`, dan pencatatan `audit_logs`.
  - Uji pembuktian isolasi data multi-tenant antar dua organisasi (zero data leakage).

## [2026-10-24] - Selesai Tugas F1.2 (Modul Superadmin & Multi-Organisasi)

### Ditambahkan
- Controller Superadmin di [src/app/controllers/superadmin_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/superadmin_controller.rs):
  - `list_orgs`: Menampilkan statistik seluruh organisasi dan daftar tabel paguyuban/RT/RW.
  - `show_create_org` & `create_org`: Formulir pendaftaran organisasi baru dan pembuatan akun admin pertama (dengan verifikasi transaksi database).
  - `update_status`: Mengubah status organisasi (`aktif`, `menunggu`, `nonaktif`) dengan audit trail.
  - `impersonate`: Fitur *"Masuk sebagai admin"* untuk asistensi teknis langsung di organisasi target dengan pencatatan audit log `IMPERSONATE_ORG`.
  - `list_audit_logs`: Menampilkan riwayat 50 aktivitas terbaru sistem dengan waktu WIB dan rincian perubahan.
- Template Superadmin di `resources/views/superadmin/`:
  - `organizations.html`: Tampilan manajemen multi-organisasi, kartu statistik, dan tombol aksi per paguyuban/RT.
  - `org_create.html`: Formulir tambah organisasi dan admin pertama.
  - `audit_logs.html`: Tampilan jejak audit global (audit trail).
- Rute web Superadmin di [src/routes/web.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/web.rs): `/sa/organisasi`, `/sa/organisasi/baru`, `/sa/organisasi/:id/status`, `/sa/impersonasi/:id`, `/sa/log`.
- Pemuatan template rekursif di [src/app/services/view_service.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/services/view_service.rs) yang otomatis memuat seluruh subfolder template `.html` tanpa perlu deklarasi manual.
- Suite pengujian Superadmin di [tests/f1_2_superadmin_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f1_2_superadmin_test.rs):
  - Uji render HTML seluruh view superadmin (organisasi, buat baru, log audit).
  - Uji logika RBAC guard `require_superadmin()` menolak pengguna biasa dan meloloskan superadmin.
  - Uji pembuatan organisasi baru, akun admin pertama, relasi `memberships`, impersonasi, pengubahan status organisasi, dan verifikasi `audit_logs` di database.

## [2026-10-24] - Selesai Tugas F1.3 (Anggota, Rumah & Integrasi API Mobile SvelteKit)

### Ditambahkan
- Model data di `src/app/models/`: [Household](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/models/household.rs) dan [Member](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/models/member.rs).
- Controller PC Dashboard di [src/app/controllers/member_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/member_controller.rs):
  - `index`: Menampilkan daftar warga terdaftar dengan filter pencarian teks (`q`), filter status kependudukan, dan kartu statistik jiwa & KK.
  - `show_create` & `create`: Formulir penambahan warga baru dengan pembuatan/penautan unit rumah (`households`) otomatis dalam satu transaksi database.
  - `show_edit` & `update`: Formulir ubah data warga dan rumah.
  - `toggle_status`: Pengubahan cepat status warga (`aktif`, `menunggu`, `nonaktif`) dengan pencatatan `audit_logs`.
  - `show_import` & `import_csv`: Fitur impor massal CSV yang tangguh (resilient partial success) yang mendukung unggah berkas CSV atau *copy-paste* langsung teks tabel CSV.
- Controller RESTful JSON API Mobile di [src/app/controllers/member_api_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/member_api_controller.rs):
  - `GET /api/v1/me`: Menghasilkan profil warga, status keanggotaan, data hunian rumah, dan daftar anggota keluarga serumah (`family_members`) untuk aplikasi mobile SvelteKit PWA.
  - `PUT /api/v1/me`: Memungkinkan warga memperbarui nama dan nomor kontak mandiri.
  - `GET /api/v1/members`: Direktori kontak warga lingkungan untuk aplikasi mobile.
- Template Tera Dashboard PC di `resources/views/members/`:
  - `index.html`: Tampilan tabel data warga lengkap dengan pencarian, chip status (`ok`/`warn`/`bad`), dan tombol aksi.
  - `form.html`: Formulir tambah/ubah warga dengan datalist nomor rumah yang ada.
  - `import.html`: Panduan format CSV dan kotak input impor massal warga.
- Rute web dan API di [src/routes/web.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/web.rs) dan [src/routes/api.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/api.rs).
- Pengujian otomatis F1.3 di [tests/f1_3_members_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f1_3_members_test.rs):
  - Uji render HTML seluruh template PC warga (`index`, `form`, `import`).
  - Uji matriks izin RBAC: Bendahara hanya bisa melihat daftar dan ditolak memutasi data, sedangkan Admin dan Sekretaris memiliki akses penuh.
  - Uji transaksi pembuatan warga baru dan penautan otomatis nomor rumah (`households`).
  - Uji kontrak payload JSON API Mobile `/api/v1/members` dan `/api/v1/me`.
  - Uji isolasi data warga multi-tenant antar-organisasi (zero data leakage).

## [2026-10-24] - Selesai Tugas F1.4 (Iuran, Tagihan, Matriks Kotak 12 Bulan & Kas Atomik)

### Ditambahkan
- Model data di `src/app/models/`:
  - [FeePlan](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/models/fee_plan.rs): Skema pos tarif iuran paguyuban / RT.
  - [Invoice](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/models/invoice.rs): Skema tagihan iuran bulanan per warga.
  - [Payment](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/models/payment.rs): Skema pencatatan pembayaran dan verifikasi bendahara.
- Controller Web PC Dashboard di [src/app/controllers/fee_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/fee_controller.rs):
  - `index`: Menampilkan antarmuka **Matriks Kotak 12 Bulan (Square Grid)** per warga/rumah dengan filter tahun kalender, pencarian instan nama/blok rumah, 4 kartu ringkasan keuangan, dan fitur salin teks pengumuman broadcast WhatsApp.
  - `tariffs` & `store_tariff`: Pengelolaan tarif iuran rutin paguyuban (`fee_plans`).
  - `generate_view` & `generate_action`: Penerbitan tagihan bulanan massal untuk seluruh warga aktif secara otomatis (idempoten dengan `ON CONFLICT DO NOTHING`).
  - `pay_view` & `pay_action`: Formulir pelunasan tagihan dengan **transaksi atomik** yang secara bersamaan memperbarui status invoice ke `'lunas'`, mencatat record di tabel `payments`, dan membuat mutasi kas masuk di tabel `transactions` (kategori `'iuran'`) tanpa selisih.
- Controller RESTful JSON API Mobile di [src/app/controllers/fee_api_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/fee_api_controller.rs):
  - `GET /api/v1/iuran`: Menghasilkan rincian tagihan personal dan total tunggakan untuk warga di aplikasi mobile SvelteKit PWA.
  - `GET /api/v1/iuran/ringkasan`: Menghasilkan ringkasan transparansi kas iuran lingkungan untuk publik warga.
- Template Tera di `resources/views/fees/`:
  - `index.html`: Matriks Kartu Iuran Warga berpetak 12 bulan (hijau `✓` lunas, oranye `!` nunggak interaktif klik untuk bayar, abu-abu `-` belum terbit), statistik tertagih vs lunas, dan modal broadcast WhatsApp.
  - `tariffs.html`: Tabel daftar tarif dan formulir penambahan pos iuran baru.
  - `generate.html`: Formulir penagihan massal per periode bulan dan tahun.
  - `pay.html`: Formulir pelunasan tagihan tunai atau transfer dengan bukti catatan.
- Komponen CSS Desain di `public/css/components.css`:
  - Kelas `.pg-matrix-table`, `.pg-square-cell`, `.pg-square-cell--paid`, `.pg-square-cell--unpaid`, `.pg-square-cell--empty`, dan `.pg-matrix-legend`.
- Rute web dan API di [src/routes/web.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/web.rs) dan [src/routes/api.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/api.rs).
- Pengujian otomatis F1.4 di [tests/f1_4_fees_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f1_4_fees_test.rs):
  - Uji render HTML seluruh template modul iuran (`index`, `tariffs`, `generate`, `pay`).
  - Uji matriks otorisasi peran: Bendahara dan Admin memiliki kuasa penuh, Sekretaris berstatus *read-only*.
  - Uji isolasi multi-tenant data tarif dan tagihan antar dua organisasi (zero data leakage).
  - Uji penerbitan tagihan massal dan idempotensi.
  - Uji transaksi kas atomik pelunasan tagihan dan sinkronisasi mutasi kas masuk di `transactions`.

## [2026-10-24] - Selesai Tugas F1.5 (Kas, Pembukuan, Donasi Barang & Generator LPJ)

### Ditambahkan
- Model data di `src/app/models/`:
  - [Transaction](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/models/transaction.rs): Skema mutasi kas masuk dan keluar dengan relasi donatur/pembayaran.
  - [InKindDonation](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/models/in_kind_donation.rs): Skema bantuan barang/natura non-tunai dari warga.
- Controller Web PC Dashboard di [src/app/controllers/cash_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/cash_controller.rs):
  - `index`: Buku kas umum dengan kalkulasi *running balance* real-time, filter tahun kalender, filter jenis mutasi (masuk/keluar), dan kategori.
  - `show_in` & `store_in`: Formulir pencatatan kas masuk dengan opsi *"Donatur Anonim (Hamba Allah)"*.
  - `show_out` & `store_out`: Formulir belanja/pengeluaran uang dengan rincian penerima/toko.
  - `show_inkind` & `store_inkind`: Pencatatan donasi barang logistik warga tanpa mengubah saldo kas.
  - `lpj_preview`: Pratinjau dokumen formal Laporan Pertanggungjawaban (LPJ) keuangan standar Paguyuban Kanfer 2 (Ringkasan, Bagian A, Bagian B, Bagian C rekonsiliasi kas, Bagian D bantuan barang, dan lembar 4 tanda tangan pengesahan).
  - `export_docx`: Generator berkas Microsoft Word (`.docx`) siap cetak menggunakan crate `docx-rs`.
- Controller RESTful JSON API Mobile di [src/app/controllers/cash_api_controller.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/cash_api_controller.rs):
  - `GET /api/v1/kas/ringkasan`: Menghasilkan ringkasan saldo kas umum, total pemasukan, dan total pengeluaran untuk warga di aplikasi mobile.
- Template Tera di `resources/views/cash/`:
  - `index.html`: Buku kas umum, 4 kartu ringkasan, riwayat transaksi, dan tabel donasi barang.
  - `form_in.html`: Form pencatatan kas masuk.
  - `form_out.html`: Form pencatatan belanja kas keluar.
  - `form_inkind.html`: Form pencatatan bantuan donasi barang.
  - `lpj_preview.html`: Lembar pratinjau LPJ resmi siap cetak dan tanda tangan.
- Rute web dan API di [src/routes/web.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/web.rs) dan [src/routes/api.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/api.rs).
- Pengujian otomatis F1.5 di [tests/f1_5_cash_lpj_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f1_5_cash_lpj_test.rs):
  - Uji render HTML seluruh view modul kas dan pratinjau LPJ.
  - Uji otorisasi RBAC peran: Bendahara & Admin berhak mencatat, Sekretaris read-only.
  - Uji kalkulasi rumus saldo kas: `Saldo Akhir = Saldo Awal + Masuk - Keluar`.
  - Uji generasi berkas DOCX valid dengan signature magic header zip package.
  - Uji isolasi data kas multi-tenant antar dua organisasi (zero data leakage).

## [2026-10-25] - Selesai Tugas F1.6 (Modul Kegiatan, Agenda Warga, RSVP Kehadiran & Papan Pengumuman)

### Ditambahkan
- Model data di `src/app/models/`:
  - [Event](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/models/event.rs): Skema agenda kegiatan warga (judul, waktu mulai, tempat, deskripsi, status).
  - [EventRsvp](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/models/event_rsvp.rs): Skema konfirmasi kehadiran warga dengan kekangan idempotensi `UNIQUE(event_id, member_id)`.
  - [Announcement](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/models/announcement.rs): Skema papan pengumuman/informasi dengan fitur *pinned* (sematan di atas).
- Controller Web PC Dashboard di `src/app/controllers/`:
  - [EventController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/event_controller.rs): Daftar agenda kegiatan (`index`), formulir tambah/ubah (`show_create`, `create`, `show_edit`, `update`), pengubahan status kegiatan (`update_status`), halaman detail & pelacakan RSVP (`detail`), serta pencatatan RSVP manual oleh pengurus (`store_rsvp`).
  - [AnnouncementController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/announcement_controller.rs): Papan pengumuman terurut sematan (`index`), terbitkan pengumuman baru (`show_create`, `create`), ubah pengumuman (`show_edit`, `update`), ubah status sematan (`toggle_pin`), dan hapus pengumuman (`delete`).
- Controller RESTful JSON API Mobile di `src/app/controllers/`:
  - [EventApiController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/event_api_controller.rs): `GET /api/v1/kegiatan` (daftar acara + status RSVP pribadi) dan `POST /api/v1/kegiatan/:id/rsvp` (kirim respon kehadiran warga mandiri).
  - [AnnouncementApiController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/announcement_api_controller.rs): `GET /api/v1/info` (papan informasi paguyuban terurut sematan).
- Template Tera di `resources/views/`:
  - `events/index.html`: Daftar agenda kegiatan, 3 kartu ringkasan kegiatan, dan tabel acara.
  - `events/form.html`: Formulir penjadwalan/pengubahan acara kegiatan warga.
  - `events/detail.html`: Lembar detail acara, kartu rekapitulasi konfirmasi (Hadir, Tidak Hadir, Belum Respon), formulir cepat catat RSVP manual pengurus, dan tabel kehadiran.
  - `announcements/index.html`: Papan informasi berpenanda visual 📌 tersemat dengan aksi ubah, lepas pin, dan hapus.
  - `announcements/form.html`: Formulir penulisan dan penyematan pengumuman.
- Rute web dan API di [src/routes/web.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/web.rs) dan [src/routes/api.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/routes/api.rs).
- Pengujian otomatis F1.6 di [tests/f1_6_events_info_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f1_6_events_info_test.rs):
  - Uji render HTML seluruh view modul kegiatan dan pengumuman.
  - Uji matriks otorisasi RBAC peran: Admin dan Sekretaris memiliki hak kelola penuh, Bendahara dan Warga berstatus *read-only*.
  - Uji alur database kegiatan, agregasi RSVP, dan idempotensi pembaruan jawaban kehadiran warga.
  - Uji urutan sematan pengumuman (`ORDER BY pinned DESC, published_at DESC`) dan isolasi data multi-tenant antar dua organisasi.
  - Uji kontrak serialisasi dan deserialisasi payload JSON API Mobile SvelteKit PWA.
- Verifikasi langsung melalui antarmuka web (live browser testing) dengan akun Sekretaris Kanfer.

## [2026-11-06] - Migrasi Native Blade: Modul Iuran & Tagihan (F1.4) Selesai

### Ditambahkan
- Template Native Blade Modul Iuran di `resources/views/fees/`:
  - [index.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/fees/index.blade.rs): Matriks Kartu Iuran Warga berpetak 12 bulan (hijau `✓` lunas, oranye `!` nunggak klik bayar, abu-abu `-` belum terbit), 4 kartu ringkasan keuangan dengan filter native `{{ $val | rupiah }}`, filter tahun `@selected(...)`, pencarian nama/blok rumah, dan fitur salin broadcast WhatsApp.
  - [tariffs.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/fees/tariffs.blade.rs): Daftar tarif iuran aktif dan formulir tambah pos iuran baru berproteksi `@csrf`.
  - [generate.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/fees/generate.blade.rs): Formulir penerbitan tagihan massal untuk seluruh warga aktif berproteksi `@csrf`.
  - [pay.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/fees/pay.blade.rs): Formulir pelunasan tagihan tunai/transfer rekening warga berproteksi `@csrf`.
- Pengujian Otomatis Native Blade di [tests/f1_4_fees_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f1_4_fees_test.rs):
  - `test_render_fee_views_native_blade`: Memverifikasi keabsahan render seluruh template Blade iuran (`index`, `tariffs`, `generate`, `pay`) dengan `BladeContext` (100% lolos).

### Diperbaiki
- **Lumina Framework Core (`src/core/blade.rs`)**: Memperbaiki bug UTF-8 character boundary panic pada parser loop `@foreach`, `@forelse`, `@for`, dan blok `@if/@elseif/@else`. Sebelumnya loop increment `idx += 1` mengasumsikan ASCII 1-byte sehingga mengalami panic saat membaca karakter multi-byte seperti simbol tanda centang `✓` (3 byte) dan emoji; diperbaiki dengan `idx += slice.chars().next().map_or(1, |c| c.len_utf8())`.

### Diubah
- [Cargo.toml](file:///d:/PROJECT%20LUMINA/app_paguyuban/Cargo.toml): Menegaskan spesifikasi versi `lumina = { package = "lumina-framework", version = "0.6.1", path = "D:/FRAMEWORK RUST/framework" }`.
- [FeeController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/fee_controller.rs): Memigrasikan seluruh method perender view (`index`, `tariffs`, `generate_view`, `pay_view`) dari `tera::Context` ke `BladeContext` via `ViewService::render_blade`.
- Dokumen pelacakan [docs/upgrade/README.md](file:///d:/PROJECT%20LUMINA/app_paguyuban/docs/upgrade/README.md): Memperbarui status Tahap 3 menjadi **SELESAI** dan membuka antrean Tahap 4 (Modul Kas & Generator LPJ).

## [2026-11-06] - Migrasi Native Blade: Modul Anggota & Rumah (F1.3) Selesai

### Ditambahkan
- Template Native Blade Modul Anggota di `resources/views/members/`:
  - [index.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/members/index.blade.rs): Daftar warga terintegrasi `@extends('layouts/dashboard.blade.rs')`, kartu statistik jiwa & KK, pencarian teks `$search_query`, filter status kependudukan `@selected(...)`, serta aksi toggle status warga berproteksi `@csrf`.
  - [form.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/members/form.blade.rs): Formulir tambah dan ubah warga baru dengan penautan nomor rumah (datalist `$existing_houses`), proteksi `@csrf`, dan retensi input.
  - [import.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/members/import.blade.rs): Antarmuka impor massal CSV dengan panduan format dan proteksi `@csrf`.
- Pengujian Otomatis Native Blade di [tests/f1_3_members_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f1_3_members_test.rs):
  - `test_render_member_views_native_blade`: Memverifikasi keabsahan render seluruh template Blade anggota (`index`, `form`, `import`) dengan `BladeContext` (100% lolos).

### Diubah
- [MemberController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/member_controller.rs): Memigrasikan seluruh method perender view (`index`, `show_create`, `show_edit`, `show_import`) dari `tera::Context` ke `BladeContext` bawaan Lumina via `ViewService::render_blade`.
- Dokumen pelacakan [docs/upgrade/README.md](file:///d:/PROJECT%20LUMINA/app_paguyuban/docs/upgrade/README.md): Memperbarui status Tahap 2 menjadi **SELESAI** dan membuka antrean Tahap 3 (Modul Iuran & Tagihan).

## [2026-11-06] - Upgrade Lumina Framework v0.6.1 & Audit Progres Proyek

### Diperbarui
- Mengupgrade dependensi internal [lumina-framework](file:///d:/FRAMEWORK%20RUST/framework) dari versi `v0.3.0` ke versi **`v0.6.1`** (beserta `lumina_macros v0.1.5`).
- Menyelesaikan resolusi ambiguitas simbol `Transaction` antara transaksi ACID Lumina (`lumina::prelude::Transaction`) dan model entitas buku kas paguyuban (`app_paguyuban::app::models::Transaction`) pada [src/app/mod.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/mod.rs).
- Verifikasi kompilasi penuh seluruh library, 4 executable binary, dan 7 file unit/integration test suite dengan hasil bersih tanpa error (`cargo check --offline` & `cargo test --offline --no-run`).
- Audit komprehensif roadmap pengembangan Fase 1 serta sinkronisasi task list migrasi Blade Engine.


### Ditambahkan
- Integrasi **Pure Native Blade Engine** dari Lumina v0.3.0 ke dalam [ViewService](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/services/view_service.rs) (`ViewService::render_blade`) menggunakan `BladeEngine` dan `BladeContext` native Rust.
- Layout Induk Dashboard Native Blade di [resources/views/layouts/dashboard.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/layouts/dashboard.blade.rs): Mendukung `@yield('content')`, `@include('components/sidebar.blade.rs')`, `@csrf`, and `@selected(...)`.
- Komponen Sidebar Native Blade di [resources/views/components/sidebar.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/components/sidebar.blade.rs): Menggunakan helper atribut dinamis `@class(['is-active' => $active == '...'])` dan isolasi peran superadmin vs pengurus.
- Template Masuk Native Blade di [resources/views/auth/login.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/auth/login.blade.rs): Proteksi CSRF `@csrf`, penanganan pesan galat `@if ($error)`, dan retensi input `$old_phone`.
- Template Dashboard Native Blade di [resources/views/dashboard/index.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/dashboard/index.blade.rs): Pemanfaatan filter mata uang native `{{ $current_balance | rupiah }}` langsung tanpa pemformatan manual.
- Pengujian otomatis Native Blade di [tests/f1_1_foundation_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f1_1_foundation_test.rs):
  - `test_render_login_view_native_blade`: Memverifikasi auto-replacement `@csrf`, `@if ($error)`, dan interpolasi input lama.
  - `test_render_dashboard_view_native_blade`: Memverifikasi layout inheritance `@extends`, subview inclusion `@include`, filter pipeline `| rupiah`, dan penanda menu sidebar aktif `@class`.
- Migrasi Penuh Modul Superadmin (F1.2) ke Native Blade:
  - Template [resources/views/superadmin/organizations.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/superadmin/organizations.blade.rs): Daftar organisasi, chip badge tipe/status, impersonasi admin, dan aksi toggle status berproteksi `@csrf`.
  - Template [resources/views/superadmin/org_create.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/superadmin/org_create.blade.rs): Formulir pendaftaran organisasi & admin pertama.
  - Template [resources/views/superadmin/audit_logs.blade.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/resources/views/superadmin/audit_logs.blade.rs): Jejak audit trail global 50 aktivitas terbaru.
  - Pembaruan [SuperadminController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/superadmin_controller.rs), [AuthController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/auth_controller.rs), dan [DashboardController](file:///d:/PROJECT%20LUMINA/app_paguyuban/src/app/controllers/dashboard_controller.rs) untuk merender native Blade view.
  - Penambahan uji otomatis `test_render_superadmin_views_native_blade` di [tests/f1_2_superadmin_test.rs](file:///d:/PROJECT%20LUMINA/app_paguyuban/tests/f1_2_superadmin_test.rs) (100% lulus).




