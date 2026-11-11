@extends('layouts/dashboard.blade.rs')

@section('content')
<header class="pg-page-head" style="display:flex; justify-content:space-between; align-items:center; margin-bottom:var(--pg-s5);">
  <div>
    <h1 class="pg-page-head__title" style="font-size:22px; font-weight:700; margin:0;">Manajemen Organisasi</h1>
    <p class="pg-page-head__subtitle" style="color:var(--pg-muted); font-size:13px; margin:4px 0 0 0;">Kelola seluruh paguyuban, RW, dan RT yang terdaftar di dalam sistem.</p>
  </div>
  <div class="pg-page-head__actions">
    <a href="/sa/organisasi/baru" class="pg-btn pg-btn--primary">
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
      Tambah Organisasi Baru
    </a>
  </div>
</header>

<!-- Kartu Statistik Superadmin -->
<div class="pg-grid-3">
  <div class="pg-card pg-card--interactive">
    <div style="display:flex; justify-content:space-between; align-items:flex-start;">
      <div>
        <div class="pg-card__title">Total Organisasi</div>
        <div class="pg-stat__value">{{ $total_orgs }}</div>
      </div>
      <span style="font-size:22px; opacity:0.8;">🏛️</span>
    </div>
    <div class="pg-muted" style="margin-top:6px;">Unit terdaftar di seluruh sistem</div>
  </div>

  <div class="pg-card pg-card--interactive">
    <div style="display:flex; justify-content:space-between; align-items:flex-start;">
      <div>
        <div class="pg-card__title">Organisasi Aktif</div>
        <div class="pg-stat__value" style="color:var(--pg-ok-fg);">{{ $active_orgs }}</div>
      </div>
      <span style="font-size:22px; opacity:0.8;">✓</span>
    </div>
    <div class="pg-muted" style="margin-top:6px;">Beroperasi aktif dan melayani warga</div>
  </div>

  <div class="pg-card pg-card--interactive">
    <div style="display:flex; justify-content:space-between; align-items:flex-start;">
      <div>
        <div class="pg-card__title">Menunggu / Nonaktif</div>
        <div class="pg-stat__value" style="color:var(--pg-warn-fg);">{{ $pending_orgs }}</div>
      </div>
      <span style="font-size:22px; opacity:0.8;">⏳</span>
    </div>
    <div class="pg-muted" style="margin-top:6px;">Perlu verifikasi atau tinjauan admin</div>
  </div>
</div>

<!-- Tabel Daftar Organisasi -->
<div class="pg-card" style="background:var(--pg-surface); border:1px solid var(--pg-border); border-radius:var(--pg-r-card); padding:var(--pg-s5);">
  <div class="pg-card__header" style="display:flex; justify-content:space-between; align-items:center; margin-bottom:var(--pg-s4);">
    <h2 class="pg-card__title" style="font-size:16px; font-weight:700; margin:0;">Daftar Paguyuban & RT Terdaftar</h2>
  </div>

  <div class="pg-table-responsive" style="overflow-x:auto;">
    <table class="pg-table" style="width:100%; border-collapse:collapse;">
      <thead>
        <tr style="border-bottom:1px solid var(--pg-border); text-align:left;">
          <th style="padding:var(--pg-s3);">Nama Organisasi</th>
          <th style="padding:var(--pg-s3);">Tipe</th>
          <th style="padding:var(--pg-s3);">Admin Utama</th>
          <th style="padding:var(--pg-s3);">Nomor HP</th>
          <th style="padding:var(--pg-s3);">Status</th>
          <th style="padding:var(--pg-s3); text-align:right;">Aksi Superadmin</th>
        </tr>
      </thead>
      <tbody>
        @forelse ($organizations as $org)
        <tr style="border-bottom:1px solid var(--pg-border-soft, #f1f5f9);">
          <td style="padding:var(--pg-s3);">
            <strong>{{ $org.name }}</strong>
            <div style="font-size:12px; color:var(--pg-muted);">ID: {{ $org.id }}</div>
          </td>
          <td style="padding:var(--pg-s3);">
            @if ($org.org_type == 'paguyuban')
              <span class="pg-chip pg-chip--ok">PAGUYUBAN</span>
            @elseif ($org.org_type == 'rw')
              <span class="pg-chip pg-chip--warn">RW</span>
            @else
              <span class="pg-chip pg-chip--warn">RT</span>
            @endif
          </td>
          <td style="padding:var(--pg-s3);">{{ $org.admin_name ?? '-' }}</td>
          <td style="padding:var(--pg-s3);">{{ $org.admin_phone ?? '-' }}</td>
          <td style="padding:var(--pg-s3);">
            @if ($org.status == 'aktif')
              <span class="pg-chip pg-chip--ok">Aktif</span>
            @elseif ($org.status == 'menunggu')
              <span class="pg-chip pg-chip--warn">Menunggu</span>
            @else
              <span class="pg-chip pg-chip--bad">Nonaktif</span>
            @endif
          </td>
          <td style="padding:var(--pg-s3); text-align:right;">
            <div style="display:inline-flex; gap:var(--pg-s2); justify-content:flex-end;">
              <!-- Masuk Sebagai Admin (Impersonasi) -->
              <form action="/sa/impersonasi/{{ $org.id }}" method="POST" style="display:inline;">
                @csrf
                <button type="submit" class="pg-btn pg-btn--primary" style="padding:6px 12px; font-size:13px;" title="Kelola langsung sebagai Admin organisasi ini">
                  ⚡ Masuk Kelola
                </button>
              </form>

              <!-- Ubah Status -->
              <form action="/sa/organisasi/{{ $org.id }}/status" method="POST" style="display:inline;">
                @csrf
                @if ($org.status == 'aktif')
                  <input type="hidden" name="status" value="nonaktif">
                  <button type="submit" class="pg-btn" style="padding:6px 12px; font-size:13px; border:1px solid var(--pg-bad-bg); color:var(--pg-bad-fg); background:transparent;" onclick="return confirm('Nonaktifkan organisasi ini?')">
                    Nonaktifkan
                  </button>
                @else
                  <input type="hidden" name="status" value="aktif">
                  <button type="submit" class="pg-btn" style="padding:6px 12px; font-size:13px; border:1px solid var(--pg-ok-bg); color:var(--pg-ok-fg); background:transparent;">
                    Aktifkan
                  </button>
                @endif
              </form>
            </div>
          </td>
        </tr>
        @empty
        <tr>
          <td colspan="6" style="text-align:center; padding:var(--pg-s8); color:var(--pg-muted);">
            Belum ada organisasi terdaftar. Klik "Tambah Organisasi Baru" untuk memulai.
          </td>
        </tr>
        @endforelse
      </tbody>
    </table>
  </div>
</div>
@endsection
