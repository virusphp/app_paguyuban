@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>Agenda Kegiatan & Acara Warga</h1>
    <p class="pg-muted">Jadwal rapat rutin, kerja bakti, pertemuan RT, dan konfirmasi kehadiran warga (RSVP).</p>
  </div>
  @if ($can_manage)
  <div>
    <a href="/kegiatan/baru" class="pg-btn">+ Tambah Kegiatan Baru</a>
  </div>
  @endif
</div>

<!-- 3 Kartu Statistik Kegiatan -->
<div class="pg-grid-3">
  <div class="pg-card">
    <div class="pg-card__title">Kegiatan Mendatang</div>
    <div class="pg-stat__value" style="color:var(--pg-primary);">{{ $upcoming_count }}</div>
    <div class="pg-muted">Acara aktif yang akan datang</div>
  </div>
  <div class="pg-card">
    <div class="pg-card__title">Total Seluruh Kegiatan</div>
    <div class="pg-stat__value">{{ $total_count }}</div>
    <div class="pg-muted">Termasuk kegiatan selesai & lampau</div>
  </div>
  <div class="pg-card">
    <div class="pg-card__title">Tingkat Partisipasi Warga</div>
    <div class="pg-stat__value" style="color:var(--pg-ok-fg);">{{ $average_rsvp_percentage }}%</div>
    <div class="pg-muted">Rata-rata konfirmasi hadir</div>
  </div>
</div>

<!-- Tabel Daftar Agenda Kegiatan -->
<div class="pg-card" style="padding:0; overflow:hidden;">
  <div style="padding:16px 20px; border-bottom:1px solid var(--pg-border); display:flex; justify-content:space-between; align-items:center; flex-wrap:wrap; gap:8px;">
    <h2 style="font-size:16px;">Daftar Acara & Kegiatan Lingkungan</h2>
  </div>

  <div class="pg-table-wrap">
    <table class="pg-table" aria-label="Tabel Agenda Kegiatan">
      <thead>
        <tr>
          <th style="min-width:180px; padding-left:16px;">Nama Kegiatan</th>
          <th style="min-width:150px;">Waktu & Tanggal</th>
          <th style="min-width:160px;">Lokasi / Tempat</th>
          <th style="width:110px;">Status</th>
          <th style="min-width:140px;">Konfirmasi Hadir</th>
          <th style="width:110px; text-align:right; padding-right:16px;">Aksi</th>
        </tr>
      </thead>
      <tbody>
        @forelse ($events as $ev)
        <tr>
          <td style="padding-left:16px;">
            <a href="/kegiatan/{{ $ev.id }}" style="font-weight:700; color:var(--pg-text); text-decoration:none;">
              {{ $ev.title }}
            </a>
            @if ($ev.description)
            <div class="pg-muted" style="font-size:12px; max-width:260px; white-space:nowrap; overflow:hidden; text-overflow:ellipsis;">
              {{ $ev.description }}
            </div>
            @endif
          </td>
          <td>
            <span style="font-weight:600;">{{ $ev.date_formatted }}</span>
            <div class="pg-muted" style="font-size:12px;">{{ $ev.time_formatted }} WIB</div>
          </td>
          <td>
            <span>{{ $ev.location }}</span>
          </td>
          <td>
            @if ($ev.status == 'aktif')
              <span class="pg-chip pg-chip--ok">Aktif</span>
            @elseif ($ev.status == 'selesai')
              <span class="pg-chip">Selesai</span>
            @else
              <span class="pg-chip pg-chip--bad">Batal</span>
            @endif
          </td>
          <td>
            <span style="font-weight:700; color:var(--pg-ok-fg);">{{ $ev.attending_count }} Hadir</span>
            <span class="pg-muted" style="font-size:12px;"> / {{ $ev.absent_count }} Tidak</span>
          </td>
          <td style="text-align:right; padding-right:16px;">
            <a href="/kegiatan/{{ $ev.id }}" class="pg-btn pg-btn--outline pg-btn--sm">Detail & RSVP</a>
          </td>
        </tr>
        @empty
        <tr>
          <td colspan="6" class="pg-empty">
            <p>Belum ada agenda kegiatan yang dibuat.</p>
            @if ($can_manage)
            <a href="/kegiatan/baru" class="pg-btn pg-btn--sm">+ Tambah Kegiatan Pertama</a>
            @endif
          </td>
        </tr>
        @endforelse
      </tbody>
    </table>
  </div>
</div>
@endsection

