@extends('layouts/dashboard.blade.rs')

@section('content')
<header class="pg-page-head">
  <div>
    <h1 class="pg-page-head__title">Impor Data Warga Massal (CSV)</h1>
    <p class="pg-page-head__subtitle">Masukkan data puluhan warga dan rumah sekaligus ke {{ $org_name }}.</p>
  </div>
  <div class="pg-page-head__actions">
    <a href="/anggota" class="pg-btn pg-btn--secondary">
      Kembali ke Daftar Warga
    </a>
  </div>
</header>

<div style="max-width: 720px;">
  <!-- Panduan Format CSV -->
  <div class="pg-card" style="margin-bottom: var(--pg-s5); background: var(--pg-bg); border-left: 4px solid var(--pg-primary);">
    <h2 style="font-size:15px; font-weight:700; color:var(--pg-text); margin-bottom:var(--pg-s2);">
      📋 Format Standar CSV
    </h2>
    <p style="font-size:13px; color:var(--pg-muted); margin-bottom:var(--pg-s3);">
      Susun kolom data dengan urutan: <code>nomor_rumah,nama_lengkap,nomor_hp,status</code>
    </p>
    <pre style="background:var(--pg-surface); padding:var(--pg-s3); border-radius:8px; border:1px solid var(--pg-border); font-size:13px; overflow-x:auto;">
nomor_rumah,nama_lengkap,nomor_hp,status
B-01,Bambang Sudarsono,081234567890,aktif
B-02,Siti Rahayu,081234567891,aktif
B-03,Ahmad Fauzi,,menunggu
C-05,Hendra Gunawan,081300000001,aktif</pre>
    <div style="font-size:12px; color:var(--pg-muted); margin-top:var(--pg-s2);">
      * Baris header otomatis dilewati. Kolom <code>nomor_hp</code> boleh dikosongkan.
    </div>
  </div>

  @if ($error)
  <div class="pg-notice pg-notice--bad" style="margin-bottom:var(--pg-s5);">
    {{ $error }}
  </div>
  @endif

  <!-- Formulir Impor CSV -->
  <div class="pg-card">
    <form action="/anggota/impor" method="POST" style="display:flex; flex-direction:column; gap:var(--pg-s5);">
      @csrf

      <div class="pg-field">
        <label for="csv_content" class="pg-label">Tempel Data CSV di Sini (Direkomendasikan)</label>
        <textarea id="csv_content" name="csv_content" class="pg-input" rows="10" placeholder="nomor_rumah,nama_lengkap,nomor_hp,status&#10;B-01,Bpk. Bambang,081234567890,aktif" style="font-family:monospace; font-size:13px; line-height:1.5;"></textarea>
        <span class="pg-help">Anda bisa langsung menyalin kolom dari Microsoft Excel atau Google Spreadsheet.</span>
      </div>

      <div style="display:flex; gap:var(--pg-s3); border-top:1px solid var(--pg-border-soft); padding-top:var(--pg-s4); flex-wrap:wrap;">
        <button type="submit" class="pg-btn pg-btn--primary">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="17 8 12 3 7 8"></polyline><line x1="12" y1="3" x2="12" y2="15"></line></svg>
          Mulai Proses Impor
        </button>
        <a href="/anggota" class="pg-btn pg-btn--secondary">
          Batal
        </a>
      </div>

    </form>
  </div>
</div>
@endsection

