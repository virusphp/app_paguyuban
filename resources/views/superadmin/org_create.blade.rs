@extends('layouts/dashboard.blade.rs')

@section('content')
<header class="pg-page-head" style="display:flex; justify-content:space-between; align-items:center; margin-bottom:var(--pg-s5);">
  <div>
    <h1 class="pg-page-head__title" style="font-size:22px; font-weight:700; margin:0;">Tambah Organisasi Baru</h1>
    <p class="pg-page-head__subtitle" style="color:var(--pg-muted); font-size:13px; margin:4px 0 0 0;">Daftarkan paguyuban atau RT/RW baru beserta akun admin pertamanya.</p>
  </div>
  <div class="pg-page-head__actions">
    <a href="/sa/organisasi" class="pg-btn pg-btn--secondary">
      Kembali ke Daftar
    </a>
  </div>
</header>

<div style="max-width: 640px;">
  <div class="pg-card" style="background:var(--pg-surface); border:1px solid var(--pg-border); border-radius:var(--pg-r-card); padding:var(--pg-s6);">
    <form action="/sa/organisasi/baru" method="POST" style="display:flex; flex-direction:column; gap:var(--pg-s5);">
      @csrf
      
      @if ($error)
      <div class="pg-notice pg-notice--bad" style="padding:var(--pg-s3) var(--pg-s4); border-radius:var(--pg-r-btn); background:var(--pg-bad-bg); color:var(--pg-bad-fg); font-size:13px;">
        {{ $error }}
      </div>
      @endif

      <div>
        <h2 style="font-size:16px; font-weight:700; color:var(--pg-text); margin-bottom:var(--pg-s3); border-bottom:1px solid var(--pg-border-soft, #f1f5f9); padding-bottom:var(--pg-s2);">
          1. Data Organisasi
        </h2>
        
        <div style="display:flex; flex-direction:column; gap:var(--pg-s4);">
          <div class="pg-field">
            <label for="org_name" class="pg-label">Nama Organisasi (wajib)</label>
            <input type="text" id="org_name" name="org_name" class="pg-input" required placeholder="Contoh: Paguyuban Kanfer 2 atau RT 05 RW 09" value="{{ $old_name ?? '' }}">
            <span class="pg-help" style="font-size:12px; color:var(--pg-muted);">Nama resmi paguyuban, rukun warga, atau rukun tetangga.</span>
          </div>

          <div class="pg-field">
            <label for="org_type" class="pg-label">Jenis Organisasi (wajib)</label>
            <select id="org_type" name="org_type" class="pg-input" required>
              <option value="paguyuban">Paguyuban Warga</option>
              <option value="rt">Rukun Tetangga (RT)</option>
              <option value="rw">Rukun Warga (RW)</option>
            </select>
          </div>
        </div>
      </div>

      <div style="margin-top:var(--pg-s3);">
        <h2 style="font-size:16px; font-weight:700; color:var(--pg-text); margin-bottom:var(--pg-s3); border-bottom:1px solid var(--pg-border-soft, #f1f5f9); padding-bottom:var(--pg-s2);">
          2. Akun Admin Utama (Ketua / Pengelola)
        </h2>

        <div style="display:flex; flex-direction:column; gap:var(--pg-s4);">
          <div class="pg-field">
            <label for="admin_name" class="pg-label">Nama Lengkap Admin (wajib)</label>
            <input type="text" id="admin_name" name="admin_name" class="pg-input" required placeholder="Contoh: Bpk. Bambang Sudarsono" value="{{ $old_admin_name ?? '' }}">
          </div>

          <div class="pg-field">
            <label for="admin_phone" class="pg-label">Nomor HP / WhatsApp (wajib)</label>
            <input type="tel" id="admin_phone" name="admin_phone" class="pg-input" required placeholder="Contoh: 081234567890" value="{{ $old_admin_phone ?? '' }}">
            <span class="pg-help" style="font-size:12px; color:var(--pg-muted);">Nomor ini akan digunakan admin untuk masuk ke aplikasi.</span>
          </div>

          <div class="pg-field">
            <label for="password" class="pg-label">Kata Sandi Awal</label>
            <input type="text" id="password" name="password" class="pg-input" value="password123">
            <span class="pg-help" style="font-size:12px; color:var(--pg-muted);">Kata sandi default untuk akun admin baru (dapat diubah nanti).</span>
          </div>
        </div>
      </div>

      <div style="display:flex; gap:var(--pg-s3); margin-top:var(--pg-s4); border-top:1px solid var(--pg-border-soft, #f1f5f9); padding-top:var(--pg-s4); flex-wrap:wrap;">
        <button type="submit" class="pg-btn pg-btn--primary">
          Simpan & Daftarkan Organisasi
        </button>
        <a href="/sa/organisasi" class="pg-btn pg-btn--secondary">
          Batal
        </a>
      </div>

    </form>
  </div>
</div>
@endsection
