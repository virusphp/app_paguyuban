@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <div style="display:inline-flex; align-items:center; gap:8px; margin-bottom:6px;">
      <span class="pg-chip pg-chip--ok" style="font-weight:700;">
        <span class="pg-dot-pulse"></span> PORTAL SUPERADMIN & DEVELOPER
      </span>
      <span class="pg-muted" style="font-size:12px;">Multi-Tenant Central Engine</span>
    </div>
    <h1 style="font-size:26px; font-weight:800; letter-spacing:-0.5px; margin:0 0 4px;">Pusat Kontrol Sistem & Telemetri</h1>
    <p class="pg-muted">Monitoring komprehensif seluruh organisasi lingkungan warga, telemetri backend Rust Lumina, dan manajemen hak akses global.</p>
  </div>
  <div style="display:flex; gap:var(--pg-s2); flex-wrap:wrap; align-items:center;">
    <a href="/pilih-organisasi" class="pg-btn pg-btn--outline">
      🏢 Pilih Organisasi Warga
    </a>
    <a href="/sa/organisasi/baru" class="pg-btn pg-btn--primary">
      + Daftarkan Organisasi Baru
    </a>
  </div>
</div>

<!-- 4 Kartu Statistik Metrik Global -->
<div class="pg-grid-4" style="margin-bottom:var(--pg-s5);">
  <div class="pg-card pg-card--interactive">
    <div style="display:flex; justify-content:space-between; align-items:flex-start;">
      <div>
        <div class="pg-card__title">Total Organisasi</div>
        <div class="pg-stat__value" style="color:var(--pg-primary);">{{ $total_orgs }}</div>
      </div>
      <span style="font-size:22px; opacity:0.8;">🏛️</span>
    </div>
    <div class="pg-muted" style="font-size:12px; margin-top:6px;">
      <strong>{{ $active_orgs }}</strong> organisasi berstatus aktif
    </div>
  </div>

  <div class="pg-card pg-card--interactive">
    <div style="display:flex; justify-content:space-between; align-items:flex-start;">
      <div>
        <div class="pg-card__title">Warga Terdata (KK)</div>
        <div class="pg-stat__value">{{ $total_members }}</div>
      </div>
      <span style="font-size:22px; opacity:0.8;">👥</span>
    </div>
    <div class="pg-muted" style="font-size:12px; margin-top:6px;">
      Tersebar di <strong>{{ $total_households }}</strong> nomor hunian
    </div>
  </div>

  <div class="pg-card pg-card--interactive">
    <div style="display:flex; justify-content:space-between; align-items:flex-start;">
      <div>
        <div class="pg-card__title">Kas Kumulatif Global</div>
        <div class="pg-stat__value" style="color:var(--pg-ok-fg);">{{ $global_cash_balance | rupiah }}</div>
      </div>
      <span style="font-size:22px; opacity:0.8;">💰</span>
    </div>
    <div class="pg-muted" style="font-size:12px; margin-top:6px;">
      Total saldo kas seluruh unit lingkungan
    </div>
  </div>

  <div class="pg-card pg-card--interactive">
    <div style="display:flex; justify-content:space-between; align-items:flex-start;">
      <div>
        <div class="pg-card__title">Kepatuhan Iuran Global</div>
        <div class="pg-stat__value" style="color:var(--pg-primary);">{{ $global_compliance_percent }}%</div>
      </div>
      <span style="font-size:22px; opacity:0.8;">💳</span>
    </div>
    <div class="pg-muted" style="font-size:12px; margin-top:6px;">
      {{ $paid_invoices_count }} dari {{ $total_invoices_count }} tagihan lunas
    </div>
  </div>
</div>

<!-- Layout 2 Kolom: Kiri Telemetri & Organisasi, Kanan Aksi Cepat & Log -->
<div class="pg-grid-2" style="grid-template-columns: 1.5fr 1fr; align-items:start; gap:var(--pg-s5);">
  <div style="display:flex; flex-direction:column; gap:var(--pg-s5);">

    <!-- Developer & System Telemetry Panel -->
    <div class="pg-card" style="border-top:4px solid var(--pg-primary);">
      <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:14px; border-bottom:1px solid var(--pg-border-soft); padding-bottom:10px;">
        <div style="display:flex; align-items:center; gap:8px;">
          <span style="font-size:18px;">⚡</span>
          <h2 style="font-size:16px; font-weight:700; margin:0;">Developer &amp; Engine Telemetry</h2>
        </div>
        <span class="pg-chip pg-chip--ok" style="font-size:11px;">System Health: 100% OK</span>
      </div>

      <div style="display:grid; grid-template-columns:repeat(2, 1fr); gap:12px; font-size:13px; margin-bottom:14px;">
        <div style="background:var(--pg-track); padding:10px 12px; border-radius:var(--pg-r-ctl);">
          <div style="font-size:11px; color:var(--pg-muted); font-weight:600; text-transform:uppercase;">Backend Framework</div>
          <div style="font-weight:700; color:var(--pg-text); margin-top:2px;">Lumina Framework v0.6.1 (Rust)</div>
          <div style="font-size:11px; color:var(--pg-muted);">Tokio Async + Axum 0.7</div>
        </div>

        <div style="background:var(--pg-track); padding:10px 12px; border-radius:var(--pg-r-ctl);">
          <div style="font-size:11px; color:var(--pg-muted); font-weight:600; text-transform:uppercase;">Database Engine</div>
          <div style="font-weight:700; color:var(--pg-text); margin-top:2px;">PostgreSQL (SQLx Pool)</div>
          <div style="font-size:11px; color:var(--pg-ok-fg); font-weight:600;">✓ RLS Active (Row Level Security)</div>
        </div>

        <div style="background:var(--pg-track); padding:10px 12px; border-radius:var(--pg-r-ctl);">
          <div style="font-size:11px; color:var(--pg-muted); font-weight:600; text-transform:uppercase;">Security &amp; Auth</div>
          <div style="font-weight:700; color:var(--pg-text); margin-top:2px;">BCrypt Cost 12 + CSRF Guard</div>
          <div style="font-size:11px; color:var(--pg-muted);">JWT Bearer + Session Cookie</div>
        </div>

        <div style="background:var(--pg-track); padding:10px 12px; border-radius:var(--pg-r-ctl);">
          <div style="font-size:11px; color:var(--pg-muted); font-weight:600; text-transform:uppercase;">RESTful API v1 (Mobile)</div>
          <div style="font-weight:700; color:var(--pg-text); margin-top:2px;">Rate Limiter: 60 req/min</div>
          <div style="font-size:11px; color:var(--pg-ok-fg); font-weight:600;">✓ CORS Preflight Ready</div>
        </div>
      </div>

      <div style="display:flex; gap:8px; flex-wrap:wrap; align-items:center; border-top:1px dashed var(--pg-border); padding-top:12px; font-size:12px;">
        <span style="color:var(--pg-muted);">API Endpoints Aktif:</span>
        <code style="background:var(--pg-track); padding:2px 6px; border-radius:4px; font-size:11px;">/api/v1/auth/*</code>
        <code style="background:var(--pg-track); padding:2px 6px; border-radius:4px; font-size:11px;">/api/v1/warga</code>
        <code style="background:var(--pg-track); padding:2px 6px; border-radius:4px; font-size:11px;">/api/v1/iuran</code>
        <code style="background:var(--pg-track); padding:2px 6px; border-radius:4px; font-size:11px;">/api/v1/kas</code>
        <code style="background:var(--pg-track); padding:2px 6px; border-radius:4px; font-size:11px;">/api/v1/kegiatan</code>
        <code style="background:var(--pg-track); padding:2px 6px; border-radius:4px; font-size:11px;">/api/v1/surat</code>
      </div>
    </div>

    <!-- Ringkasan Unit Organisasi Terdaftar -->
    <div class="pg-card" style="padding:0; overflow:hidden;">
      <div style="padding:16px 20px; border-bottom:1px solid var(--pg-border); display:flex; justify-content:space-between; align-items:center;">
        <div>
          <h2 style="font-size:16px; margin:0; font-weight:700;">Daftar Lingkungan Paguyuban / RT Terdaftar</h2>
          <span class="pg-muted" style="font-size:12px;">Unit yang siap diakses dan dikelola oleh Superadmin</span>
        </div>
        <a href="/sa/organisasi" class="pg-btn pg-btn--outline pg-btn--sm">Kelola Semua &rarr;</a>
      </div>

      <div class="pg-table-wrap">
        <table class="pg-table" style="margin:0;">
          <thead>
            <tr>
              <th style="padding-left:20px;">Nama Lingkungan</th>
              <th>Tipe</th>
              <th>Warga</th>
              <th>Status</th>
              <th style="text-align:right; padding-right:20px;">Aksi Superadmin</th>
            </tr>
          </thead>
          <tbody>
            @forelse ($top_orgs as $org)
            <tr>
              <td style="padding-left:20px;">
                <div style="font-weight:700; color:var(--pg-text);">{{ $org.name }}</div>
              </td>
              <td>
                <span class="pg-chip" style="font-size:11px; text-transform:uppercase;">{{ $org.org_type }}</span>
              </td>
              <td>
                <span style="font-weight:600;">{{ $org.members_count }} Warga</span>
              </td>
              <td>
                @if ($org.status == 'aktif')
                <span class="pg-chip pg-chip--ok">Aktif</span>
                @else
                <span class="pg-chip pg-chip--warn">{{ $org.status }}</span>
                @endif
              </td>
              <td style="text-align:right; padding-right:20px;">
                <form action="/sa/impersonasi/{{ $org.id }}" method="POST" style="margin:0; display:inline-block;">
                  @csrf
                  <button type="submit" class="pg-btn pg-btn--primary pg-btn--sm" title="Masuk dan kelola dashboard organisasi ini sebagai admin">
                    ⚡ Masuk Kelola
                  </button>
                </form>
              </td>
            </tr>
            @empty
            <tr>
              <td colspan="5" class="pg-empty">Belum ada organisasi yang terdaftar di sistem.</td>
            </tr>
            @endforelse
          </tbody>
        </table>
      </div>
    </div>

  </div>

  <!-- Kolom Kanan: Pintasan Akses & Log Aktivitas Sistem -->
  <div style="display:flex; flex-direction:column; gap:var(--pg-s5);">

    <!-- Kartu Pintasan Cepat Developer / Superadmin -->
    <div class="pg-card" style="padding:20px;">
      <h2 style="font-size:15px; font-weight:700; margin:0 0 14px;">Pintasan Superadmin</h2>
      <div class="pg-grid-2" style="gap:10px;">
        <a href="/sa/organisasi/baru" class="pg-btn pg-btn--outline pg-btn--sm" style="display:flex; align-items:center; justify-content:center; gap:6px; padding:10px 8px; text-align:center;">
          🏛️ Tambah Paguyuban / RT
        </a>
        <a href="/sa/pengguna" class="pg-btn pg-btn--outline pg-btn--sm" style="display:flex; align-items:center; justify-content:center; gap:6px; padding:10px 8px; text-align:center;">
          👤 Kelola Akun Pengguna
        </a>
        <a href="/sa/log" class="pg-btn pg-btn--outline pg-btn--sm" style="display:flex; align-items:center; justify-content:center; gap:6px; padding:10px 8px; text-align:center;">
          📋 Audit Trail / Log
        </a>
        <a href="/pilih-organisasi" class="pg-btn pg-btn--outline pg-btn--sm" style="display:flex; align-items:center; justify-content:center; gap:6px; padding:10px 8px; text-align:center;">
          🏢 Switch Organisasi
        </a>
      </div>
    </div>

    <!-- Feed Log Audit Terkini -->
    <div class="pg-card" style="padding:20px;">
      <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:14px; border-bottom:1px solid var(--pg-border-soft); padding-bottom:8px;">
        <h2 style="font-size:15px; font-weight:700; margin:0;">Audit Aktivitas Sistem Terkini</h2>
        <a href="/sa/log" class="pg-btn pg-btn--outline pg-btn--sm" style="font-size:11px;">Semua Log &rarr;</a>
      </div>

      <div style="display:flex; flex-direction:column; gap:12px;">
        @forelse ($recent_logs as $log)
        <div style="display:flex; flex-direction:column; gap:2px; font-size:12px; border-bottom:1px solid var(--pg-border-soft); padding-bottom:8px;">
          <div style="display:flex; justify-content:space-between; align-items:center;">
            <strong style="color:var(--pg-text);">{{ $log.actor_name }}</strong>
            <span class="pg-muted">{{ $log.created_at }} WIB</span>
          </div>
          <div style="display:flex; align-items:center; gap:6px;">
            <span class="pg-chip" style="font-size:10px; padding:1px 6px;">{{ $log.action }}</span>
            <span class="pg-muted">{{ $log.org_name ?? 'Sistem Global' }}</span>
          </div>
        </div>
        @empty
        <div class="pg-empty" style="padding:16px 0;">Belum ada log audit tercatat.</div>
        @endforelse
      </div>
    </div>

  </div>
</div>
@endsection

