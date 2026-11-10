@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>Terbitkan Tagihan Iuran Massal</h1>
    <p class="pg-muted">Buat tagihan iuran serentak untuk seluruh warga aktif yang terdaftar.</p>
  </div>
  <div>
    <a href="/iuran" class="pg-btn pg-btn--outline">← Kembali ke Rekap Iuran</a>
  </div>
</div>

<div class="pg-card" style="max-width:680px; margin:0 auto;">
  <form method="POST" action="/iuran/generate">
    @csrf

    <div class="pg-field">
      <label for="fee_plan_id">Pilih Jenis Tarif Iuran</label>
      <select id="fee_plan_id" name="fee_plan_id" class="pg-input" style="height:42px;" required>
        @forelse ($fee_plans as $plan)
        <option value="{{ $plan.id }}">{{ $plan.name }} — {{ $plan.amount_formatted }} ({{ $plan.period }})</option>
        @empty
        <option value="" disabled selected>Belum ada tarif terdaftar. Silakan atur tarif terlebih dahulu.</option>
        @endforelse
      </select>
    </div>

    <div class="pg-field" style="margin-bottom:var(--pg-s4);">
      <label class="pg-label" style="font-weight:700;">Cakupan Periode Penerbitan</label>
      <div style="display:flex; flex-direction:column; gap:8px; margin-top:6px;">
        <label style="display:flex; align-items:center; gap:8px; cursor:pointer; font-size:14px;">
          <input type="radio" name="scope" value="up_to_current" checked onchange="toggleMonthSelect(this.value)">
          <span><strong>Otomatis Hingga Bulan Berjalan Saat Ini</strong> (Januari s/d Bulan Ini — Direkomendasikan)</span>
        </label>
        <label style="display:flex; align-items:center; gap:8px; cursor:pointer; font-size:14px;">
          <input type="radio" name="scope" value="single" onchange="toggleMonthSelect(this.value)">
          <span>Hanya 1 Bulan Tertentu Saja</span>
        </label>
        <label style="display:flex; align-items:center; gap:8px; cursor:pointer; font-size:14px;">
          <input type="radio" name="scope" value="full_year" onchange="toggleMonthSelect(this.value)">
          <span>Satu Tahun Penuh Sekaligus (12 Bulan / Januari s/d Desember)</span>
        </label>
      </div>
    </div>

    <div class="pg-grid-2" style="margin-bottom:var(--pg-s3);">
      <div class="pg-field" id="box_month_select" style="display:none;">
        <label for="month">Pilih Bulan</label>
        <select id="month" name="month" class="pg-input" style="height:42px;">
          <option value="1" @selected($current_month == 1)>Januari</option>
          <option value="2" @selected($current_month == 2)>Februari</option>
          <option value="3" @selected($current_month == 3)>Maret</option>
          <option value="4" @selected($current_month == 4)>April</option>
          <option value="5" @selected($current_month == 5)>Mei</option>
          <option value="6" @selected($current_month == 6)>Juni</option>
          <option value="7" @selected($current_month == 7)>Juli</option>
          <option value="8" @selected($current_month == 8)>Agustus</option>
          <option value="9" @selected($current_month == 9)>September</option>
          <option value="10" @selected($current_month == 10)>Oktober</option>
          <option value="11" @selected($current_month == 11)>November</option>
          <option value="12" @selected($current_month == 12)>Desember</option>
        </select>
      </div>

      <div class="pg-field">
        <label for="year">Periode Tahun Kalender</label>
        <input type="number" id="year" name="year" class="pg-input" value="{{ $current_year }}" min="2020" max="2035" required>
      </div>
    </div>

    <div class="pg-alert pg-alert--ok" style="margin-top:16px;">
      ℹ️ Sistem akan otomatis mendistribusikan tagihan ke <strong>{{ $active_members_count }}</strong> warga aktif. Tagihan yang sudah pernah terbit di periode yang sama tidak akan terduplikasi.
    </div>

    <div class="pg-form-actions">
      <a href="/iuran" class="pg-btn pg-btn--outline">Batal</a>
      <button type="submit" class="pg-btn">Terbitkan Tagihan Sekarang</button>
    </div>
  </form>
</div>

<script>
function toggleMonthSelect(scope) {
  const box = document.getElementById('box_month_select');
  if (scope === 'single') {
    box.style.display = 'block';
  } else {
    box.style.display = 'none';
  }
}
</script>
@endsection

