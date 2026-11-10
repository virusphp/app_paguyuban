# Panduan Pengguna Pengurus Paguyuban / RT / RW

Dokumen ini adalah panduan operasional praktis untuk pengurus (Ketua, Sekretaris, Bendahara) dan Superadmin dalam mengelola administrasi warga, keuangan kas, iuran, kegiatan, korespondensi surat resmi, serta pemeliharaan sistem.

---

## 1. Pendahuluan & Struktur Organisasi

Aplikasi Manajemen Paguyuban dirancang dengan konsep **Multi-Organisasi Terisolasi**. Satu aplikasi dapat melayani banyak tingkatan rukun warga, paguyuban klaster, maupun RT tanpa risiko pencampuran data (*strict tenant isolation*).

### Matriks Peran Pengurus
| Fitur / Halaman | Superadmin | Ketua | Bendahara | Sekretaris | Warga (Fase 2) |
|---|:---:|:---:|:---:|:---:|:---:|
| **Dashboard Ringkasan** | ✅ | ✅ | ✅ | ✅ | ❌ |
| **Data Warga & Rumah** | ✅ | ✅ (Lihat/Ubah) | 👁️ (Hanya Lihat) | ✅ (Lihat/Ubah) | ❌ |
| **Impor CSV Warga** | ✅ | ✅ | ❌ | ✅ | ❌ |
| **Tarif & Tagihan Iuran** | ✅ | 👁️ (Hanya Lihat) | ✅ (Kelola Penuh) | ❌ | ❌ |
| **Pencatatan Pembayaran** | ✅ | 👁️ (Hanya Lihat) | ✅ (Input/Konfirmasi) | ❌ | ❌ |
| **Buku Kas & Ekspor LPJ** | ✅ | 👁️ (Hanya Lihat) | ✅ (Kelola Kas/LPJ) | ❌ | ❌ |
| **Kegiatan & RSVP** | ✅ | ✅ | 👁️ | ✅ | 👁️ (Fase 2) |
| **Papan Informasi (Pin/Hapus)** | ✅ | ✅ | 👁️ | ✅ | 👁️ (Fase 2) |
| **Surat & Undangan (PDF)** | ✅ | ✅ (Setujui/Cetak) | ❌ | ✅ (Buat/Kelola) | ❌ |
| **Manajemen Organisasi (/sa)** | ✅ | ❌ | ❌ | ❌ | ❌ |

---

## 2. Masuk Akun & Keamanan

### Cara Masuk Dashboard
1. Buka peramban (browser) dan akses alamat: `http://localhost:8000/login`.
2. Masukkan **Nomor Handphone** yang telah didaftarkan (contoh: `081234567890`).
3. Masukkan **Kata Sandi**.
4. Klik tombol **Masuk ke Dashboard**.

### Pengamanan Akun (Rate Limiting)
- Sistem membatasi percobaan masuk yang gagal maksimal **5 kali berturut-turut** dalam rentang 15 menit.
- Jika terlampaui, akun akan terkunci sementara selama 15 menit untuk melindungi dari percobaan peretasan (*brute-force*).
- Pesan kesalahan login bersifat netral untuk mencegah pelacakan nomor HP (*anti-enumeration*).

### Mengganti Organisasi Aktif
Bagi pengurus yang mengelola lebih dari satu organisasi (misal merangkap di RT dan Paguyuban):
1. Klik nama organisasi di bilah atas (header).
2. Pilih organisasi tujuan dari dropdown.
3. Seluruh data anggota, kas, iuran, dan agenda akan seketika beralih ke organisasi yang dipilih secara aman.

---

## 3. Panduan Operasional Per Modul

### 3.1. Pengelolaan Warga & Rumah (`/anggota`)
- **Menambah Warga Manual**:
  - Klik menu **Warga** ➔ **+ Tambah Warga**.
  - Masukkan Nama Lengkap, Nomor HP, Hubungan Keluarga (Kepala Keluarga / Pasangan / Anak / Kerabat), Status Kependudukan (Warga Tetap, Kontrak, Kost), dan tautkan nomor rumah.
- **Impor Massal CSV**:
  - Klik tombol **Impor CSV**.
  - Unduh template CSV contoh, isi data warga sesuai kolom, lalu unggah berkas CSV.

### 3.2. Pengelolaan Iuran & Tagihan (`/iuran`)
- **Penetapan Pos & Tarif Iuran (`/iuran/tarif`)**:
  - Tentukan nominal pos iuran bulanan (misal: Iuran Kas Warga, Sampah, Keamanan).
- **Menerbitkan Tagihan Bulanan Massal (`/iuran/terbitkan`)**:
  - Setiap awal bulan, Bendahara memilih bulan tagihan lalu klik **Terbitkan Tagihan**. Tagihan akan otomatis dibuat untuk seluruh Kepala Keluarga aktif.
- **Mencatat Pembayaran Warga (`/iuran/bayar`)**:
  - Saat warga membayar tunai atau transfer ke rekening paguyuban, Bendahara membuka form pelunasan, memilih tagihan, metode bayar (Tunai/Transfer), dan menyimpan transaksi.
  - Matriks 12 bulan akan berubah warna dari oranye (tertunggak) menjadi hijau (lunas).

### 3.3. Buku Kas Umum & Ekspor LPJ Keuangan (`/kas`)
- **Pencatatan Transaksi**:
  - **Pemasukan**: Klik **+ Kas Masuk** untuk mencatat penerimaan uang tunai/transfer di luar iuran (donasi warga, sponsor, lelang barang).
  - **Pengeluaran**: Klik **- Kas Keluar** untuk mencatat belanja operasional, kebersihan, keamanan, atau konsumsi kegiatan.
  - **Donasi Barang**: Klik **+ Donasi Barang** untuk mencatat bantuan natura non-uang (contoh: 2 kardus air mineral, 5 tenda terpal).
- **Pratinjau & Ekspor LPJ Cetak**:
  - Buka `/laporan/lpj/2026`.
  - Format laporan telah disesuaikan dengan standar baku LPJ Paguyuban Kanfer 2 (Bagian A: Pemasukan, Bagian B: Pengeluaran, Bagian C: Rekonsiliasi Saldo Akhir, Bagian D: Donasi Barang Natura, serta kolom tanda tangan Ketua, Sekretaris, dan Bendahara).
  - Klik tombol **Unduh Dokumen LPJ (.docx)** untuk mencetak atau mengedit dokumen di Microsoft Word.

### 3.4. Agenda Kegiatan & Info Warga (`/kegiatan`, `/info`)
- **Agenda Kegiatan**:
  - Buat jadwal pertemuan warga, kerja bakti, senam bersama, atau tirakatan 17-an.
  - Form RSVP memungkinkan warga atau pengurus mencatat kepastian hadir / tidak hadir warga per rumah.
- **Papan Informasi**:
  - Tulis pengumuman resmi RT/Paguyuban.
  - Gunakan tombol **Sematkan (Pin)** agar pengumuman darurat (jadwal posyandu, perbaikan pompa air) selalu berada di bagian teratas.

### 3.5. Korespondensi Surat & Undangan Resmi (`/surat`)
- **Pembuatan Surat**:
  - Klik menu **Surat & Undangan** ➔ **+ Buat Surat Baru**.
  - Pilih Jenis: *Surat Resmi* atau *Undangan Acara Warga*.
  - Nomor surat dihasilkan otomatis oleh sistem sesuai urutan standar (`001/PGY-KNF2/XI/2026`).
  - Masukkan tujuan penerima, perihal, isi surat, dan penandatangan.
- **Ekspor Dokumen PDF**:
  - Klik tombol **Unduh PDF** pada lembar detail surat.
  - Dokumen PDF A4 resmi berkop paguyuban, nomor registrasi, dan kolom pengesahan langsung siap dicetak atau dibagikan ke warga via WhatsApp.

---

## 4. Pemeliharaan & Prosedur Cadangan (Backup)

Sesuai standar operasional keamanan sistem:

### 1. Menjalankan Backup Otomatis
- **Lingkungan Linux / Server / Docker**:
  ```bash
  ./scripts/backup_db.sh
  ```
- **Lingkungan Windows (PowerShell)**:
  ```powershell
  .\scripts\backup_db.ps1
  ```
- Berkas arsip kompresi akan disimpan di folder `backups/backup_paguyuban_YYYYMMDD_HHMMSS.sql.gz`.
- Sistem secara otomatis menghapus arsip cadangan yang telah berusia lebih dari 30 hari (*retention rotation*).

### 2. Prosedur Pemulihan Berkala (Quarterly Recovery Drill)
Setiap kuartal, pengurus teknis disarankan melakukan simulasi pemulihan:
```bash
./scripts/restore_db.sh backups/backup_paguyuban_20261106_120000.sql.gz
```
atau pada PowerShell:
```powershell
.\scripts\restore_db.ps1 -BackupFile backups\backup_paguyuban_20261106_120000.sql
```

---

## 5. Bantuan & Dukungan Teknis

Jika mengalami kendala operasional atau menjumpai halaman kendala sistem (403 Akses Ditolak atau 500 Kendala Sistem), silakan hubungi tim administrator teknis paguyuban atau laporkan insiden melalui log audit Superadmin (`/sa/log`).

