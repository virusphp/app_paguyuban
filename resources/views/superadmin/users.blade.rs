@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>Daftar Pengguna Sistem</h1>
    <p class="pg-muted">Seluruh akun pengguna, pengurus organisasi paguyuban/RT, dan superadmin terdaftar.</p>
  </div>
</div>

<!-- 2 Kartu Statistik Pengguna -->
<div class="pg-grid-3">
  <div class="pg-card">
    <div class="pg-card__title">Total Pengguna Terdaftar</div>
    <div class="pg-stat__value">{{ $total_users }} Akun</div>
    <div class="pg-muted">Warga & pengurus berakun</div>
  </div>
  <div class="pg-card">
    <div class="pg-card__title">Akses Superadmin</div>
    <div class="pg-stat__value" style="color:var(--pg-primary);">{{ $total_superadmins }} Akun</div>
    <div class="pg-muted">Hak kelola lintas organisasi</div>
  </div>
</div>

<!-- Tabel Pengguna Sistem -->
<div class="pg-card" style="padding:0; overflow:hidden;">
  <div style="padding:16px 20px; border-bottom:1px solid var(--pg-border);">
    <h2 style="font-size:16px; margin:0;">Data Akun Pengguna</h2>
  </div>

  <div class="pg-table-wrap">
    <table class="pg-table" style="margin:0;">
      <thead>
        <tr>
          <th style="padding-left:20px;">Nama Pengguna</th>
          <th>Nomor Ponsel (Login)</th>
          <th>Hak Akses</th>
          <th>Organisasi & Peran</th>
          <th>Status Akun</th>
          <th style="padding-right:20px;">Login Terakhir</th>
        </tr>
      </thead>
      <tbody>
        @forelse ($users as $u)
        <tr>
          <td style="padding-left:20px; font-weight:600;">{{ $u.name }}</td>
          <td>{{ $u.phone }}</td>
          <td>
            @if ($u.role_badge == 'SUPERADMIN')
              <span class="pg-chip pg-chip--ok">SUPERADMIN</span>
            @else
              <span class="pg-chip">PENGGUNA</span>
            @endif
          </td>
          <td>
            <span style="font-size:13px; color:var(--pg-muted);">{{ $u.affiliations }}</span>
          </td>
          <td>
            @if ($u.status == 'aktif')
              <span class="pg-chip pg-chip--ok">Aktif</span>
            @else
              <span class="pg-chip pg-chip--bad">{{ $u.status }}</span>
            @endif
          </td>
          <td style="padding-right:20px; font-size:12px; color:var(--pg-muted);">
            {{ $u.last_login }}
          </td>
        </tr>
        @empty
        <tr>
          <td colspan="6" style="text-align:center; padding:32px; color:var(--pg-muted);">
            Belum ada akun pengguna yang terdaftar.
          </td>
        </tr>
        @endforelse
      </tbody>
    </table>
  </div>
</div>
@endsection

