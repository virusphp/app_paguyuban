# Panduan & Lembar Uji Pengujian Fitur (Web Dashboard & REST API Insomnia)

Dokumen ini disiapkan untuk pengujian komprehensif seluruh fitur web dashboard pengurus dan RESTful API v1 menggunakan peramban (browser) dan **Insomnia / Postman**.

---

## 1. Kredensial Akun Seeder Dev

| Peran | Nomor HP (Login) | Kata Sandi | Cakupan Hak Akses |
| :--- | :--- | :--- | :--- |
| **Superadmin** | `081111111111` | `password123` | Portal `/sa/...`, kelola seluruh organisasi, pengguna sistem, dan log audit. |
| **Ketua (Admin)** | `081234567890` | `password123` | Dashboard Paguyuban Kanfer 2, anggota, surat, kegiatan, pengumuman, pengaturan. |
| **Bendahara** | `081234567891` | `password123` | Buku kas (masuk/keluar/donasi), tarif & pembuatan tagihan iuran, ekspor LPJ. |
| **Sekretaris** | `081234567892` | `password123` | Pembuatan & penomoran surat otomatis, ekspor PDF/DOCX, agenda kegiatan, pengumuman. |
| **Warga Biasa** | `081234567893` | `password123` | Akses API mobile warga, cek tagihan sendiri, RSVP kegiatan, baca surat terkirim. |

---

## 2. Cara Menjalankan Server Dev

Jalankan perintah berikut di terminal:
```bash
# Pastikan migrasi & data contoh terisi
cargo run --bin migrate
cargo run --bin seed

# Jalankan server Lumina
cargo run --bin app_paguyuban-server
```
Server aktif di: `http://127.0.0.1:8000`

---

## 3. Matriks Pengujian Web Dashboard (Browser)

Akses `http://127.0.0.1:8000` di peramban Chrome / Firefox / Edge:

| No | Modul / Halaman | URL Web | Skenario yang Diuji |
| :---: | :--- | :--- | :--- |
| 1 | **Login Pengurus** | `GET /` atau `GET /login` | Form login nomor HP & password, proteksi brute force rate limit. |
| 2 | **Dashboard Utama** | `GET /dashboard` | 4 kartu statistik, progress iuran, 5 transaksi kas terakhir, pintasan aksi pengurus, agenda terdekat. |
| 3 | **Ganti Organisasi** | `POST /pilih-organisasi` | Switcher dropdown organisasi di topbar header. |
| 4 | **Data Warga** | `GET /anggota` | Filter pencarian nama/blok, tabel warga berakun, chip status. |
| 5 | **Tambah Warga** | `GET /anggota/baru` | Form tambah anggota keluarga baru dan unit rumah. |
| 6 | **Impor CSV Warga** | `GET /anggota/impor` | Unggah / tempel format CSV massal warga. |
| 7 | **Rekap Iuran 12 Bln** | `GET /iuran` | Matriks status iuran Jan–Des, salin teks siaran rekap WhatsApp. |
| 8 | **Tarif Iuran** | `GET /iuran/tarif` | Atur nominal iuran paguyuban bulanan. |
| 9 | **Generate Tagihan** | `GET /iuran/generate` | Penerbitan tagihan massal otomatis per bulan. |
| 10 | **Catat Pembayaran** | `GET /iuran/bayar/:id` | Konfirmasi pembayaran warga (tunai/transfer). |
| 11 | **Buku Kas Umum** | `GET /kas` | Saldo kas berjalan, filter jenis masuk/keluar/tahun. |
| 12 | **Catat Kas Masuk** | `GET /kas/masuk` | Pencatatan penerimaan uang kas. |
| 13 | **Catat Kas Keluar** | `GET /kas/keluar` | Pencatatan belanja & pengeluaran kas. |
| 14 | **Donasi Barang** | `GET /kas/donasi-barang` | Pencatatan bantuan natura / inventaris lingkungan. |
| 15 | **Pratinjau LPJ** | `GET /laporan/lpj/2026` | Tinjau laporan pertanggungjawaban keuangan resmi. |
| 16 | **Ekspor LPJ DOCX** | `GET /laporan/lpj/2026/docx` | Unduh file Microsoft Word `.docx` LPJ berstandar Paguyuban Kanfer. |
| 17 | **Agenda Kegiatan** | `GET /kegiatan` | Daftar jadwal rapat, kerja bakti, persentase RSVP warga. |
| 18 | **Detail Kegiatan** | `GET /kegiatan/:id` | Rincian acara, daftar nama warga yang hadir & izin. |
| 19 | **Papan Pengumuman** | `GET /info` | Kartu informasi resmi, aksi sematkan/pin pengumuman ke atas. |
| 20 | **Registrasi Surat** | `GET /surat` | Arsip surat pengantar & undangan, penomoran otomatis. |
| 21 | **Unduh PDF Surat** | `GET /surat/:id/pdf` | Unduh berkas resmi PDF ukuran A4 kop surat lingkungan. |
| 22 | **Pengaturan Organisasi**| `GET /pengaturan` | Profil lingkungan, nama paguyuban, daftar susunan pengurus aktif. |
| 23 | **Portal Superadmin** | `GET /sa/organisasi` | Kelola seluruh paguyuban/RT di sistem & impersonasi admin. |
| 24 | **Pengguna Superadmin** | `GET /sa/pengguna` | Monitoring seluruh akun login & hak akses superadmin/pengguna. |
| 25 | **Audit Logs Keamanan** | `GET /sa/log` | Catatan jejak audit aktivitas sensitif & perpindahan organisasi. |

---

## 4. Matriks Pengujian REST API (Insomnia / REST Client)

Konfigurasi Environment Insomnia:
- `base_url`: `http://127.0.0.1:8000/api/v1`
- `token`: `<token JWT dari respons login>`
- `org_id`: `<uuid organisasi aktif>`

Header Wajib pada Endpoint Terproteksi:
```http
Authorization: Bearer {{ token }}
Content-Type: application/json
Accept: application/json
```

### A. Autentikasi & Sesi
1. **Login Warga / Pengurus**:
   - `POST {{ base_url }}/auth/login`
   - Body JSON:
     ```json
     {
       "phone": "081234567890",
       "password": "password123"
     }
     ```
   - *Expected*: Status `200 OK`, mengembalikan token JWT, data user, dan daftar organisasi.
2. **Cek Profil Saya**:
   - `GET {{ base_url }}/auth/me`
3. **Pindah Organisasi Aktif**:
   - `POST {{ base_url }}/auth/switch-org`
   - Body JSON: `{"org_id": "c1f885e3-85f4-4a0f-90e6-58fb56a81aa4"}`
4. **Logout & Blacklist Token**:
   - `POST {{ base_url }}/auth/logout`

### B. Warga & Profil
5. **Data Diri Saya**: `GET {{ base_url }}/me`
6. **Perbarui Profil Saya**: `PUT {{ base_url }}/me`
7. **Daftar Anggota Warga**: `GET {{ base_url }}/members`

### C. Iuran & Keuangan Publik
8. **Daftar Tagihan Iuran Saya**: `GET {{ base_url }}/iuran`
9. **Ringkasan Transparansi Iuran**: `GET {{ base_url }}/iuran/ringkasan`
10. **Ringkasan Saldo Kas Umum**: `GET {{ base_url }}/kas/ringkasan`

### D. Kegiatan & RSVP
11. **Daftar Agenda Kegiatan**: `GET {{ base_url }}/kegiatan`
12. **Kirim Konfirmasi Kehadiran (RSVP)**:
    - `POST {{ base_url }}/kegiatan/:id/rsvp`
    - Body JSON: `{"response": "hadir", "notes": "Hadir bersama keluarga"}`

### E. Pengumuman & Surat
13. **Papan Pengumuman Warga**: `GET {{ base_url }}/info`
14. **Daftar Surat Publik**: `GET {{ base_url }}/surat`
15. **Detail Surat**: `GET {{ base_url }}/surat/:id`
16. **Unduh PDF Surat**: `GET {{ base_url }}/surat/:id/pdf`

### F. Middleware CORS & Fallback Error
17. **Preflight CORS Test**:
    - Method: `OPTIONS {{ base_url }}/auth/login`
    - Header: `Origin: http://localhost:5173`
    - *Expected*: Status `204 No Content` dengan header `Access-Control-Allow-Origin: http://localhost:5173`.
18. **Fallback Endpoint 404 JSON**:
    - `GET {{ base_url }}/invalid-route`
    - *Expected*: Format JSON `{"success": false, "error": "...", "code": 404}` alih-alih HTML.

