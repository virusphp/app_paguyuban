@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <div style="display:flex; align-items:center; gap:var(--pg-s2); margin-bottom:4px;">
      <h1>{{ $event.title }}</h1>
      @if ($event.status == 'aktif')
        <span class="pg-chip pg-chip--ok">Aktif</span>
      @elseif ($event.status == 'selesai')
        <span class="pg-chip">Selesai</span>
      @else
        <span class="pg-chip pg-chip--bad">Batal</span>
      @endif
    </div>
    <p class="pg-muted">Detail acara, waktu pelaksanaan, dan rekapitulasi konfirmasi kehadiran (RSVP) warga.</p>
  </div>
  <div style="display:flex; gap:var(--pg-s2);">
    <a href="/kegiatan" class="pg-btn pg-btn--outline">← Kembali ke Agenda</a>
    @if ($can_manage)
    <a href="/kegiatan/{{ $event.id }}/edit" class="pg-btn pg-btn--outline">Ubah Kegiatan</a>
    @endif
  </div>
</div>

<!-- Informasi Acara & Kartu Statistik RSVP -->
<div class="pg-grid-2">
  <!-- Rincian Acara -->
  <div class="pg-card">
    <h2 class="pg-card__title">Informasi Pelaksanaan</h2>
    <div style="display:flex; flex-direction:column; gap:12px; font-size:14px;">
      <div style="display:flex; justify-content:space-between; border-bottom:1px solid var(--pg-border); padding-bottom:8px;">
        <span class="pg-muted">Waktu & Tanggal:</span>
        <strong style="color:var(--pg-primary);">{{ $event.datetime_formatted }} WIB</strong>
      </div>
      <div style="display:flex; justify-content:space-between; border-bottom:1px solid var(--pg-border); padding-bottom:8px;">
        <span class="pg-muted">Tempat / Lokasi:</span>
        <span style="font-weight:600;">{{ $event.location }}</span>
      </div>
      <div>
        <span class="pg-muted" style="display:block; margin-bottom:4px;">Deskripsi / Susunan Acara:</span>
        <div style="background:var(--pg-track); padding:12px; border-radius:var(--pg-r-card); line-height:1.5;">
          {{ $event.description }}
        </div>
      </div>
    </div>
  </div>

  <!-- Statistik Kehadiran (RSVP) -->
  <div class="pg-card">
    <h2 class="pg-card__title">Rekapitulasi Kehadiran Warga</h2>
    <div class="pg-grid-3" style="margin-bottom:16px;">
      <div style="text-align:center; padding:12px; background:var(--pg-ok-bg); border-radius:var(--pg-r-card);">
        <div style="font-size:24px; font-weight:700; color:var(--pg-ok-fg);">{{ $attending_count }}</div>
        <div style="font-size:12px; font-weight:600; color:var(--pg-ok-fg);">Konfirmasi Hadir</div>
      </div>
      <div style="text-align:center; padding:12px; background:var(--pg-bad-bg); border-radius:var(--pg-r-card);">
        <div style="font-size:24px; font-weight:700; color:var(--pg-bad-fg);">{{ $absent_count }}</div>
        <div style="font-size:12px; font-weight:600; color:var(--pg-bad-fg);">Tidak Hadir</div>
      </div>
      <div style="text-align:center; padding:12px; background:var(--pg-track); border-radius:var(--pg-r-card);">
        <div style="font-size:24px; font-weight:700; color:var(--pg-muted);">{{ $pending_count }}</div>
        <div style="font-size:12px; font-weight:600; color:var(--pg-muted);">Belum Respon</div>
      </div>
    </div>

    @if ($can_manage)
    <!-- Form Cepat Catat RSVP Warga Manual oleh Pengurus -->
    <div style="border-top:1px solid var(--pg-border); padding-top:14px; margin-top:14px;">
      <h3 style="font-size:13px; font-weight:700; margin:0 0 10px;">+ Catat Respon Kehadiran Warga (Manual)</h3>
      <form method="POST" action="/kegiatan/{{ $event.id }}/rsvp" style="display:flex; gap:var(--pg-s2); flex-wrap:wrap;">
        @csrf
        <div style="flex:1; min-width:180px;">
          <select name="member_id" class="pg-select" style="width:100%; height:38px;" required>
            <option value="" disabled selected>Pilih Nama Warga / Rumah...</option>
            @foreach ($members_options as $m)
            <option value="{{ $m.id }}">{{ $m.house_no }} — {{ $m.full_name }}</option>
            @endforeach
          </select>
        </div>
        <div style="width:130px;">
          <select name="answer" class="pg-select" style="width:100%; height:38px;" required>
            <option value="hadir">Hadir</option>
            <option value="tidak">Tidak Hadir</option>
          </select>
        </div>
        <button type="submit" class="pg-btn pg-btn--sm" style="height:38px;">Simpan</button>
      </form>
    </div>
    @endif
  </div>
</div>

<!-- Tabel Daftar Warga & Status Kehadiran (RSVP) -->
<div class="pg-card" style="padding:0; overflow:hidden;">
  <div style="padding:16px 20px; border-bottom:1px solid var(--pg-border); display:flex; justify-content:space-between; align-items:center;">
    <h2 style="font-size:16px;">Daftar Konfirmasi Partisipasi Warga</h2>
  </div>

  <div class="pg-table-wrap">
    <table class="pg-table" aria-label="Tabel Respon RSVP Warga">
      <thead>
        <tr>
          <th style="min-width:110px; padding-left:16px;">No. Rumah</th>
          <th style="min-width:200px;">Nama Warga</th>
          <th style="min-width:140px;">Status Kehadiran</th>
          <th style="min-width:140px; text-align:right; padding-right:16px;">Waktu Konfirmasi</th>
        </tr>
      </thead>
      <tbody>
        @forelse ($rsvps as $r)
        <tr>
          <td style="padding-left:16px; font-weight:600;">{{ $r.house_no }}</td>
          <td>
            <span style="font-weight:600; color:var(--pg-text);">{{ $r.full_name }}</span>
            @if ($r.phone)
            <span class="pg-muted" style="font-size:12px; margin-left:6px;">({{ $r.phone }})</span>
            @endif
          </td>
          <td>
            @if ($r.answer == 'hadir')
              <span class="pg-chip pg-chip--ok">✓ Konfirmasi Hadir</span>
            @elseif ($r.answer == 'tidak')
              <span class="pg-chip pg-chip--bad">✕ Tidak Hadir</span>
            @else
              <span class="pg-chip" style="opacity:0.7;">- Belum Respon</span>
            @endif
          </td>
          <td style="text-align:right; padding-right:16px;" class="pg-muted">
            {{ $r.confirmed_at }}
          </td>
        </tr>
        @empty
        <tr>
          <td colspan="4" class="pg-empty">Belum ada data warga terdaftar.</td>
        </tr>
        @endforelse
      </tbody>
    </table>
  </div>
</div>
@endsection

