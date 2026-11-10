{{-- Sidebar Pengurus dan Superadmin Blade Native --}}
@if (($area ?? '') == 'sa')
<aside class="pg-sidebar pg-sidebar--sa" aria-label="Menu Superadmin">
  <div class="pg-sidebar__header">
    <div class="pg-sidebar__brand">Superadmin</div>
    <button type="button" class="pg-sidebar-close" id="pg-sidebar-close" aria-label="Tutup menu">✕</button>
  </div>
  <div class="pg-sidebar__sub">Manajemen sistem</div>
  <a href="/sa/organisasi" @class(['is-active' => $active == 'organisasi'])>Organisasi</a>
  <a href="/sa/pengguna" @class(['is-active' => $active == 'pengguna'])>Pengguna</a>
  <a href="/sa/log" @class(['is-active' => $active == 'log'])>Log Aktivitas</a>
  <div style="margin-top:auto; padding-top:var(--pg-s4);">
    <a href="/logout" style="color:var(--pg-bad-bg);">Keluar</a>
  </div>
</aside>
@else
<aside class="pg-sidebar" aria-label="Menu utama">
  <div class="pg-sidebar__header">
    <div class="pg-sidebar__brand">{{ $org_name ?? 'Paguyuban' }}</div>
    <button type="button" class="pg-sidebar-close" id="pg-sidebar-close" aria-label="Tutup menu">✕</button>
  </div>
  <div class="pg-sidebar__sub">{{ $role_label ?? 'Pengurus' }}</div>
  <a href="/dashboard" @class(['is-active' => $active == 'dashboard'])>Dashboard</a>
  <a href="/anggota" @class(['is-active' => $active == 'anggota'])>Anggota</a>
  <a href="/iuran" @class(['is-active' => $active == 'iuran'])>Iuran</a>
  <a href="/kas" @class(['is-active' => $active == 'kas'])>Kas dan Laporan</a>
  <a href="/kegiatan" @class(['is-active' => $active == 'kegiatan'])>Kegiatan</a>
  <a href="/info" @class(['is-active' => $active == 'info'])>Info Penting</a>
  <a href="/surat" @class(['is-active' => $active == 'surat'])>Surat dan Undangan</a>
  <a href="/pengaturan" @class(['is-active' => $active == 'pengaturan'])>Pengaturan</a>
  <div style="margin-top:auto; padding-top:var(--pg-s4);">
    <a href="/logout" style="color:var(--pg-bad-bg);">Keluar</a>
  </div>
</aside>
@endif
