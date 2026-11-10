@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>Catat Donasi Barang (Natura)</h1>
    <p class="pg-muted">Bantuan barang atau logistik dari warga/panitia (tidak dinilai uang, dimuat pada Bagian D LPJ).</p>
  </div>
  <div>
    <a href="/kas" class="pg-btn pg-btn--outline">← Kembali ke Buku Kas</a>
  </div>
</div>

<div class="pg-card" style="max-width:640px; margin:0 auto;">
  <form method="POST" action="/kas/donasi-barang">
    @csrf

    <div class="pg-field">
      <label for="donor_name">Nama Donatur / Penyumbang</label>
      <input type="text" id="donor_name" name="donor_name" class="pg-input" placeholder="Contoh: Bpk. H. Agus Santoso" required>
    </div>

    <div class="pg-field">
      <label for="item">Nama Barang / Bantuan</label>
      <input type="text" id="item" name="item" class="pg-input" placeholder="Contoh: Air Mineral Gelas / Snack Kue Tradisional" required>
    </div>

    <div class="pg-field">
      <label for="quantity_text">Jumlah / Kuantitas</label>
      <input type="text" id="quantity_text" name="quantity_text" class="pg-input" placeholder="Contoh: 3 Dus / 40 Kotak" required>
      <div class="pg-field__help">Tuliskan angka beserta satuannya agar jelas saat dimuat di lampiran LPJ.</div>
    </div>

    <div class="pg-alert pg-alert--ok" style="font-size:13px; margin-top:16px;">
      ℹ️ Bantuan barang dicatat secara khusus sebagai kontribusi non-tunai dan tidak menambah saldo kas umum paguyuban.
    </div>

    <div class="pg-form-actions">
      <a href="/kas" class="pg-btn pg-btn--outline">Batal</a>
      <button type="submit" class="pg-btn">Simpan Donasi Barang</button>
    </div>
  </form>
</div>
@endsection

