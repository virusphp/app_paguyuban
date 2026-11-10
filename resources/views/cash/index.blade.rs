@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>Buku Kas & Keuangan Paguyuban</h1>
    <p class="pg-muted">Pencatatan kas masuk, belanja pengeluaran, donasi barang, dan laporan keuangan tahun {{ $selected_year }}.</p>
  </div>
  <div style="display:flex; gap:var(--pg-s2); flex-wrap:wrap;">
    <a href="/laporan/lpj/{{ $selected_year }}" class="pg-btn pg-btn--outline">
      📄 Pratinjau LPJ Resmi
    </a>
    @if ($can_manage)
    <a href="/kas/donasi-barang" class="pg-btn pg-btn--outline">🎁 Donasi Barang</a>
    <a href="/kas/keluar" class="pg-btn pg-btn--outline" style="color:var(--pg-bad-fg); border-color:var(--pg-bad-fg);">- Kas Keluar</a>
    <a href="/kas/masuk" class="pg-btn">+ Kas Masuk</a>
    @endif
  </div>
</div>

<!-- 4 Kartu Statistik Keuangan -->
<div class="pg-grid-4">
  <div class="pg-card">
    <div class="pg-card__title">Saldo Kas Berjalan</div>
    <div class="pg-stat__value" style="color:var(--pg-primary);">{{ $current_balance_formatted }}</div>
    <div class="pg-muted">Posisi kas riil saat ini</div>
  </div>
  <div class="pg-card">
    <div class="pg-card__title">Total Pemasukan ({{ $selected_year }})</div>
    <div class="pg-stat__value" style="color:var(--pg-ok-fg);">{{ $total_income_formatted }}</div>
    <div class="pg-muted">{{ $income_count }} transaksi penerimaan</div>
  </div>
  <div class="pg-card">
    <div class="pg-card__title">Total Pengeluaran ({{ $selected_year }})</div>
    <div class="pg-stat__value" style="color:var(--pg-bad-fg);">{{ $total_expense_formatted }}</div>
    <div class="pg-muted">{{ $expense_count }} transaksi pengeluaran</div>
  </div>
  <div class="pg-card">
    <div class="pg-card__title">Donasi Barang (Natura)</div>
    <div class="pg-stat__value">{{ $in_kind_count }}</div>
    <div class="pg-muted">Bantuan barang dari warga</div>
  </div>
</div>

<!-- Filter Buku Kas -->
<div class="pg-card" style="padding:16px;">
  <form method="GET" action="/kas" style="display:flex; gap:var(--pg-s3); align-items:center; flex-wrap:wrap;">
    <div style="min-width:140px;">
      <label for="year_select" style="display:block; font-size:12px; font-weight:600; margin-bottom:4px;">Tahun Kalender:</label>
      <select id="year_select" name="year" class="pg-select" style="width:100%; height:38px;" onchange="this.form.submit()">
        @foreach ($available_years as $y)
        <option value="{{ $y }}" @selected($y == $selected_year)>Tahun {{ $y }}</option>
        @endforeach
      </select>
    </div>
    <div style="min-width:140px;">
      <label for="kind_select" style="display:block; font-size:12px; font-weight:600; margin-bottom:4px;">Jenis Transaksi:</label>
      <select id="kind_select" name="kind" class="pg-select" style="width:100%; height:38px;" onchange="this.form.submit()">
        <option value="" @selected(!$filter_kind)>Semua Jenis</option>
        <option value="masuk" @selected($filter_kind == 'masuk')>Pemasukan (Masuk)</option>
        <option value="keluar" @selected($filter_kind == 'keluar')>Pengeluaran (Keluar)</option>
      </select>
    </div>
    <div style="min-width:160px;">
      <label for="cat_select" style="display:block; font-size:12px; font-weight:600; margin-bottom:4px;">Kategori:</label>
      <select id="cat_select" name="category" class="pg-select" style="width:100%; height:38px;" onchange="this.form.submit()">
        <option value="" @selected(!$filter_category)>Semua Kategori</option>
        <option value="iuran" @selected($filter_category == 'iuran')>Iuran Warga</option>
        <option value="iuran_panitia" @selected($filter_category == 'iuran_panitia')>Iuran Panitia</option>
        <option value="donasi" @selected($filter_category == 'donasi')>Donasi Warga</option>
        <option value="lelang" @selected($filter_category == 'lelang')>Hasil Lelang</option>
        <option value="konsumsi" @selected($filter_category == 'konsumsi')>Konsumsi</option>
        <option value="hadiah" @selected($filter_category == 'hadiah')>Hadiah & Lomba</option>
        <option value="perlengkapan" @selected($filter_category == 'perlengkapan')>Perlengkapan</option>
        <option value="operasional" @selected($filter_category == 'operasional')>Operasional</option>
      </select>
    </div>
    <div style="align-self:flex-end;">
      <button type="submit" class="pg-btn pg-btn--sm" style="height:38px; padding:0 var(--pg-s4);">Terapkan</button>
      @if ($filter_kind || $filter_category)
      <a href="/kas?year={{ $selected_year }}" class="pg-btn pg-btn--outline pg-btn--sm" style="height:38px; margin-left:6px;">Reset</a>
      @endif
    </div>
  </form>
</div>

<!-- Tabel Mutasi Buku Kas Umum -->
<div class="pg-card" style="padding:0; overflow:hidden;">
  <div style="padding:16px 20px; border-bottom:1px solid var(--pg-border); display:flex; justify-content:space-between; align-items:center; flex-wrap:wrap; gap:8px;">
    <h2 style="font-size:16px;">Mutasi Transaksi Kas (Tahun {{ $selected_year }})</h2>
  </div>

  <div class="pg-table-wrap">
    <table class="pg-table" aria-label="Tabel Mutasi Buku Kas Umum">
      <thead>
        <tr>
          <th style="min-width:100px; padding-left:16px;">Tanggal</th>
          <th style="width:100px;">Jenis</th>
          <th style="min-width:130px;">Kategori</th>
          <th style="min-width:220px;">Keterangan</th>
          <th style="min-width:160px;">Donatur / Pihak</th>
          <th class="num" style="min-width:120px;">Nominal</th>
          <th class="num" style="min-width:130px; padding-right:16px;">Saldo Berjalan</th>
        </tr>
      </thead>
      <tbody>
        @forelse ($transactions as $tx)
        <tr>
          <td style="padding-left:16px; font-weight:600;">{{ $tx.date_formatted }}</td>
          <td>
            @if ($tx.kind == 'masuk')
            <span class="pg-chip pg-chip--ok">Masuk</span>
            @else
            <span class="pg-chip pg-chip--bad">Keluar</span>
            @endif
          </td>
          <td>
            <span style="text-transform:capitalize; font-weight:600;">{{ $tx.category_label }}</span>
          </td>
          <td>
            <span style="color:var(--pg-text);">{{ $tx.description }}</span>
            @if ($tx.source_payment_id)
            <span class="pg-muted" style="font-size:11px; display:block;">Otomatis dari Modul Iuran</span>
            @endif
          </td>
          <td>
            @if ($tx.donor_name)
              <span style="font-weight:600;">{{ $tx.donor_name }}</span>
            @else
              <span class="pg-muted">-</span>
            @endif
          </td>
          <td class="num" style="font-weight:700; color:@if ($tx.kind == 'masuk') var(--pg-ok-fg) @else var(--pg-bad-fg) @endif;">
            @if ($tx.kind == 'masuk')+@else-@endif{{ $tx.amount_formatted }}
          </td>
          <td class="num" style="font-weight:700; color:var(--pg-primary); padding-right:16px;">
            {{ $tx.running_balance_formatted }}
          </td>
        </tr>
        @empty
        <tr>
          <td colspan="7" class="pg-empty">
            <p>Belum ada catatan mutasi transaksi kas di tahun {{ $selected_year }}.</p>
            @if ($can_manage)
            <a href="/kas/masuk" class="pg-btn pg-btn--sm">+ Catat Kas Masuk Pertama</a>
            @endif
          </td>
        </tr>
        @endforelse
      </tbody>
    </table>
  </div>
</div>

<!-- Tabel Donasi Barang (Natura) -->
<div class="pg-card" style="padding:0; overflow:hidden;">
  <div style="padding:16px 20px; border-bottom:1px solid var(--pg-border); display:flex; justify-content:space-between; align-items:center;">
    <div>
      <h2 style="font-size:16px;">Bantuan & Donasi Barang (Natura)</h2>
      <p class="pg-muted" style="font-size:12px; margin:0;">Bantuan logistik/barang warga (tidak dinilai uang, dimuat pada Bagian D LPJ).</p>
    </div>
    @if ($can_manage)
    <a href="/kas/donasi-barang" class="pg-btn pg-btn--outline pg-btn--sm">+ Tambah Donasi Barang</a>
    @endif
  </div>

  <div class="pg-table-wrap">
    <table class="pg-table" aria-label="Tabel Donasi Barang">
      <thead>
        <tr>
          <th style="min-width:100px; padding-left:16px;">Tanggal</th>
          <th style="min-width:180px;">Nama Donatur</th>
          <th style="min-width:200px;">Nama Barang / Item</th>
          <th style="min-width:140px; padding-right:16px;">Jumlah / Kuantitas</th>
        </tr>
      </thead>
      <tbody>
        @forelse ($in_kind_items as $item)
        <tr>
          <td style="padding-left:16px;" class="pg-muted">{{ $item.date_formatted }}</td>
          <td style="font-weight:600;">{{ $item.donor_name }}</td>
          <td>{{ $item.item }}</td>
          <td style="font-weight:600; color:var(--pg-primary); padding-right:16px;">{{ $item.quantity_text }}</td>
        </tr>
        @empty
        <tr>
          <td colspan="4" class="pg-empty">
            <p>Belum ada donasi barang tercatat.</p>
          </td>
        </tr>
        @endforelse
      </tbody>
    </table>
  </div>
</div>
@endsection

