@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>Rekap Iuran & Tunggakan Warga</h1>
    <p class="pg-muted">Matriks status pembayaran iuran 12 bulan Paguyuban / RT tahun {{ $selected_year }}.</p>
  </div>
  <div style="display:flex; gap:var(--pg-s2); flex-wrap:wrap; align-items:center;">
    <button type="button" class="pg-btn pg-btn--outline" onclick="copyBroadcastText()" id="btnCopyWa">
      📢 Salin Teks WhatsApp
    </button>
    @if ($can_manage)
    <form method="POST" action="/iuran/sinkronkan" style="margin:0;" onsubmit="return confirm('Sinkronkan tagihan bulanan otomatis dari awal tahun/tarif hingga bulan ini bagi seluruh warga yang belum memiliki tagihan?');">
      @csrf
      <button type="submit" class="pg-btn pg-btn--outline" title="Terbitkan otomatis tagihan bulan berjalan untuk seluruh warga aktif yang belum memiliki tagihan">
        ⚡ Sinkronkan Tagihan Bulan Ini
      </button>
    </form>
    <a href="/iuran/tarif" class="pg-btn pg-btn--outline">Atur Tarif Iuran</a>
    <a href="/iuran/generate" class="pg-btn">+ Terbitkan Tagihan</a>
    @endif
  </div>
</div>

<!-- 4 Kartu Statistik Finansial Iuran -->
<div class="pg-grid-4">
  <div class="pg-card">
    <div class="pg-card__title">Total Tagihan {{ $selected_year }}</div>
    <div class="pg-stat__value">{{ $total_billed_raw | rupiah }}</div>
    <div class="pg-muted">{{ $total_invoices_count }} tagihan diterbitkan</div>
  </div>
  <div class="pg-card">
    <div class="pg-card__title">Iuran Terkumpul</div>
    <div class="pg-stat__value" style="color:var(--pg-ok-fg);">{{ $total_collected_raw | rupiah }}</div>
    <div class="pg-muted">{{ $paid_invoices_count }} tagihan lunas</div>
  </div>
  <div class="pg-card">
    <div class="pg-card__title">Total Tunggakan</div>
    <div class="pg-stat__value" style="color:var(--pg-bad-fg);">{{ $total_arrears_raw | rupiah }}</div>
    <div class="pg-muted">{{ $unpaid_members_count }} rumah menunggak</div>
  </div>
  <div class="pg-card">
    <div class="pg-card__title">Kepatuhan Warga</div>
    <div class="pg-stat__value">{{ $compliance_percentage }}%</div>
    <div class="pg-progress" style="margin-top:8px;">
      <span style="width: {{ $compliance_percentage }}%;"></span>
    </div>
  </div>
</div>

<!-- Filter & Pencarian Warga / Blok -->
<div class="pg-card" style="padding:16px;">
  <form method="GET" action="/iuran" style="display:flex; gap:var(--pg-s3); align-items:center; flex-wrap:wrap;">
    <div style="min-width:140px;">
      <label for="year_select" style="display:block; font-size:12px; font-weight:600; margin-bottom:4px;">Tahun Kalender:</label>
      <select id="year_select" name="year" class="pg-select" style="width:100%; height:38px;" onchange="this.form.submit()">
        @foreach ($available_years as $y)
        <option value="{{ $y }}" @selected($y == $selected_year)>Tahun {{ $y }}</option>
        @endforeach
      </select>
    </div>
    <div style="flex:1; min-width:220px;">
      <label for="search_input" style="display:block; font-size:12px; font-weight:600; margin-bottom:4px;">Cari Warga / No. Rumah:</label>
      <input type="text" id="search_input" name="q" value="{{ $query_search ?? '' }}" placeholder="Ketik nama atau blok rumah..." class="pg-input" style="height:38px;">
    </div>
    <div style="align-self:flex-end;">
      <button type="submit" class="pg-btn pg-btn--sm" style="height:38px; padding:0 var(--pg-s4);">Terapkan Filter</button>
      @if ($query_search)
      <a href="/iuran?year={{ $selected_year }}" class="pg-btn pg-btn--outline pg-btn--sm" style="height:38px; margin-left:6px;">Reset</a>
      @endif
    </div>
  </form>
</div>

<!-- Matriks Kotak 12 Bulan (Square Grid Ledger) -->
<div class="pg-card" style="padding:0; overflow:hidden;">
  <div style="padding:16px 20px; border-bottom:1px solid var(--pg-border); display:flex; justify-content:space-between; align-items:center; flex-wrap:wrap; gap:10px;">
    <h2 style="font-size:16px;">Matriks Kartu Iuran Warga (12 Bulan)</h2>
    <div class="pg-matrix-legend">
      <div class="pg-matrix-legend-item">
        <span class="pg-square-cell pg-square-cell--paid" style="width:20px; height:20px; font-size:10px;">✓</span>
        <span>Lunas</span>
      </div>
      <div class="pg-matrix-legend-item">
        <span class="pg-square-cell pg-square-cell--unpaid" style="width:20px; height:20px; font-size:10px;">!</span>
        <span>Belum Lunas @if ($can_manage) (Klik untuk bayar) @endif</span>
      </div>
      <div class="pg-matrix-legend-item">
        <span class="pg-square-cell pg-square-cell--empty" style="width:20px; height:20px; font-size:10px;">-</span>
        <span>Belum Terbit</span>
      </div>
    </div>
  </div>

  <div class="pg-table-wrap">
    <table class="pg-table pg-matrix-table" aria-label="Tabel Matriks Rekap Iuran 12 Bulan">
      <thead>
        <tr>
          <th class="text-left" style="min-width:110px; padding-left:16px;">No. Rumah</th>
          <th class="text-left" style="min-width:180px;">Nama Warga</th>
          <th style="width:42px;">Jan</th>
          <th style="width:42px;">Feb</th>
          <th style="width:42px;">Mar</th>
          <th style="width:42px;">Apr</th>
          <th style="width:42px;">Mei</th>
          <th style="width:42px;">Jun</th>
          <th style="width:42px;">Jul</th>
          <th style="width:42px;">Agu</th>
          <th style="width:42px;">Sep</th>
          <th style="width:42px;">Okt</th>
          <th style="width:42px;">Nov</th>
          <th style="width:42px;">Des</th>
          <th class="text-right" style="min-width:110px;">Total Terbayar</th>
          <th class="text-right" style="min-width:110px; padding-right:16px;">Tunggakan</th>
        </tr>
      </thead>
      <tbody>
        @forelse ($matrix_rows as $row)
        <tr>
          <td class="text-left" style="padding-left:16px;">
            <span style="font-weight:600;">{{ $row.house_no }}</span>
          </td>
          <td class="text-left">
            <span style="font-weight:600; color:var(--pg-text);">{{ $row.full_name }}</span>
            @if ($row.phone)
            <div class="pg-muted" style="font-size:12px;">{{ $row.phone }}</div>
            @endif
          </td>
          
          @foreach ($row.months as $cell)
          <td>
            @if ($cell.status == 'lunas')
              <span class="pg-square-cell pg-square-cell--paid" title="{{ $cell.month_name }}: Lunas {{ $cell.amount_raw | rupiah }} ({{ $cell.paid_date ?? 'Lunas' }})">✓</span>
            @elseif ($cell.status == 'belum')
              @if ($can_manage && $cell.invoice_id)
                <a href="/iuran/bayar/{{ $cell.invoice_id }}" class="pg-square-cell pg-square-cell--unpaid" title="{{ $cell.month_name }}: Belum Lunas ({{ $cell.amount_raw | rupiah }}). Klik untuk catat pembayaran!">!</a>
              @else
                <span class="pg-square-cell pg-square-cell--unpaid" title="{{ $cell.month_name }}: Belum Lunas ({{ $cell.amount_raw | rupiah }})">!</span>
              @endif
            @else
              <span class="pg-square-cell pg-square-cell--empty" title="{{ $cell.month_name }}: Belum Terbit">-</span>
            @endif
          </td>
          @endforeach

          <td class="text-right num" style="font-weight:600; color:var(--pg-ok-fg);">
            {{ $row.total_paid_raw | rupiah }}
          </td>
          <td class="text-right num" style="font-weight:700; color:var(--pg-bad-fg); padding-right:16px;">
            {{ $row.total_unpaid_raw | rupiah }}
          </td>
        </tr>
        @empty
        <tr>
          <td colspan="16" class="pg-empty">
            <p>Belum ada data warga terdaftar atau tagihan di tahun {{ $selected_year }}.</p>
            @if ($can_manage)
            <a href="/iuran/generate" class="pg-btn pg-btn--sm">+ Terbitkan Tagihan Pertama</a>
            @endif
          </td>
        </tr>
        @endforelse
      </tbody>
    </table>
  </div>
</div>

<!-- Modal / Hidden Box Broadcast WhatsApp -->
<div id="waBroadcastContainer" style="display:none;">
  <textarea id="waBroadcastText">
*REKAP IURAN & KAS WARGA — {{ $org_name }}*
Periode Tahun: {{ $selected_year }}

📊 *Ringkasan Finansial Lingkungan:*
• Total Tagihan: {{ $total_billed_raw | rupiah }}
• Iuran Terkumpul: {{ $total_collected_raw | rupiah }}
• Total Tunggakan: {{ $total_arrears_raw | rupiah }}
• Persentase Kepatuhan: {{ $compliance_percentage }}%

Bagi Bapak/Ibu warga yang memiliki tunggakan iuran bulanan, mohon dapat melakukan pelunasan melalui Bendahara Paguyuban/RT. Terima kasih atas partisipasi aktif seluruh warga.
  </textarea>
</div>

<script>
function copyBroadcastText() {
  const textElem = document.getElementById('waBroadcastText');
  const btn = document.getElementById('btnCopyWa');
  if (!textElem) return;

  const content = textElem.value.trim();
  navigator.clipboard.writeText(content).then(() => {
    const originalText = btn.innerHTML;
    btn.innerHTML = '✅ Teks Berhasil Disalin!';
    btn.style.borderColor = 'var(--pg-ok-fg)';
    btn.style.color = 'var(--pg-ok-fg)';
    setTimeout(() => {
      btn.innerHTML = originalText;
      btn.style.borderColor = '';
      btn.style.color = '';
    }, 3000);
  }).catch(() => {
    alert('Salin manual teks:\n\n' + content);
  });
}
</script>
@endsection

