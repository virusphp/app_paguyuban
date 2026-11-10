# Rencana Aplikasi Manajemen Paguyuban (dan RT)

Status: draft v1, 24 Okt 2026. Pemilik: pengurus Paguyuban Kanfer 2 RW 09 Slamaran.

## 1. Tujuan

Satu aplikasi untuk mengelola paguyuban warga, dan bisa dipakai juga oleh RT/RW tanpa membuat ulang. Mencakup anggota, iuran, kas dan laporan (LPJ), kegiatan, info penting, serta surat dan undangan.

Prioritas: sederhana dipakai pengurus awam, transparan untuk warga, data aman, mudah dipindah atau diperluas.

## 2. Keputusan teknologi

| Bagian                   | Pilihan                                              | Catatan                                                 |
| ------------------------ | ---------------------------------------------------- | ------------------------------------------------------- |
| Backend dan dashboard PC | **Lumina** (Rust, Tokio + Axum, template Tera, SQLx) | Fase 1                                                  |
| Database                 | **PostgreSQL**                                       | Wajib. Dukungan di Lumina belum terverifikasi (Spike 0) |
| Mobile anggota           | SvelteKit PWA, memakai JSON API `/api/v1`            | Fase 2                                                  |
| Auth                     | Session + JWT bawaan Lumina, kata sandi bcrypt       | Login dengan nomor HP                                   |
| Ekspor dokumen           | PDF dan DOCX dibuat di server                        | Kandidat crate dicek di Spike 0                         |
| Deploy                   | Docker + Nginx (generator Lumina), 1 VPS             | Backup `pg_dump` harian                                 |

### Spike 0: gerbang keputusan (maks 2–3 hari)

Buktikan hal berikut. Hasil ditulis di `docs/SPIKE-0.md`.

1. Lumina berjalan dengan PostgreSQL (`DATABASE_URL=postgres://...`), `migrate` dan satu model CRUD berfungsi.
2. `AuthUser` bisa membawa `org_id` dan peran, atau middleware sendiri bisa menyisipkannya.
3. Satu request dapat memakai transaksi dengan `SET LOCAL app.org_id = '<uuid>'` untuk Row Level Security.
4. Template Tera: lokasi folder, cara memakai layout dan macro (`references/ui.html`).
5. Ekspor PDF dan DOCX sederhana dari Rust (pilih crate, uji satu halaman).

Keputusan setelah spike:

- **Lolos semua:** lanjut dengan Lumina + PostgreSQL.
- **PostgreSQL gagal di Lumina:** pilihan (A) Lumina + MySQL/MariaDB, kehilangan RLS; (B) Axum + SQLx + Tera langsung dengan struktur folder gaya Lumina; (C) menambah dukungan PostgreSQL ke Lumina. Rekomendasi: (B), karena PostgreSQL adalah syarat awal.

## 3. Konsep inti: multi-organisasi

- `organizations` memiliki jenis `paguyuban`, `rw`, `rt`, dan `parent_id` opsional (RT di bawah RW).
- Satu pengguna bisa tergabung di beberapa organisasi lewat `memberships` dengan peran berbeda.
- Semua data bisnis membawa `org_id`. Isolasi dijaga di kode dan di PostgreSQL (RLS).
- Superadmin (developer/manajemen) membuat organisasi dan admin pertamanya, tidak ikut mengurus data harian.

## 4. Peran dan izin

| Modul                         | Superadmin | Admin (Ketua)     | Bendahara | Sekretaris | Anggota                 |
| ----------------------------- | ---------- | ----------------- | --------- | ---------- | ----------------------- |
| Organisasi dan pengguna       | Kelola     | Lihat org sendiri | -         | -          | -                       |
| Anggota dan rumah             | Lihat      | Kelola            | Lihat     | Lihat      | Profil sendiri          |
| Iuran (tarif, tagihan, bayar) | -          | Lihat             | Kelola    | -          | Tagihan sendiri         |
| Kas dan laporan LPJ           | -          | Lihat, setujui    | Kelola    | Lihat      | Ringkasan publik        |
| Kegiatan                      | -          | Kelola            | Lihat     | Kelola     | Lihat, konfirmasi hadir |
| Info penting                  | -          | Kelola            | -         | Kelola     | Lihat                   |
| Surat dan undangan            | -          | Setujui           | -         | Kelola     | Terima                  |
| Log aktivitas                 | Semua      | Org sendiri       | -         | -          | -                       |

"Ringkasan publik" = total pemasukan, pengeluaran, saldo. Rincian nama donatur hanya tampil bila admin mengaktifkannya.

## 5. Modul dan fitur

1. **Anggota:** data warga, rumah/keluarga, status (aktif, nonaktif, menunggu), impor CSV.
2. **Iuran:** tarif per periode, tagihan bulanan otomatis, catat pembayaran (tunai/transfer), konfirmasi bendahara, daftar tunggakan.
3. **Kas dan laporan:** pemasukan dan pengeluaran per kategori dan per kegiatan, termasuk saldo awal, iuran panitia, donasi, hasil lelang. Donasi barang dicatat terpisah. **Ekspor LPJ** (lihat bagian 9).
4. **Kegiatan:** jadwal, panitia, lokasi, konfirmasi hadir.
5. **Info penting:** pengumuman dengan tanggal terbit dan sematan.
6. **Surat dan undangan:** template, nomor surat otomatis, hasil PDF, status draft/terkirim.
7. **Dokumen:** arsip LPJ dan dokumen lain.
8. **Superadmin:** buat organisasi, undang admin pertama, ubah status, log aktivitas.

## 6. Skema database (spesifikasi, bukan file migrasi)

Ikuti format migrasi Lumina saat membuatnya. Semua `id` bertipe `uuid` (default `gen_random_uuid()`), uang `BIGINT` rupiah.

```sql
CREATE TYPE org_type   AS ENUM ('paguyuban','rw','rt');
CREATE TYPE org_status AS ENUM ('aktif','menunggu','nonaktif');
CREATE TYPE org_role   AS ENUM ('admin','bendahara','sekretaris','anggota');

organizations(id, name, type org_type, parent_id NULL -> organizations, status org_status, created_at)
users(id, name, phone UNIQUE, password_hash, is_superadmin bool DEFAULT false, status, last_login_at, created_at)
memberships(id, org_id, user_id, role org_role, UNIQUE(org_id, user_id))

households(id, org_id, house_no, note)
members(id, org_id, household_id, user_id NULL, full_name, phone, status, joined_on)   -- data warga; tidak semua punya akun

fee_plans(id, org_id, name, amount_idr BIGINT, period 'bulanan', active_from, active_to NULL)
invoices(id, org_id, member_id, fee_plan_id, period_month DATE, amount_idr BIGINT,
         status ('belum','lunas','batal'), UNIQUE(member_id, fee_plan_id, period_month))
payments(id, org_id, invoice_id, amount_idr BIGINT, method ('tunai','transfer'), paid_on, confirmed_by, note)

events(id, org_id, title, starts_at, location, description, status)
event_rsvps(id, org_id, event_id, member_id, answer ('hadir','tidak'))

transactions(id, org_id, event_id NULL, kind ('masuk','keluar'),
             category ('saldo_awal','iuran','iuran_panitia','donasi','lelang','konsumsi','hadiah','perlengkapan','lainnya'),
             description, amount_idr BIGINT CHECK (amount_idr > 0), occurred_on DATE,
             donor_name NULL, source_payment_id NULL, created_by)
in_kind_donations(id, org_id, event_id NULL, donor_name, item, quantity_text)   -- donasi barang, tidak masuk kas

announcements(id, org_id, title, body, pinned bool, published_at, created_by)
letter_templates(id, org_id, name, body)
letters(id, org_id, kind ('undangan','surat'), number, subject, body, recipient,
        status ('draft','terkirim'), created_by, sent_at)
documents(id, org_id, title, kind ('lpj','surat','lain'), file_path, created_at)
audit_logs(id, org_id NULL, actor_user_id, action, entity, entity_id, meta JSONB, at)
```

Row Level Security (contoh, terapkan ke semua tabel ber-`org_id`):

```sql
ALTER TABLE invoices ENABLE ROW LEVEL SECURITY;
CREATE POLICY org_isolation ON invoices
  USING (org_id = current_setting('app.org_id', true)::uuid);
-- Aplikasi memakai role DB non-owner. Tiap request: BEGIN; SET LOCAL app.org_id = '<uuid>'; ...; COMMIT;
```

## 7. Daftar route dashboard PC

Pengelola: `/login`, `/logout`, `/dashboard`, `/anggota` (`/baru`, `/:id`), `/iuran` (`/tagihan`, `/pembayaran`, `/tunggakan`), `/kas` (`/baru`), `/laporan` (`/lpj/:tahun`), `/kegiatan` (`/baru`, `/:id`), `/info`, `/surat` (`/baru`, `/:id`), `/pengaturan`.

Superadmin: `/sa/organisasi` (`/baru`, `/:id`), `/sa/pengguna`, `/sa/log`.

Fase 2 (JSON): `/api/v1/me`, `/api/v1/iuran`, `/api/v1/kegiatan`, `/api/v1/info`, `/api/v1/surat`.

## 8. Roadmap dan tugas (perkiraan 1 developer paruh waktu)

| Fase | Minggu | Isi                                                   |
| ---- | ------ | ----------------------------------------------------- |
| F0   | 1      | Spike 0 dan keputusan stack                           |
| F1   | 2–8    | Dashboard PC (rincian di bawah)                       |
| F2   | 9–11   | Mobile anggota (SvelteKit PWA) memakai `/api/v1`      |
| F3   | 12     | Uji coba di Paguyuban Kanfer 2, lalu pilot di satu RT |

Tugas F0 dan F1 (kerjakan berurutan, satu per satu):

- [x] **F0.1** Pasang Lumina, buat proyek, docker compose PostgreSQL, `.env.example`.
- [x] **F0.2** Jalankan Spike 0 (bagian 2), tulis `docs/SPIKE-0.md`, tunggu persetujuan.
- [x] **F1.1** Fondasi: layout dasar (sidebar + konten) dari `references/`, `tokens.css`, login nomor HP, sesi, middleware peran, pemilih organisasi, `audit_logs`, tes isolasi organisasi.
- [x] **F1.2** Superadmin: daftar organisasi, buat organisasi dan undang admin pertama, ubah status, "masuk sebagai admin" (tercatat).
- [x] **F1.3** Anggota dan rumah: daftar, tambah/ubah, impor CSV, status.
- [x] **F1.4** Iuran: tarif, pembuatan tagihan bulanan (job terjadwal), catat dan konfirmasi pembayaran, tunggakan.
- [x] **F1.5** Kas dan laporan: transaksi, kategori, donasi barang, ringkasan, ekspor LPJ PDF dan DOCX.
- [x] **F1.6** Kegiatan dan info penting.
- [x] **F1.7** Surat dan undangan: template, nomor otomatis, PDF.
- [x] **F1.8** Pengerasan: backup, rate limit, halaman error, panduan pengguna singkat.

Tugas Fase 2 (Mobile Anggota & REST API v1):

- [x] **F2.1.1** Autentikasi API Mobile Warga: `POST /api/v1/auth/login`, `POST /api/v1/auth/logout`, `GET /api/v1/auth/me`, `POST /api/v1/auth/switch-org`, integrasi JWT Bearer token Lumina, TokenBlacklist revocation, dan dukungan hybrid `Authorization: Bearer` di `AuthContext`.
- [x] **F2.1.2** Endpoint Surat & Undangan Warga: `GET /api/v1/surat` & `GET /api/v1/surat/:id/pdf` (arsip surat resmi publik dan undangan kegiatan).
- [x] **F2.1.3** Middleware CORS & API Error Formatting untuk klien PWA mobile.
- [x] **F2.1.4** Suite Pengujian Otomatis RESTful API v1 lengkap (`tests/f2_1_api_test.rs`).
- [ ] **F2.2** Inisialisasi Proyek Frontend SvelteKit PWA (`mobile_app`), Web App Manifest, Service Worker, dan UI Kit Design Tokens.
- [ ] **F2.3** Implementasi Layar Mobile: Login, Beranda, Iuran Warga, Agenda RSVP, Papan Pengumuman, Surat/Undangan, dan Profil Keluarga.
- [ ] **F2.4** Uji Coba PWA Offline, Caching, dan Pemasangan ke Layar Utama (Add to Home Screen).

## 9. Spesifikasi ekspor LPJ

Hasil ekspor mengikuti format LPJ Paguyuban Kanfer 2 tahun 2026 (`LPJ_TIRAKATAN_DAN_LOMBA_TAHUN_2026.docx`):

1. Judul: LPJ, kegiatan, paguyuban, kelurahan.
2. Kata pengantar, latar belakang, tujuan.
3. Pelaksanaan kegiatan: hari/tanggal, waktu, tempat, susunan acara.
4. Susunan panitia.
5. Laporan keuangan:
   - Ringkasan: total pemasukan, total pengeluaran, sisa saldo akhir.
   - A. Pemasukan: sisa saldo tahun lalu, iuran panitia (jumlah orang × nominal), donasi warga, hasil lelang; total.
   - Rincian donasi warga (nama dan jumlah), total.
   - B. Pengeluaran: nomor, keterangan, jumlah; total.
   - C. Saldo akhir: pemasukan dikurangi pengeluaran, dibawa sebagai saldo awal tahun berikutnya.
   - D. Catatan: donasi barang dari panitia (tidak dihitung sebagai pemasukan uang).
6. Penutup, tempat dan tanggal, tanda tangan Ketua, Sekretaris, Pembina, Bendahara.

Aturan hitung: `sisa saldo = saldo awal + semua pemasukan − semua pengeluaran`. Tanggal pelaksanaan dan tanggal laporan harus bisa diisi pengguna (tidak ditebak). Jika saldo awal tidak sama dengan saldo akhir periode sebelumnya, tampilkan peringatan sebelum ekspor.

## 10. Keamanan dan privasi

- HTTPS wajib di produksi. Cookie sesi `HttpOnly`, `Secure`, `SameSite`.
- Kata sandi di-hash (bcrypt bawaan Lumina). Batasi percobaan login.
- Isolasi organisasi: filter `org_id` di service layer + RLS PostgreSQL.
- Data pribadi hanya tampil sesuai matriks izin. Log aktivitas untuk perubahan keuangan, peran, dan aksi superadmin.
- Backup `pg_dump` harian, simpan di luar server, uji pemulihan tiap kuartal.

## 11. Kriteria selesai Fase 1

- Pengurus dapat: menambah anggota, membuat tagihan bulanan, mencatat pembayaran, mencatat kas, mengekspor LPJ yang sama formatnya dengan contoh.
- Dua organisasi uji tidak dapat saling melihat data (dibuktikan tes otomatis).
- Semua halaman mengikuti `design-system.md` dan lolos daftar periksa di skill `paguyuban-dashboard-ui`.

## 12. Keputusan produk yang disetujui pengguna (24 Okt 2026)

1. **Notifikasi anggota**: Tahap awal memakai Web Push browser PWA (gratis) dan tombol salin format rekap WA; opsi WhatsApp Gateway ditunda dan akan diingatkan kembali setelah seluruh fitur inti tuntas.
2. **Pembayaran iuran**: Opsi manual tunai atau transfer bank dicatat oleh Bendahara (Fase 1).
3. **Visibilitas donatur**: Nama donatur dicantumkan lengkap demi transparansi, dengan opsi checkbox _"Anonim / Hamba Allah"_ saat pencatatan jika warga menginginkannya.
