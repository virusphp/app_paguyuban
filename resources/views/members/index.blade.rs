@extends('layouts/dashboard.blade.rs')

@section('content')
<header class="pg-page-head">
  <div>
    <h1 class="pg-page-head__title">Daftar Warga & Keluarga</h1>
    <p class="pg-page-head__subtitle">{{ $total_members }} jiwa terdaftar di {{ $org_name }}</p>
  </div>
  <div class="pg-page-head__actions">
    @if ($can_edit)
    <a href="/anggota/impor" class="pg-btn pg-btn--outline">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="17 8 12 3 7 8"></polyline><line x1="12" y1="3" x2="12" y2="15"></line></svg>
      Impor CSV
    </a>
    <a href="/anggota/baru" class="pg-btn">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
      Tambah Warga
    </a>
    @endif
  </div>
</header>

<!-- Kartu Statistik Warga (Responsif: 3 Kolom di PC, 1 Kolom di Smartphone) -->
<div class="pg-grid-3">
  <div class="pg-card">
    <div class="pg-card__title">Total Jiwa (Warga)</div>
    <div class="pg-stat__value">{{ $total_members }} Jiwa</div>
    <div class="pg-muted">Terdaftar di organisasi</div>
  </div>
  <div class="pg-card">
    <div class="pg-card__title">Total Rumah / KK</div>
    <div class="pg-stat__value" style="color:var(--pg-primary);">{{ $total_households }} Unit</div>
    <div class="pg-muted">Unit hunian terdata</div>
  </div>
  <div class="pg-card">
    <div class="pg-card__title">Warga Aktif</div>
    <div class="pg-stat__value" style="color:var(--pg-ok-fg);">{{ $active_members }} Jiwa</div>
    <div class="pg-muted">{{ $pending_members }} menunggu / nonaktif</div>
  </div>
</div>

<!-- Filter & Pencarian Warga -->
<div class="pg-card" style="padding:16px;">
  <form action="/anggota" method="GET" style="display:flex; gap:10px; flex-wrap:wrap; align-items:center;">
    <div style="flex:1; min-width:220px;">
      <input type="text" name="q" class="pg-input" placeholder="Cari nama warga, nomor HP, atau nomor rumah..." value="{{ $search_query }}">
    </div>
    <div style="width:180px;">
      <select name="status" class="pg-select" style="width:100%; height:42px;">
        <option value="" @selected(!$filter_status)>Semua Status</option>
        <option value="aktif" @selected($filter_status == 'aktif')>Status: Aktif</option>
        <option value="menunggu" @selected($filter_status == 'menunggu')>Status: Menunggu</option>
        <option value="nonaktif" @selected($filter_status == 'nonaktif')>Status: Nonaktif</option>
      </select>
    </div>
    <div style="display:flex; gap:8px;">
      <button type="submit" class="pg-btn" style="min-height:42px;">Cari</button>
      @if ($search_query || $filter_status)
        <a href="/anggota" class="pg-btn pg-btn--outline" style="min-height:42px;">Reset</a>
      @endif
    </div>
  </form>
</div>

<!-- Tabel Daftar Warga -->
<div class="pg-card" style="padding:0; overflow:hidden;">
  <div class="pg-table-responsive">
    <table class="pg-table" style="margin:0;">
      <thead>
        <tr>
          <th style="padding-left:16px;">Nama Lengkap</th>
          <th>Rumah / Blok</th>
          <th>Nomor HP</th>
          <th>Status</th>
          <th>Terdaftar Sejak</th>
          @if ($can_edit)
          <th style="text-align:right; padding-right:16px;">Aksi</th>
          @endif
        </tr>
      </thead>
      <tbody>
        @forelse ($members as $m)
        <tr>
          <td style="padding-left:16px;">
            <strong>{{ $m.full_name }}</strong>
            @if ($m.has_account)
              <span style="font-size:11px; color:var(--pg-primary); background:var(--pg-border-soft); padding:2px 6px; border-radius:4px; margin-left:4px;">Akun</span>
            @endif
          </td>
          <td>
            <span style="font-weight:600;">{{ $m.house_no ?? '-' }}</span>
            @if ($m.house_note)
              <div style="font-size:12px; color:var(--pg-muted);">{{ $m.house_note }}</div>
            @endif
          </td>
          <td>{{ $m.phone ?? '-' }}</td>
          <td>
            @if ($m.status == 'aktif')
              <span class="pg-chip pg-chip--ok">Aktif</span>
            @elseif ($m.status == 'menunggu')
              <span class="pg-chip pg-chip--warn">Menunggu</span>
            @else
              <span class="pg-chip pg-chip--bad">Nonaktif</span>
            @endif
          </td>
          <td style="font-size:13px; color:var(--pg-muted);">{{ $m.joined_date }}</td>
          @if ($can_edit)
          <td style="text-align:right; padding-right:16px;">
            <div style="display:inline-flex; gap:6px; justify-content:flex-end;">
              <a href="/anggota/{{ $m.id }}" class="pg-btn pg-btn--outline pg-btn--sm">
                Ubah
              </a>
              <form action="/anggota/{{ $m.id }}/status" method="POST" style="display:inline; margin:0;">
                @csrf
                @if ($m.status == 'aktif')
                  <input type="hidden" name="status" value="nonaktif">
                  <button type="submit" class="pg-btn pg-btn--outline pg-btn--sm" style="color:var(--pg-bad-fg); border-color:var(--pg-bad-fg);" onclick="return confirm('Nonaktifkan warga ini?')">
                    Nonaktifkan
                  </button>
                @else
                  <input type="hidden" name="status" value="aktif">
                  <button type="submit" class="pg-btn pg-btn--outline pg-btn--sm" style="color:var(--pg-ok-fg); border-color:var(--pg-ok-fg);">
                    Aktifkan
                  </button>
                @endif
              </form>
            </div>
          </td>
          @endif
        </tr>
        @empty
        <tr>
          <td colspan="@if ($can_edit) 6 @else 5 @endif" style="text-align:center; padding:32px 16px; color:var(--pg-muted);">
            <div style="font-weight:600; margin-bottom:8px;">Belum ada data warga yang cocok.</div>
            @if ($can_edit)
              <a href="/anggota/baru" class="pg-btn" style="margin-top:4px;">Tambah Warga Pertama</a>
            @endif
          </td>
        </tr>
        @endforelse
      </tbody>
    </table>
  </div>
</div>
@endsection
