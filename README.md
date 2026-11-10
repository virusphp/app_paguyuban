# Kit Antigravity — Aplikasi Manajemen Paguyuban

Isi kit ini:

| File | Fungsi |
|---|---|
| `AGENTS.md` | Aturan proyek yang selalu dibaca agent (stack, aturan multi-organisasi, keamanan, definisi selesai) |
| `docs/PLAN.md` | Rencana lengkap: peran, modul, skema database, route, roadmap |
| `.agents/rules/design-system.md` | Aturan desain standar (warna, huruf, spasi, komponen, aksesibilitas) |
| `.agents/skills/paguyuban-dashboard-ui/` | Skill membuat halaman dashboard PC + `references/` (tokens.css, components.css, ui.html) |

## Cara pasang

1. Ekstrak zip ini ke **root workspace** Antigravity (folder proyek).
2. `AGENTS.md` di root dibaca Antigravity (IDE versi 1.20.5 ke atas). Kalau versi Anda lebih lama, ganti nama menjadi `GEMINI.md`.
3. Folder `.agents/` adalah lokasi default Antigravity saat ini. Kalau rules/skill tidak terdeteksi, ganti nama folder menjadi `.agent/` (lokasi versi lama).
4. Di panel **Rules** Antigravity, atur `design-system.md` menjadi *Always On*, atau *Glob* `**/*.html, **/*.css`. Batas Antigravity: 12.000 karakter per file rules (file di kit ini sudah di bawah batas).
5. Skill terpicu otomatis lewat deskripsinya saat Anda meminta halaman dashboard.

## Prompt pertama (salin ke Antigravity)

```
Baca AGENTS.md, docs/PLAN.md, dan .agents/rules/design-system.md.
Kerjakan HANYA Tugas F0 (Spike 0): pasang Lumina, sambungkan ke PostgreSQL
lewat docker compose, buktikan `migrate` dan satu model CRUD berjalan di
PostgreSQL, lalu tulis hasilnya di docs/SPIKE-0.md. Jangan mulai F1 sebelum
saya menyetujui hasil spike.
```

## Catatan

- Mockup visual (2 layar mobile dan 2 dashboard PC) ada di kanvas desain Claude; token warna di kit ini diambil dari mockup tersebut.
- Semua nama dan angka di mockup hanyalah contoh.
