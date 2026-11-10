---
name: paguyuban-dashboard-ui
description: Gunakan skill ini saat membuat atau mengubah halaman dashboard PC aplikasi manajemen paguyuban/RT di proyek Lumina (template Tera): halaman daftar, form tambah/ubah, detail, ringkasan dashboard, kartu statistik, tabel, status iuran, sidebar admin atau superadmin. Berisi pola halaman, komponen siap pakai, dan daftar periksa sesuai design-system.
---

# Skill: Dashboard PC Paguyuban

Tujuan: setiap halaman terlihat dan terasa sama, cepat dibuat, dan lolos aturan desain.

## Langkah kerja

1. Baca `.agents/rules/design-system.md` (token, komponen, larangan). Jangan menyalin ulang isinya.
2. Pastikan file dasar ada di proyek. Jika belum, salin dari folder `references/` ke lokasi static dan template milik Lumina (lihat hasil `cargo lumina new`; jangan memindahkan folder bawaan):
   - `references/tokens.css` → `static/css/tokens.css`
   - `references/components.css` → `static/css/components.css`
   - `references/ui.html` → `templates/components/ui.html`
   - `references/sidebar.html` → `templates/components/sidebar.html` (controller wajib mengirim `area`, `active`, `org_name`, `role_label`; item menu yang tidak boleh diakses peran harus dihapus dari template atau diberi kondisi `{% if %}`)
3. Pilih pola halaman di bawah. Bangun dari komponen, bukan dari HTML mentah.
4. Ambil semua data dari controller/service (bukan angka tetap di template).
5. Jalankan daftar periksa di bagian akhir sebelum menyatakan selesai.

## Layout dasar

Semua halaman memperluas satu layout: sidebar kiri 232px + area konten. Item sidebar pengurus: Dashboard, Anggota, Iuran, Kas dan Laporan, Kegiatan, Info Penting, Surat dan Undangan, Pengaturan. Sidebar Superadmin (kelas `pg-sidebar--sa`): Ringkasan, Organisasi, Pengguna, Log Aktivitas, Pengaturan Sistem. Item aktif memakai kelas `is-active` dan `aria-current="page"`. Item yang tidak boleh diakses peran saat ini tidak ditampilkan.

```html
{# templates/layouts/app.html (sesuaikan dengan konvensi Lumina) #}
<!doctype html>
<html lang="id">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{% block title %}Paguyuban{% endblock %}</title>
  <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;600;700&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="/static/css/tokens.css">
  <link rel="stylesheet" href="/static/css/components.css">
</head>
<body>
  <div class="pg-app">
    {% include "components/sidebar.html" %}
    <main class="pg-main">{% block content %}{% endblock %}</main>
  </div>
</body>
</html>
```

## Pola halaman

**A. Dashboard ringkasan** (contoh: Dashboard Admin)
`page_head` → grid 4 `stat_card` → grid 2 kolom (1,6fr / 1fr): kiri kartu dengan `progress` + tabel ringkas; kanan tumpukan kartu "Kegiatan Mendatang" dan "Surat dan Undangan".

**B. Daftar** (Anggota, Iuran, Transaksi, Surat)
`page_head` (tombol utama "Tambah ...") → baris filter (pencarian, status) → tabel di dalam `pg-card` → paginasi di bawah. Kolom terakhir berisi aksi. Kosong → `empty_state`.

**C. Form tambah/ubah**
`page_head` → satu `pg-card` lebar maks 640px berisi `field` berurutan → baris tombol: "Simpan" (utama) dan "Batal" (sekunder). Galat validasi tampil per field; input lama dipertahankan (`Redirect::back(...).with_input(...)`).

**D. Detail**
`page_head` (judul, chip status, aksi) → grid 2 kolom: informasi utama / riwayat atau catatan.

**E. Superadmin: Organisasi**
`page_head` ("Buat Organisasi") → 3 `stat_card` → grid 2 kolom: tabel organisasi (nama, tipe, anggota, status, aksi "Kelola") / kartu form "Buat Organisasi Baru" (nama, tipe, nama admin pertama, nomor HP admin, tombol "Buat dan Kirim Undangan").

## Komponen (makro Tera di `references/ui.html`)

```html
{% import "components/ui.html" as ui %}
{{ ui::page_head(title="Anggota", subtitle="48 anggota aktif", action_label="Tambah Anggota", action_href="/anggota/baru") }}
{{ ui::stat_card(label="Iuran terkumpul", value=iuran_terkumpul | rupiah, hint="34 dari 48 anggota lunas") }}
{{ ui::chip(tone="ok", label="Lunas") }}
{{ ui::progress(percent=71, caption="34 dari 48 anggota sudah lunas (71%)") }}
{{ ui::field(label="Nomor HP", name="phone", value=old.phone, error=errors.phone, help="Contoh: 08123456789") }}
{{ ui::empty_state(message="Belum ada anggota.", action_label="Tambah Anggota", action_href="/anggota/baru") }}
```

Filter `rupiah` dan `tanggal` harus didaftarkan di Tera pada backend (format `Rp1.250.000` dan `25 Okt 2026`). Jika belum ada, buat sebagai tugas kecil sebelum memakai angka di template.

## Aturan data di template

- Template hanya menampilkan. Perhitungan (total, persen, tunggakan) dilakukan di service dan dites.
- Jangan menampilkan data organisasi lain. `org_id` tidak pernah muncul di URL atau input tersembunyi sebagai sumber kebenaran.
- Tombol/menu aksi hanya tampil jika peran mengizinkan, dan route tetap dijaga di server.

## Daftar periksa sebelum selesai

- [ ] Hanya memakai token `--pg-*` dan kelas di `components.css`; tidak ada hex/px liar.
- [ ] Satu `<h1>`; satu tombol utama; urutan: judul → statistik → isi.
- [ ] Semua status memakai `chip` dengan tone sesuai pemetaan, lengkap dengan teks.
- [ ] Angka uang berformat `Rp...`, rata kanan di tabel; tanggal berformat Indonesia.
- [ ] Label terhubung ke input; pesan galat jelas; fokus terlihat; ikon saja punya `aria-label`.
- [ ] Keadaan kosong, galat, dan memuat sudah ditangani.
- [ ] Tampil benar di lebar 1024px dan 1280px; tabel lebar bisa digulir di dalam kartunya.
- [ ] Tidak ada angka atau nama karangan di template.
- [ ] Peran yang tidak berhak tidak melihat menu dan ditolak di server.
