@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>Pratinjau Laporan Pertanggungjawaban (LPJ)</h1>
    <p class="pg-muted">Format resmi LPJ Keuangan & Kegiatan Paguyuban / RT Tahun {{ $selected_year }}.</p>
  </div>
  <div style="display:flex; gap:var(--pg-s2); flex-wrap:wrap;">
    <a href="/kas?year={{ $selected_year }}" class="pg-btn pg-btn--outline">← Kembali ke Buku Kas</a>
    <a href="/laporan/lpj/{{ $selected_year }}/docx" class="pg-btn pg-btn--outline">📥 Unduh Berkas Word (.docx)</a>
    <button type="button" class="pg-btn" onclick="window.print()">🖨️ Cetak Dokumen / PDF</button>
  </div>
</div>

<!-- Kertas Dokumen LPJ Resmi -->
<div class="pg-card" style="max-width:850px; margin:0 auto; padding:40px; background:#fff; box-shadow:0 4px 12px rgba(0,0,0,0.06);">
  <!-- Kop & Judul Dokumen -->
  <div style="text-align:center; border-bottom:2px solid #000; padding-bottom:16px; margin-bottom:24px;">
    <h2 style="font-size:18px; font-weight:700; margin:0 0 6px; letter-spacing:0.5px;">LAPORAN PERTANGGUNGJAWABAN (LPJ) KEUANGAN</h2>
    <h3 style="font-size:16px; font-weight:700; margin:0 0 4px;">{{ $org_name | uppercase }}</h3>
    <p style="font-size:13px; color:#555; margin:0;">RW 09 KELURAHAN SLAMARAN — PERIODE TAHUN {{ $selected_year }}</p>
  </div>

  <!-- Pendahuluan & Pengantar -->
  <div style="margin-bottom:24px; font-size:14px; line-height:1.6; text-align:justify;">
    <h4 style="font-size:15px; font-weight:700; margin:0 0 8px; border-bottom:1px solid var(--pg-border); padding-bottom:4px;">1. PENDAHULUAN</h4>
    <p style="margin:0 0 10px;">
      Puji syukur kami panjatkan ke hadirat Tuhan Yang Maha Esa atas terselenggaranya seluruh rangkaian kegiatan rutin dan kebersamaan warga di lingkungan {{ $org_name }}. Laporan Pertanggungjawaban (LPJ) ini disusun sebagai bentuk transparansi, akuntabilitas, dan keterbukaan informasi seluruh pengurus kepada seluruh warga.
    </p>
  </div>

  <!-- Ringkasan Finansial Eksekutif -->
  <div style="margin-bottom:24px;">
    <h4 style="font-size:15px; font-weight:700; margin:0 0 8px; border-bottom:1px solid var(--pg-border); padding-bottom:4px;">2. RINGKASAN FINANSIAL KEUANGAN</h4>
    <div class="pg-table-wrap">
      <table class="pg-table" style="width:100%; border:1px solid var(--pg-border);">
        <tbody>
          <tr>
            <td style="font-weight:600; width:50%;">Total Penerimaan (Pemasukan)</td>
            <td class="num" style="font-weight:700; color:var(--pg-ok-fg);">{{ $total_income_formatted }}</td>
          </tr>
          <tr>
            <td style="font-weight:600;">Total Pengeluaran (Belanja)</td>
            <td class="num" style="font-weight:700; color:var(--pg-bad-fg);">{{ $total_expense_formatted }}</td>
          </tr>
          <tr style="background:var(--pg-track);">
            <td style="font-weight:700; font-size:15px;">Sisa Saldo Kas Akhir Periode</td>
            <td class="num" style="font-weight:700; font-size:16px; color:var(--pg-primary);">{{ $final_balance_formatted }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>

  <!-- Bagian A: Pemasukan Kas -->
  <div style="margin-bottom:24px;">
    <h4 style="font-size:15px; font-weight:700; margin:0 0 8px; border-bottom:1px solid var(--pg-border); padding-bottom:4px;">3. BAGIAN A: RINCIAN PEMASUKAN DANA</h4>
    <div class="pg-table-wrap">
      <table class="pg-table" style="width:100%; border:1px solid var(--pg-border); font-size:13px;">
        <thead>
          <tr style="background:var(--pg-track);">
            <th style="width:35px;">No</th>
            <th>Tanggal</th>
            <th>Pos / Kategori</th>
            <th>Donatur / Sumber</th>
            <th>Keterangan</th>
            <th class="num" style="width:130px;">Jumlah (Rp)</th>
          </tr>
        </thead>
        <tbody>
          @forelse ($income_list as $in_tx)
          <tr>
            <td style="text-align:center;">{{ $loop.iteration }}</td>
            <td>{{ $in_tx.date_formatted }}</td>
            <td style="text-transform:capitalize; font-weight:600;">{{ $in_tx.category_label }}</td>
            <td style="font-weight:600;">{{ $in_tx.donor_name ?? '-' }}</td>
            <td>{{ $in_tx.description }}</td>
            <td class="num" style="font-weight:600;">{{ $in_tx.amount_formatted }}</td>
          </tr>
          @empty
          <tr>
            <td colspan="6" class="pg-empty">Tidak ada catatan pemasukan.</td>
          </tr>
          @endforelse
          <tr style="background:var(--pg-track); font-weight:700;">
            <td colspan="5" style="text-align:right;">TOTAL PEMASUKAN (A)</td>
            <td class="num">{{ $total_income_formatted }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>

  <!-- Bagian B: Pengeluaran Kas -->
  <div style="margin-bottom:24px;">
    <h4 style="font-size:15px; font-weight:700; margin:0 0 8px; border-bottom:1px solid var(--pg-border); padding-bottom:4px;">4. BAGIAN B: RINCIAN PENGELUARAN DANA</h4>
    <div class="pg-table-wrap">
      <table class="pg-table" style="width:100%; border:1px solid var(--pg-border); font-size:13px;">
        <thead>
          <tr style="background:var(--pg-track);">
            <th style="width:35px;">No</th>
            <th>Tanggal</th>
            <th>Pos / Kategori</th>
            <th>Penerima / Toko</th>
            <th>Keterangan Belanja</th>
            <th class="num" style="width:130px;">Jumlah (Rp)</th>
          </tr>
        </thead>
        <tbody>
          @forelse ($expense_list as $out_tx)
          <tr>
            <td style="text-align:center;">{{ $loop.iteration }}</td>
            <td>{{ $out_tx.date_formatted }}</td>
            <td style="text-transform:capitalize; font-weight:600;">{{ $out_tx.category_label }}</td>
            <td>{{ $out_tx.donor_name ?? '-' }}</td>
            <td>{{ $out_tx.description }}</td>
            <td class="num" style="font-weight:600;">{{ $out_tx.amount_formatted }}</td>
          </tr>
          @empty
          <tr>
            <td colspan="6" class="pg-empty">Tidak ada catatan pengeluaran.</td>
          </tr>
          @endforelse
          <tr style="background:var(--pg-track); font-weight:700;">
            <td colspan="5" style="text-align:right;">TOTAL PENGELUARAN (B)</td>
            <td class="num">{{ $total_expense_formatted }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>

  <!-- Bagian C: Rekonsiliasi Saldo Akhir -->
  <div style="margin-bottom:24px;">
    <h4 style="font-size:15px; font-weight:700; margin:0 0 8px; border-bottom:1px solid var(--pg-border); padding-bottom:4px;">5. BAGIAN C: REKONSILIASI SALDO AKHIR</h4>
    <div style="background:var(--pg-track); padding:16px; border-radius:var(--pg-r-card); font-size:14px;">
      <div style="display:flex; justify-content:space-between; margin-bottom:6px;">
        <span>Total Seluruh Pemasukan (A)</span>
        <strong>{{ $total_income_formatted }}</strong>
      </div>
      <div style="display:flex; justify-content:space-between; margin-bottom:8px;">
        <span>Total Seluruh Pengeluaran (B)</span>
        <strong>{{ $total_expense_formatted }}</strong>
      </div>
      <div style="display:flex; justify-content:space-between; border-top:1px dashed var(--pg-border); padding-top:8px; font-size:15px;">
        <strong>Sisa Saldo Kas Bersih (A - B)</strong>
        <strong style="color:var(--pg-primary);">{{ $final_balance_formatted }}</strong>
      </div>
      <p class="pg-muted" style="margin:8px 0 0; font-size:12px;">
        *Sisa saldo kas di atas disimpan di kas paguyuban dan dibukukan sebagai saldo awal tahun berikutnya.
      </p>
    </div>
  </div>

  <!-- Bagian D: Donasi Barang / Natura -->
  <div style="margin-bottom:32px;">
    <h4 style="font-size:15px; font-weight:700; margin:0 0 8px; border-bottom:1px solid var(--pg-border); padding-bottom:4px;">6. BAGIAN D: BANTUAN & DONASI BARANG (NATURA)</h4>
    <div class="pg-table-wrap">
      <table class="pg-table" style="width:100%; border:1px solid var(--pg-border); font-size:13px;">
        <thead>
          <tr style="background:var(--pg-track);">
            <th style="width:35px;">No</th>
            <th>Nama Donatur / Warga</th>
            <th>Nama Barang / Bantuan</th>
            <th>Jumlah / Kuantitas</th>
          </tr>
        </thead>
        <tbody>
          @forelse ($inkind_list as $inkind)
          <tr>
            <td style="text-align:center;">{{ $loop.iteration }}</td>
            <td style="font-weight:600;">{{ $inkind.donor_name }}</td>
            <td>{{ $inkind.item }}</td>
            <td style="font-weight:600; color:var(--pg-primary);">{{ $inkind.quantity_text }}</td>
          </tr>
          @empty
          <tr>
            <td colspan="4" class="pg-empty">Tidak ada donasi barang tercatat pada periode ini.</td>
          </tr>
          @endforelse
        </tbody>
      </table>
    </div>
  </div>

  <!-- Lembar Pengesahan Tanda Tangan (4 Pihak) -->
  <div style="margin-top:40px; page-break-inside:avoid;">
    <div style="text-align:right; margin-bottom:20px; font-size:13px;">
      Pekalongan, {{ $print_date }}
    </div>
    <div class="pg-grid-4" style="text-align:center; font-size:13px; gap:16px;">
      <div>
        <p style="margin:0 0 50px;">Ketua Paguyuban,</p>
        <p style="margin:0; font-weight:700; text-decoration:underline;">{{ $chairman_name ?? 'Bambang Sudarsono' }}</p>
        <span class="pg-muted" style="font-size:11px;">Ketua</span>
      </div>
      <div>
        <p style="margin:0 0 50px;">Sekretaris,</p>
        <p style="margin:0; font-weight:700; text-decoration:underline;">{{ $secretary_name ?? 'Ahmad Fauzi' }}</p>
        <span class="pg-muted" style="font-size:11px;">Sekretaris</span>
      </div>
      <div>
        <p style="margin:0 0 50px;">Bendahara,</p>
        <p style="margin:0; font-weight:700; text-decoration:underline;">{{ $treasurer_name ?? 'Siti Rahayu' }}</p>
        <span class="pg-muted" style="font-size:11px;">Bendahara</span>
      </div>
      <div>
        <p style="margin:0 0 50px;">Pembina / Penasihat,</p>
        <p style="margin:0; font-weight:700; text-decoration:underline;">H. Agus Santoso</p>
        <span class="pg-muted" style="font-size:11px;">Pembina</span>
      </div>
    </div>
  </div>
</div>
@endsection

