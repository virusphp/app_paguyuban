@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>Catat Pengeluaran Kas (Belanja)</h1>
    <p class="pg-muted">Catat pengeluaran dana untuk konsumsi, perlengkapan, hadiah lomba, atau operasional lingkungan.</p>
  </div>
  <div>
    <a href="/kas" class="pg-btn pg-btn--outline">← Kembali ke Buku Kas</a>
  </div>
</div>

<div class="pg-card" style="max-width:640px; margin:0 auto;">
  <form method="POST" action="/kas/keluar">
    @csrf

    <div class="pg-field">
      <label for="category">Kategori Pengeluaran</label>
      <select id="category" name="category" class="pg-input" style="height:42px;" required>
        <option value="konsumsi" selected>Konsumsi & Makanan</option>
        <option value="perlengkapan">Perlengkapan & Tenda/Alat</option>
        <option value="hadiah">Hadiah Lomba & Door Prize</option>
        <option value="operasional">Operasional & Kebersihan</option>
        <option value="lainnya">Pengeluaran Lainnya</option>
      </select>
    </div>

    <div class="pg-field">
      <label for="amount_idr">Nominal Pengeluaran (Rupiah)</label>
      <input type="number" id="amount_idr" name="amount_idr" class="pg-input" placeholder="Contoh: 150000" min="1000" step="500" required>
      <div class="pg-field__help">Nominal rupiah bulat tanpa tanda titik atau koma.</div>
    </div>

    <div class="pg-field">
      <label for="description">Keterangan Belanja / Pengeluaran</label>
      <input type="text" id="description" name="description" class="pg-input" placeholder="Contoh: Beli Lampu Penerangan Jalan Blok B 4 Buah" required>
    </div>

    <div class="pg-field">
      <label for="donor_name">Penerima / Nama Toko (Opsional)</label>
      <input type="text" id="donor_name" name="donor_name" class="pg-input" placeholder="Contoh: Toko Listrik Jaya Abadi / Warung Bu Ani">
    </div>

    <div class="pg-field">
      <label for="occurred_on">Tanggal Pengeluaran</label>
      <input type="date" id="occurred_on" name="occurred_on" class="pg-input" value="{{ $today_date }}" required>
    </div>

    <div class="pg-form-actions">
      <a href="/kas" class="pg-btn pg-btn--outline">Batal</a>
      <button type="submit" class="pg-btn" style="background:var(--pg-bad-fg);">Simpan Kas Keluar</button>
    </div>
  </form>
</div>
@endsection

