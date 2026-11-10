@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>Catat Penerimaan Kas Masuk</h1>
    <p class="pg-muted">Catat dana masuk paguyuban/RT (iuran panitia, donasi warga, saldo awal, atau hasil lelang).</p>
  </div>
  <div>
    <a href="/kas" class="pg-btn pg-btn--outline">← Kembali ke Buku Kas</a>
  </div>
</div>

<div class="pg-card" style="max-width:640px; margin:0 auto;">
  <form method="POST" action="/kas/masuk">
    @csrf

    <div class="pg-field">
      <label for="category">Kategori Pemasukan</label>
      <select id="category" name="category" class="pg-input" style="height:42px;" required>
        <option value="donasi" selected>Donasi Warga</option>
        <option value="iuran_panitia">Iuran Panitia</option>
        <option value="lelang">Hasil Lelang</option>
        <option value="saldo_awal">Saldo Awal Tahun Lalu</option>
        <option value="lainnya">Pemasukan Lainnya</option>
      </select>
    </div>

    <div class="pg-field">
      <label for="amount_idr">Nominal Penerimaan (Rupiah)</label>
      <input type="number" id="amount_idr" name="amount_idr" class="pg-input" placeholder="Contoh: 100000" min="1000" step="500" required>
      <div class="pg-field__help">Nominal rupiah bulat tanpa tanda titik atau koma (contoh: 100000).</div>
    </div>

    <div class="pg-field">
      <label for="description">Keterangan / Maksud Penerimaan</label>
      <input type="text" id="description" name="description" class="pg-input" placeholder="Contoh: Bantuan dana konsumsi tirakatan HUT RI" required>
    </div>

    <div class="pg-field">
      <label for="donor_name">Nama Donatur / Sumber Dana (Opsional)</label>
      <input type="text" id="donor_name" name="donor_name" class="pg-input" placeholder="Contoh: Bpk. Bambang Sudarsono">
      <div style="margin-top:6px; display:flex; align-items:center; gap:8px;">
        <input type="checkbox" id="chk_anonim" onchange="toggleAnonim(this)">
        <label for="chk_anonim" style="font-size:13px; font-weight:normal; margin:0; cursor:pointer;">
          Donatur Anonim (Hamba Allah) — Nama disamarkan di LPJ publik
        </label>
      </div>
    </div>

    <div class="pg-field">
      <label for="occurred_on">Tanggal Penerimaan</label>
      <input type="date" id="occurred_on" name="occurred_on" class="pg-input" value="{{ $today_date }}" required>
    </div>

    <div class="pg-form-actions">
      <a href="/kas" class="pg-btn pg-btn--outline">Batal</a>
      <button type="submit" class="pg-btn">Simpan Kas Masuk</button>
    </div>
  </form>
</div>

<script>
function toggleAnonim(chk) {
  const donorInput = document.getElementById('donor_name');
  if (chk.checked) {
    donorInput.value = 'Hamba Allah';
    donorInput.readOnly = true;
  } else {
    donorInput.value = '';
    donorInput.readOnly = false;
  }
}
</script>
@endsection

