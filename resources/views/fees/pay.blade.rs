@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>Catat Pembayaran Iuran</h1>
    <p class="pg-muted">Pelunasan tagihan iuran warga secara tunai atau transfer rekening.</p>
  </div>
  <div>
    <a href="/iuran" class="pg-btn pg-btn--outline">← Kembali ke Rekap Iuran</a>
  </div>
</div>

<div class="pg-card" style="max-width:640px; margin:0 auto;">
  <!-- Rincian Tagihan -->
  <div style="background:var(--pg-track); border-radius:var(--pg-r-card); padding:16px; margin-bottom:20px;">
    <div style="display:flex; justify-content:space-between; margin-bottom:8px;">
      <span class="pg-muted">Nama Warga:</span>
      <strong>{{ $invoice_member_name }}</strong>
    </div>
    <div style="display:flex; justify-content:space-between; margin-bottom:8px;">
      <span class="pg-muted">No. Rumah / Blok:</span>
      <strong>{{ $invoice_house_no }}</strong>
    </div>
    <div style="display:flex; justify-content:space-between; margin-bottom:8px;">
      <span class="pg-muted">Pos Iuran:</span>
      <span>{{ $invoice_plan_name }}</span>
    </div>
    <div style="display:flex; justify-content:space-between; margin-bottom:8px;">
      <span class="pg-muted">Periode Tagihan:</span>
      <span class="pg-chip pg-chip--warn">{{ $invoice_period_label }}</span>
    </div>
    <div style="display:flex; justify-content:space-between; font-size:16px; border-top:1px dashed var(--pg-border); padding-top:10px; margin-top:10px;">
      <span style="font-weight:700;">Nominal Pelunasan:</span>
      <strong style="color:var(--pg-primary);">{{ $invoice_amount_formatted }}</strong>
    </div>
  </div>

  <!-- Form Pelunasan -->
  <form method="POST" action="/iuran/bayar/{{ $invoice_id }}">
    @csrf

    <div class="pg-field">
      <label for="method">Metode Pembayaran</label>
      <select id="method" name="method" class="pg-input" style="height:42px;" required>
        <option value="tunai" selected>Tunai (Cash ke Bendahara/Ketua)</option>
        <option value="transfer">Transfer Bank / QRIS</option>
      </select>
    </div>

    <div class="pg-field">
      <label for="paid_on">Tanggal Pembayaran</label>
      <input type="date" id="paid_on" name="paid_on" class="pg-input" value="{{ $today_date }}" required>
    </div>

    <div class="pg-field">
      <label for="note">Catatan Tambahan (Opsional)</label>
      <input type="text" id="note" name="note" class="pg-input" placeholder="Contoh: Dititipkan Bu RT / Bukti Transfer BCA terlampir">
    </div>

    <div class="pg-alert pg-alert--ok" style="font-size:13px; margin-top:16px;">
      ✨ Pembayaran ini akan secara otomatis memperbarui status tagihan menjadi <strong>LUNAS</strong> dan mencatat transaksi kas masuk di buku kas umum paguyuban.
    </div>

    <div class="pg-form-actions">
      <a href="/iuran" class="pg-btn pg-btn--outline">Batal</a>
      <button type="submit" class="pg-btn">Konfirmasi & Lunaskan</button>
    </div>
  </form>
</div>
@endsection

