{{-- Sidebar Pengurus dan Superadmin Blade Native --}}
@if (($area ?? '') == 'sa')
<aside class="pg-sidebar pg-sidebar--sa" aria-label="Menu Superadmin">
  <div class="pg-sidebar__header">
    <div class="pg-sidebar__brand">🎛️ Superadmin</div>
    <button type="button" class="pg-sidebar-close" id="pg-sidebar-close" aria-label="Tutup menu">✕</button>
  </div>
  <div class="pg-sidebar__sub">Pusat Kontrol & Telemetri</div>

  <a href="/sa/dashboard" @class(['is-active' => $active == 'dashboard'])>📊 Dashboard &amp; Telemetri</a>
  <a href="/sa/organisasi" @class(['is-active' => $active == 'organisasi'])>🏛️ Organisasi Lingkungan</a>
  <a href="/sa/pengguna" @class(['is-active' => $active == 'pengguna'])>👥 Manajemen Pengguna</a>
  <a href="/sa/log" @class(['is-active' => $active == 'log'])>📋 Log Audit Sistem</a>

  <div style="margin-top:auto; padding-top:var(--pg-s4); display:flex; flex-direction:column; gap:8px;">
    <a href="/pilih-organisasi" style="font-size:13px; color:var(--pg-sidebar-sub); display:flex; align-items:center; gap:6px;">
      🏢 Beralih ke Organisasi
    </a>
    <a href="/logout" style="color:var(--pg-bad-bg); font-size:13px;">Keluar</a>
  </div>
</aside>
@else
<aside class="pg-sidebar" aria-label="Menu utama">
  <div class="pg-sidebar__header">
    <div class="pg-sidebar__brand">{{ $org_name ?? 'Paguyuban' }}</div>
    <button type="button" class="pg-sidebar-close" id="pg-sidebar-close" aria-label="Tutup menu">✕</button>
  </div>
  <div class="pg-sidebar__sub">{{ $role_label ?? 'Pengurus' }}</div>

  @if (($is_superadmin ?? false) || ($role_label ?? '') == 'SUPERADMIN')
  <div style="padding:0 0 var(--pg-s3);">
    <a href="/sa/dashboard" class="pg-btn pg-btn--primary pg-btn--sm" style="width:100%; justify-content:center; gap:6px; font-size:12px; font-weight:700; background:rgba(255,255,255,0.15); border:1px solid rgba(255,255,255,0.25);">
      🎛️ Portal Superadmin
    </a>
  </div>
  @endif

  <a href="/dashboard" @class(['is-active' => $active == 'dashboard'])>Dashboard</a>
  <a href="/anggota" @class(['is-active' => $active == 'anggota'])>Anggota</a>
  <a href="/iuran" @class(['is-active' => $active == 'iuran'])>Iuran</a>
  <a href="/kas" @class(['is-active' => $active == 'kas'])>Kas dan Laporan</a>
  <a href="/kegiatan" @class(['is-active' => $active == 'kegiatan'])>Kegiatan</a>
  <a href="/info" @class(['is-active' => $active == 'info'])>Info Penting</a>
  <a href="/surat" @class(['is-active' => $active == 'surat'])>Surat dan Undangan</a>
  <a href="/pengaturan" @class(['is-active' => $active == 'pengaturan'])>Pengaturan</a>

  <div style="margin-top:auto; padding-top:var(--pg-s4); display:flex; flex-direction:column; gap:8px;">
    <a href="/pilih-organisasi" style="font-size:13px; color:var(--pg-sidebar-sub); display:flex; align-items:center; gap:6px;">
      🏢 Ganti Organisasi
    </a>
    <a href="/logout" style="color:var(--pg-bad-bg); font-size:13px;">Keluar</a>
  </div>
</aside>
@endif
