@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>@if ($is_edit)Ubah Pengumuman@else Terbitkan Pengumuman Baru@endif</h1>
    <p class="pg-muted">Tulis pemberitahuan resmi untuk dibaca oleh seluruh warga paguyuban / RT.</p>
  </div>
  <div>
    <a href="/info" class="pg-btn pg-btn--outline">← Kembali ke Papan Informasi</a>
  </div>
</div>

<div class="pg-card" style="max-width:680px; margin:0 auto;">
  <form method="POST" action="@if ($is_edit)/info/{{ $announcement.id }}@else/info/baru@endif">
    @csrf

    <div class="pg-field">
      <label for="title">Judul Pengumuman (wajib)</label>
      <input type="text" id="title" name="title" class="pg-input" value="{{ $announcement.title }}" placeholder="Contoh: Pemberitahuan Kerja Bakti Menjelang Musim Hujan" required>
    </div>

    <div class="pg-field">
      <label for="body">Isi Pengumuman Lengkap (wajib)</label>
      <textarea id="body" name="body" class="pg-input" style="height:180px; padding-top:10px; line-height:1.5;" placeholder="Tuliskan isi pengumuman atau instruksi untuk warga..." required>{{ $announcement.body }}</textarea>
    </div>

    <div style="margin:16px 0; display:flex; align-items:center; gap:8px;">
      <input type="checkbox" id="pinned" name="pinned" value="true" @if ($announcement.pinned)checked@endif>
      <label for="pinned" style="font-size:13px; font-weight:600; margin:0; cursor:pointer;">
        📌 Sematkan Pengumuman Ini (Tampilkan Selalu di Paling Atas)
      </label>
    </div>

    <div class="pg-form-actions">
      <a href="/info" class="pg-btn pg-btn--outline">Batal</a>
      <button type="submit" class="pg-btn">@if ($is_edit)Simpan Perubahan@else Terbitkan Sekarang@endif</button>
    </div>
  </form>
</div>
@endsection

