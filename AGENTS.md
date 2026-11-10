# AGENTS.md — Aplikasi Manajemen Paguyuban / RT

Aplikasi manajemen untuk paguyuban warga, RW, dan RT: anggota, iuran, kas dan laporan (LPJ), kegiatan, info penting, surat dan undangan. Satu aplikasi melayani banyak organisasi (multi-organisasi).

Baca juga: `docs/PLAN.md` (rencana lengkap), `.agents/rules/design-system.md` (desain), skill `paguyuban-dashboard-ui` (membuat halaman).

## Fase saat ini

**Fase 0 dan Fase 1: dashboard PC untuk pengelola** (Superadmin, Admin, Bendahara, Sekretaris). Aplikasi mobile anggota ditunda ke Fase 2. Jangan membuat UI mobile sekarang, tetapi rancang route JSON `/api/v1/...` supaya siap dipakai Fase 2.

## Stack

- Backend dan dashboard PC: **Lumina** (framework web Rust, berbasis Tokio + Axum), template **Tera**, database lewat **SQLx**.
- Database: **PostgreSQL** (wajib). Dukungan PostgreSQL di Lumina BELUM terverifikasi: buktikan di Spike 0 (lihat PLAN.md).
- Mobile (Fase 2): SvelteKit PWA yang memakai JSON API dari backend ini.
- Deploy: Docker + Nginx (generator bawaan Lumina: `make:docker`, `make:nginx`).

## Fakta Lumina yang sudah diverifikasi

Sumber: README dan CHEAT_SHEET.md di github.com/lumina-java/framework.

- Pasang CLI: `cargo install cargo-lumina`. Proyek baru: `cargo lumina new <nama>`. Server jalan di `http://localhost:8000`.
- Struktur MVC: Models, Views, Controllers, Routes, Services. Ikuti struktur hasil generator; jangan memindahkan folder bawaan.
- Perintah: `./lumina watch` (hot reload), `migrate`, `db:seed`, `make:crud`, `make:auth`, `make:controller`, `make:model`, `make:docker`, `make:nginx`, `test:run`, `tinker`.
- Controller memakai bundle `Request` (`req.state.db()`, `req.session`, `req.token` untuk CSRF, `req.user` bertipe `Option<AuthUser>`) dan `ValidatedForm<T>` untuk validasi form.
- Redirect dengan flash: `Redirect::to(...).with_success(...).go(req.token, &req.session).await`; kembali dengan input lama: `Redirect::back(...).with_input(...).with_error(...)`.
- Auth bawaan: JWT + session, `Auth::check` (bcrypt), `Auth::login`, `Auth::logout`.
- ORM: trait `Model` (`find`, `query(...).with("relasi").get()`, `save`).
- Bawaan produksi: endpoint `/up` dan `/health`, graceful shutdown, security headers.
- Variabel database dibaca dari `.env` (`DATABASE_URL` dipakai SQLx).

## Belum terverifikasi: jangan berasumsi, buktikan dulu

1. Apakah Lumina dapat berjalan dengan PostgreSQL (README hanya mendokumentasikan SQLite dan MySQL).
2. Apakah `AuthUser` dapat membawa `org_id` dan peran per organisasi (atau perlu middleware sendiri).
3. Apakah transaksi per request bisa menjalankan `SET LOCAL app.org_id` untuk Row Level Security.
4. Lokasi dan konvensi folder template Tera hasil generator.

Jika dokumentasi Lumina bertentangan dengan file ini soal API, dokumentasi Lumina yang benar. File ini yang benar untuk keputusan produk.

## Multi-organisasi (WAJIB)

- Setiap tabel data memiliki kolom `org_id`. Organisasi bisa berjenis `paguyuban`, `rw`, atau `rt` dan boleh punya `parent_id`.
- Setiap query data bisnis harus memakai `org_id` dari sesi pengguna, tidak pernah dari input form atau URL.
- Pertahanan berlapis: filter `org_id` di service layer DAN Row Level Security di PostgreSQL (jika Spike 0 lolos).
- Superadmin boleh lintas organisasi lewat jalur terpisah (`/sa/...`). Setiap aksi lintas organisasi dan "masuk sebagai admin" wajib masuk `audit_logs`.
- Peran dicek di server pada setiap route (middleware atau guard), bukan hanya disembunyikan di UI.

## Uang, tanggal, bahasa

- Uang disimpan sebagai `BIGINT` rupiah (tanpa desimal, tanpa float). Tampil `Rp20.000` (titik ribuan).
- Waktu disimpan `timestamptz` (UTC), tampil WIB (Asia/Jakarta). Format tampil `25 Okt 2026`.
- Semua teks yang dilihat pengguna memakai Bahasa Indonesia yang singkat dan jelas. Nama kode (tabel, kolom, fungsi) memakai Bahasa Inggris.

## Keamanan

- Jangan simpan rahasia di repo. Gunakan `.env` (sudah di `.gitignore`) dan sediakan `.env.example`.
- Semua form memakai token CSRF bawaan Lumina. Query memakai parameter terikat (SQLx), tidak menggabungkan string SQL.
- Batasi percobaan login (rate limit), cegah enumerasi akun pada pesan error login.
- Data pribadi warga (nama, nomor HP, alamat) hanya tampil kepada peran yang berhak (lihat matriks izin di PLAN.md).

## Alur kerja dan definisi selesai

Satu tugas = satu perubahan kecil yang bisa dijalankan. Tugas dianggap selesai bila:

1. `cargo fmt` dan `cargo clippy` bersih tanpa peringatan.
2. `./lumina test:run` lulus, termasuk tes baru untuk logika yang ditambah (terutama izin dan isolasi organisasi).
3. Tidak ada query data tanpa cakupan `org_id`.
4. UI mengikuti `design-system.md` (token, bukan hex mentah) dan memakai komponen di `references/`.
5. Migrasi database ada untuk setiap perubahan skema, dan seed contoh hanya untuk lingkungan dev.
6. Perubahan dicatat singkat di `docs/CHANGELOG.md`.

Saat ragu soal produk, berhenti dan tanyakan. Jangan menebak aturan keuangan.
