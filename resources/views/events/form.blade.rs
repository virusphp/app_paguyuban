@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>@if ($is_edit)Ubah Agenda Kegiatan@else Tambah Kegiatan Baru@endif</h1>
    <p class="pg-muted">Jadwalkan rapat warga, kerja bakti, pengajian, atau perlombaan lingkungan.</p>
  </div>
  <div>
    <a href="/kegiatan" class="pg-btn pg-btn--outline">← Kembali ke Daftar Kegiatan</a>
  </div>
</div>

<div class="pg-card" style="max-width:680px; margin:0 auto;">
  <form method="POST" action="@if ($is_edit)/kegiatan/{{ $event.id }}@else/kegiatan/baru@endif">
    @csrf

    <div class="pg-field">
      <label for="title">Nama Kegiatan / Agenda Acara (wajib)</label>
      <input type="text" id="title" name="title" class="pg-input" value="{{ $event.title }}" placeholder="Contoh: Rapat Evaluasi Kas & Tirakatan HUT RI" required>
    </div>

    <div class="pg-grid-2">
      <div class="pg-field">
        <label for="starts_at">Waktu & Tanggal Mulai (wajib)</label>
        <input type="datetime-local" id="starts_at" name="starts_at" class="pg-input" value="@if ($is_edit){{ $event.starts_at_input }}@else{{ $default_starts_at }}@endif" required>
      </div>

      <div class="pg-field">
        <label for="location">Tempat / Lokasi Acara</label>
        <input type="text" id="location" name="location" class="pg-input" value="{{ $event.location }}" placeholder="Contoh: Balai Warga Paguyuban Kanfer 2">
      </div>
    </div>

    <div class="pg-field">
      <label for="description">Keterangan / Susunan Acara</label>
      <textarea id="description" name="description" class="pg-input" style="height:110px; padding-top:8px;" placeholder="Tuliskan agenda rapat, perlengkapan yang perlu dibawa warga, atau informasi tambahan...">{{ $event.description }}</textarea>
    </div>

    @if ($is_edit)
    <div class="pg-field">
      <label for="status">Status Kegiatan</label>
      <select id="status" name="status" class="pg-input" style="height:42px;">
        <option value="aktif" @selected($event.status == 'aktif')>Aktif (Berjalan / Mendatang)</option>
        <option value="selesai" @selected($event.status == 'selesai')>Selesai (Terlaksana)</option>
        <option value="batal" @selected($event.status == 'batal')>Batal (Dibatalkan)</option>
      </select>
    </div>
    @endif

    <div class="pg-form-actions">
      <a href="/kegiatan" class="pg-btn pg-btn--outline">Batal</a>
      <button type="submit" class="pg-btn">@if ($is_edit)Simpan Perubahan@else Terbitkan Kegiatan@endif</button>
    </div>
  </form>
</div>
@endsection

