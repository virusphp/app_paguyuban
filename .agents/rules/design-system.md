# Aturan Desain Standar — Dashboard PC Paguyuban

Berlaku untuk semua template (`.html`/Tera) dan CSS. Sumber token: `.agents/skills/paguyuban-dashboard-ui/references/tokens.css`. Salin ke `static/css/tokens.css` (sesuaikan folder static Lumina) dan muat di layout dasar.

## 1. Prinsip

1. **Sederhana untuk pengurus awam.** Satu tujuan per halaman, istilah sehari-hari, tombol aksi utama selalu jelas.
2. **Transparan.** Angka uang selalu tampil lengkap dengan satuan `Rp`, tidak disingkat.
3. **Tenang dan konsisten.** Satu warna utama (teal), warna status hanya untuk status.
4. **Pakai token, bukan nilai mentah.** Dilarang hex, px spasi, atau radius langsung di komponen. Gunakan `var(--pg-...)`.

## 2. Warna (token)

| Token | Nilai | Fungsi |
|---|---|---|
| `--pg-primary` | `#0F766E` | Tombol utama, link aktif, progress |
| `--pg-primary-hover` | `#0B5F59` | Hover tombol utama |
| `--pg-sidebar` | `#0B4F4A` | Sidebar pengurus |
| `--pg-sidebar-sa` | `#10231F` | Sidebar Superadmin (lebih gelap sebagai penanda peran) |
| `--pg-sidebar-text` / `--pg-sidebar-sub` | `#E6F4F1` / `#BFE3DD` | Teks di sidebar |
| `--pg-bg` | `#F3F6F5` | Latar halaman |
| `--pg-surface` | `#FFFFFF` | Kartu, tabel, form |
| `--pg-border` / `--pg-border-soft` | `#DDE5E2` / `#EEF2F0` | Garis kartu / garis baris tabel |
| `--pg-input-border` | `#7C918A` | Garis input (kontras 3:1 ke putih) |
| `--pg-text` / `--pg-muted` | `#14211F` / `#55645F` | Teks utama / teks pendukung |
| `--pg-ok-bg` / `--pg-ok-fg` | `#DCFAE6` / `#05603A` | Status baik: Lunas, Aktif, Terkirim |
| `--pg-warn-bg` / `--pg-warn-fg` | `#FEF0C7` / `#7A2E0E` | Perlu perhatian: Belum, Draft, Menunggu |
| `--pg-bad-bg` / `--pg-bad-fg` | `#FEE4E2` / `#912018` | Masalah: Terlambat, Ditolak, Nonaktif |

Kontras terukur (WCAG): putih di `--pg-primary` 5,5:1; teks di latar 15:1; muted di putih 6,2:1; chip ok 6,9:1, warn 8,3:1, bad 7,2:1; teks sidebar 8,3:1. Jangan mengubah pasangan warna tanpa menghitung ulang (minimal 4,5:1 untuk teks, 3:1 untuk garis kontrol).

Pemetaan status wajib (jangan membuat warna sendiri):

| Status | Tone |
|---|---|
| Lunas, Aktif, Terkirim, Disetujui | `ok` |
| Belum dibayar, Draft, Menunggu, Menunggu aktivasi | `warn` |
| Terlambat, Ditolak, Nonaktif, Batal | `bad` |

## 3. Tipografi

Font: **Plus Jakarta Sans** (400, 600, 700) dari Google Fonts, cadangan `system-ui, sans-serif`. Muat lewat `<link>` di layout dasar.

| Peran | Ukuran / berat |
|---|---|
| Judul halaman (h1) | 26px / 700 |
| Judul kartu (h2) | 15px / 700 |
| Isi, tabel, input | 14px / 400 |
| Label, teks pendukung | 13px / 400–600 |
| Chip, header tabel | 12px / 600 |
| Angka statistik | 26px / 700 |

Teks tidak boleh di bawah 12px. Angka uang rata kanan di tabel.

## 4. Spasi, bentuk, tata letak

- Skala spasi 4px: 4, 8, 12, 16, 20, 24, 32 (`--pg-s1` sampai `--pg-s7`).
- Padding kartu 16px, jarak antar kartu 16px, padding halaman 28px 32px.
- Radius: kartu 16px, tombol 12px, input 10px, chip penuh (pil).
- Tata letak PC: sidebar tetap 232px + konten fluid. Target 1280px, minimum 1024px. Di bawah 1024px sidebar boleh menciut menjadi ikon (mobile ditangani Fase 2).
- Urutan halaman: kepala halaman (judul, keterangan, tombol aksi utama di kanan) → baris kartu statistik → isi (tabel/form).
- Satu tombol utama per tampilan. Tombol sekunder memakai gaya garis.

## 5. Komponen baku

Gunakan makro di `references/ui.html` dan kelas di `references/components.css`. Jangan membuat versi lain.

- **Kartu statistik:** label (muted) → angka besar → keterangan kecil.
- **Chip status:** teks selalu ada (warna bukan satu-satunya penanda).
- **Tabel data:** header 12px muted, baris tinggi min 44px, aksi baris di kolom terakhir, kosong → *empty state*.
- **Form:** label di atas input, teks bantuan di bawah, pesan galat di bawah input dengan teks jelas ("Nomor HP harus 10–13 angka"), tanda wajib dengan kata "(wajib)" bukan hanya bintang.
- **Progress:** batang tipis 10px dengan teks persen di bawahnya.
- **Konfirmasi:** aksi merusak (hapus, batalkan, nonaktifkan) selalu lewat dialog konfirmasi yang menyebut nama data.
- **Umpan balik:** flash sukses/galat bawaan Lumina ditampilkan di atas konten dengan gaya chip `ok`/`bad`.
- **Keadaan kosong:** kalimat singkat + satu tombol aksi ("Belum ada anggota. Tambah anggota").
- **Memuat:** teks "Memuat..." atau kerangka abu-abu, jangan layar kosong.

## 6. Bahasa dan format

- Bahasa Indonesia, kalimat pendek, kata kerja untuk tombol: "Simpan", "Tambah Anggota", "Catat Pembayaran".
- Istilah baku: Iuran, Kas, Anggota, Pengurus, Kegiatan, Info Penting, Surat, Undangan, LPJ, Tunggakan, Saldo.
- Uang `Rp1.250.000`; tanggal `25 Okt 2026`; jam `19.30 WIB`.
- Jangan memasukkan angka atau nama contoh ke halaman produksi. Data contoh hanya di seed dev.

## 7. Aksesibilitas

- Elemen interaktif memakai `<button>`, `<a href>`, `<input>` dengan `<label for>`. Jangan `div` yang diberi onclick.
- Tombol berisi ikon saja wajib `aria-label`. Ikon dekoratif `aria-hidden="true"`.
- Fokus terlihat: outline 2px `--pg-primary`, offset 2px. Jangan dihapus.
- Target klik minimum 36px tinggi di desktop (44px bila pernah dipakai di layar sentuh).
- Status tidak hanya dibedakan warna: selalu ada teks.
- Tabel memakai `<th scope="col">`. Halaman memiliki satu `<h1>`.

## 8. Dilarang

- Emoji sebagai ikon. Gunakan SVG garis (stroke 1,8px, 22px).
- Gradasi latar, kartu dengan garis tebal sebelah kiri, bayangan berat.
- Warna baru di luar token. Font selain yang ditetapkan.
- Angka/statistik karangan. Semua angka berasal dari database.
- Membuat ulang komponen yang sudah ada di `references/`.
