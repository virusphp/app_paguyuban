@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>Kelola Template Surat & Undangan</h1>
    <p class="pg-muted">Buat dan simpan kerangka format surat kustom untuk mempermudah penulisan korespondensi paguyuban.</p>
  </div>
  <div>
    <a href="/surat" class="pg-btn pg-btn--outline">← Kembali ke Arsip Surat</a>
  </div>
</div>

<div class="pg-grid-2">
  <!-- Daftar Template Tersimpan -->
  <div class="pg-card" style="padding:0; overflow:hidden;">
    <div style="padding:16px 20px; border-bottom:1px solid var(--pg-border);">
      <h2 style="font-size:16px; margin:0;">Daftar Template Kustom Organisasi</h2>
    </div>

    <div style="padding:16px 20px; display:flex; flex-direction:column; gap:16px;">
      @forelse ($templates as $tmpl)
      <div style="border:1px solid var(--pg-border); border-radius:var(--pg-r-card); padding:16px; background:var(--pg-track);">
        <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:8px;">
          <h3 style="font-size:15px; font-weight:700; margin:0;">{{ $tmpl.name }}</h3>
          <span class="pg-muted" style="font-size:12px;">{{ $tmpl.created_at_formatted }}</span>
        </div>
        <div style="font-size:13px; line-height:1.5; color:var(--pg-text); max-height:80px; overflow:hidden; text-overflow:ellipsis; white-space:pre-line;">
          {{ $tmpl.body }}
        </div>
      </div>
      @empty
      <div class="pg-empty">
        <p>Belum ada template kustom yang dibuat.</p>
      </div>
      @endforelse
    </div>
  </div>

  <!-- Form Tambah Template Baru -->
  <div class="pg-card">
    <h2 class="pg-card__title">+ Tambah Template Baru</h2>
    <form method="POST" action="/surat/template">
      @csrf

      <div class="pg-field">
        <label for="name">Nama / Judul Template (wajib)</label>
        <input type="text" id="name" name="name" class="pg-input" placeholder="Contoh: Surat Izin Penggunaan Balai Warga" required>
      </div>

      <div class="pg-field">
        <label for="body">Format / Isi Template (wajib)</label>
        <textarea id="body" name="body" class="pg-input" style="height:220px; line-height:1.5; padding:10px;" placeholder="Tuliskan format isi surat kustom..." required></textarea>
      </div>

      <div class="pg-form-actions" style="margin-top:16px;">
        <button type="submit" class="pg-btn">Simpan Template</button>
      </div>
    </form>
  </div>
</div>
@endsection

