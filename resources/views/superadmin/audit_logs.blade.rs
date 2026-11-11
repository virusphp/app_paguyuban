@extends('layouts/dashboard.blade.rs')

@section('content')
<header class="pg-page-head" style="display:flex; justify-content:space-between; align-items:center; margin-bottom:var(--pg-s5);">
  <div>
    <h1 class="pg-page-head__title" style="font-size:22px; font-weight:700; margin:0;">Log Aktivitas Sistem (Audit Trail)</h1>
    <p class="pg-page-head__subtitle" style="color:var(--pg-muted); font-size:13px; margin:4px 0 0 0;">Riwayat seluruh aksi pengurus, perpindahan sesi, dan tindakan Superadmin.</p>
  </div>
  <div class="pg-page-head__actions">
    <a href="/sa/dashboard" class="pg-btn pg-btn--outline">
      ← Kembali ke Dashboard Superadmin
    </a>
  </div>
</header>

<div class="pg-card" style="background:var(--pg-surface); border:1px solid var(--pg-border); border-radius:var(--pg-r-card); padding:var(--pg-s5);">
  <div class="pg-card__header" style="display:flex; justify-content:space-between; align-items:center; margin-bottom:var(--pg-s4);">
    <h2 class="pg-card__title" style="font-size:16px; font-weight:700; margin:0;">Catatan 50 Aktivitas Terkini</h2>
    <span style="font-size:13px; color:var(--pg-muted);">Diperbarui secara real-time</span>
  </div>

  <div class="pg-table-responsive" style="overflow-x:auto;">
    <table class="pg-table" style="width:100%; border-collapse:collapse;">
      <thead>
        <tr style="border-bottom:1px solid var(--pg-border); text-align:left;">
          <th style="padding:var(--pg-s3);">Waktu (WIB)</th>
          <th style="padding:var(--pg-s3);">Aktor / Pengguna</th>
          <th style="padding:var(--pg-s3);">Organisasi Terkait</th>
          <th style="padding:var(--pg-s3);">Aksi</th>
          <th style="padding:var(--pg-s3);">Entitas</th>
          <th style="padding:var(--pg-s3);">Detail Perubahan</th>
        </tr>
      </thead>
      <tbody>
        @forelse ($logs as $log)
        <tr style="border-bottom:1px solid var(--pg-border-soft, #f1f5f9);">
          <td style="padding:var(--pg-s3); white-space:nowrap; font-size:13px;">{{ $log.created_at }}</td>
          <td style="padding:var(--pg-s3);">
            <strong>{{ $log.actor_name }}</strong>
            <div style="font-size:12px; color:var(--pg-muted);">{{ $log.actor_phone }}</div>
          </td>
          <td style="padding:var(--pg-s3);">
            @if ($log.org_name)
              <span class="pg-chip pg-chip--ok">{{ $log.org_name }}</span>
            @else
              <span class="pg-chip" style="background:#e2e8f0; color:#475569;">Sistem Global</span>
            @endif
          </td>
          <td style="padding:var(--pg-s3);">
            @if ($log.action == 'IMPERSONATE_ORG')
              <span class="pg-chip pg-chip--warn" style="font-weight:700;">IMPERSONASI</span>
            @elseif ($log.action == 'SWITCH_ORG')
              <span class="pg-chip pg-chip--ok">PILIH ORG</span>
            @elseif ($log.action == 'CREATE_ORG')
              <span class="pg-chip pg-chip--ok">BUAT ORG</span>
            @elseif ($log.action == 'UPDATE_STATUS')
              <span class="pg-chip pg-chip--bad">UBAH STATUS</span>
            @else
              <span class="pg-chip">{{ $log.action }}</span>
            @endif
          </td>
          <td style="padding:var(--pg-s3);"><code>{{ $log.entity }}</code></td>
          <td style="padding:var(--pg-s3); font-size:12px; font-family:monospace; max-width:240px; word-break:break-all;">
            {{ $log.meta_text }}
          </td>
        </tr>
        @empty
        <tr>
          <td colspan="6" style="text-align:center; padding:var(--pg-s8); color:var(--pg-muted);">
            Belum ada catatan aktivitas di dalam sistem.
          </td>
        </tr>
        @endforelse
      </tbody>
    </table>
  </div>
</div>
@endsection
