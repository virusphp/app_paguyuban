@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head" style="display:flex; justify-content:space-between; align-items:center; margin-bottom:var(--pg-s5); flex-wrap:wrap; gap:var(--pg-s3);">
  <div>
    <h1 class="pg-page-head__title" style="font-size:22px; font-weight:700; margin:0;">Dashboard Ringkasan</h1>
    <p class="pg-page-head__sub" style="color:var(--pg-muted); font-size:13px; margin:4px 0 0 0;">Aktivitas operasional dan posisi kas {{ $org_name }}</p>
  </div>
  @if ($can_manage)
  <div style="display:flex; gap:var(--pg-s2); flex-wrap:wrap;">
    <a href="/kas/keluar" class="pg-btn pg-btn--outline" style="color:var(--pg-bad-fg); border-color:var(--pg-bad-fg);">- Kas Keluar</a>
    <a href="/kas/masuk" class="pg-btn">+ Kas Masuk</a>
  </div>
  @endif
</div>

<!-- 4 Kartu Statistik Finansial & Warga -->
<div class="pg-grid-4">
  <div class="pg-card pg-card--interactive">
    <div style="display:flex; justify-content:space-between; align-items:flex-start;">
      <div>
        <div class="pg-card__title">Total Warga Terdata</div>
        <div class="pg-stat__value">{{ $total_members }} <span style="font-size:14px; font-weight:normal; color:var(--pg-muted);">Jiwa</span></div>
      </div>
      <span style="font-size:22px; opacity:0.8;">👥</span>
    </div>
    <div class="pg-muted" style="margin-top:6px;">{{ $total_households }} Kepala Keluarga (KK)</div>
  </div>

  <div class="pg-card pg-card--interactive">
    <div style="display:flex; justify-content:space-between; align-items:flex-start;">
      <div>
        <div class="pg-card__title">Saldo Kas Riil</div>
        <div class="pg-stat__value" style="color:var(--pg-primary);">{{ $current_balance | rupiah }}</div>
      </div>
      <span style="font-size:22px; opacity:0.8;">💰</span>
    </div>
    <div class="pg-muted" style="margin-top:6px;">Posisi kas bersih saat ini</div>
  </div>

  <div class="pg-card pg-card--interactive">
    <div style="display:flex; justify-content:space-between; align-items:flex-start;">
      <div>
        <div class="pg-card__title">Iuran Terkumpul</div>
        <div class="pg-stat__value" style="color:var(--pg-ok-fg);">{{ $fee_collected | rupiah }}</div>
      </div>
      <span style="font-size:22px; opacity:0.8;">💳</span>
    </div>
    <div class="pg-muted" style="margin-top:6px;">{{ $collection_percent }}% tingkat kepatuhan warga</div>
  </div>

  <div class="pg-card pg-card--interactive">
    <div style="display:flex; justify-content:space-between; align-items:flex-start;">
      <div>
        <div class="pg-card__title">Total Tunggakan</div>
        <div class="pg-stat__value" style="color:var(--pg-bad-fg);">{{ $total_arrears | rupiah }}</div>
      </div>
      <span style="font-size:22px; opacity:0.8;">⚠️</span>
    </div>
    <div class="pg-muted" style="margin-top:6px;">{{ $arrears_count }} tagihan belum lunas</div>
  </div>
</div>

<!-- Layout 2 Kolom Sesuai Pola Desain (Responsif: 1 Kolom di Layar Sempit) -->
<div class="pg-grid-2" style="align-items:start;">
  <!-- Kolom Kiri: Aktivitas Kas & Transaksi Terbaru -->
  <div style="display:flex; flex-direction:column; gap:var(--pg-s5);">
    <div class="pg-card" style="padding:0; overflow:hidden;">
      <div style="padding:16px 20px; border-bottom:1px solid var(--pg-border); display:flex; justify-content:space-between; align-items:center;">
        <div>
          <h2 style="font-size:16px; margin:0; font-weight:700;">Aktivitas Kas Terbaru</h2>
          <span class="pg-muted" style="font-size:12px;">5 mutasi pencatatan terakhir</span>
        </div>
        <a href="/kas" class="pg-btn pg-btn--outline pg-btn--sm">Buku Kas Selengkapnya &rarr;</a>
      </div>

      <div class="pg-table-wrap">
        <table class="pg-table" style="width:100%; margin:0;">
          <thead>
            <tr>
              <th style="padding-left:20px; min-width:110px;">Tanggal</th>
              <th>Keterangan</th>
              <th style="width:110px;">Kategori</th>
              <th style="text-align:right; padding-right:20px; min-width:130px;">Jumlah</th>
            </tr>
          </thead>
          <tbody>
            @forelse ($recent_transactions as $tx)
            <tr>
              <td style="padding-left:20px; font-size:12px; color:var(--pg-muted);">{{ $tx.date }}</td>
              <td>
                <div style="font-weight:600;">{{ $tx.description }}</div>
              </td>
              <td>
                <span class="pg-chip" style="font-size:11px;">{{ $tx.category }}</span>
              </td>
              <td style="text-align:right; padding-right:20px; font-weight:700; @if ($tx.tone == 'ok')color:var(--pg-ok-fg);@else color:var(--pg-bad-fg);@endif">
                @if ($tx.tone == 'ok')+@else-@endif Rp{{ $tx.amount }}
              </td>
            </tr>
            @empty
            <tr>
              <td colspan="4" style="text-align:center; padding:32px 20px; color:var(--pg-muted);">
                Belum ada transaksi kas yang dicatat di organisasi ini.
              </td>
            </tr>
            @endforelse
          </tbody>
        </table>
      </div>
    </div>

    <!-- Progress Penarikan Iuran Warga -->
    <div class="pg-card" style="padding:20px;">
      <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:10px;">
        <span style="font-weight:600; font-size:14px;">Realisasi Pembayaran Iuran Warga</span>
        <span style="font-weight:700; color:var(--pg-ok-fg);">{{ $collection_percent }}%</span>
      </div>
      <div class="pg-progress" style="height:10px; border-radius:5px; background:var(--pg-border-soft); overflow:hidden; margin-bottom:8px;">
        <div style="height:100%; width: {{ $collection_percent }}%; background:var(--pg-primary); border-radius:5px;"></div>
      </div>
      <div style="display:flex; justify-content:space-between; font-size:12px; color:var(--pg-muted);">
        <span>Terkumpul: {{ $fee_collected | rupiah }}</span>
        <span>Sisa Tunggakan: {{ $total_arrears | rupiah }}</span>
      </div>
    </div>
  </div>

  <!-- Kolom Kanan: Pintasan Aksi Cepat & Agenda Kegiatan -->
  <div style="display:flex; flex-direction:column; gap:var(--pg-s5);">
    <!-- Kartu Aksi Cepat Pengurus -->
    @if ($can_manage)
    <div class="pg-card" style="padding:20px;">
      <h2 style="font-size:15px; font-weight:700; margin:0 0 14px;">Pintasan Aksi Pengurus</h2>
      <div class="pg-grid-2" style="gap:10px;">
        <a href="/anggota/baru" class="pg-btn pg-btn--outline pg-btn--sm" style="display:flex; align-items:center; justify-content:center; gap:6px; padding:10px 8px; text-align:center;">
          👤 Tambah Warga
        </a>
        <a href="/iuran/generate" class="pg-btn pg-btn--outline pg-btn--sm" style="display:flex; align-items:center; justify-content:center; gap:6px; padding:10px 8px; text-align:center;">
          💳 Terbitkan Tagihan
        </a>
        <a href="/kas/masuk" class="pg-btn pg-btn--outline pg-btn--sm" style="display:flex; align-items:center; justify-content:center; gap:6px; padding:10px 8px; text-align:center;">
          📥 Catat Kas Masuk
        </a>
        <a href="/kas/keluar" class="pg-btn pg-btn--outline pg-btn--sm" style="display:flex; align-items:center; justify-content:center; gap:6px; padding:10px 8px; text-align:center;">
          📤 Catat Kas Keluar
        </a>
        <a href="/kegiatan/baru" class="pg-btn pg-btn--outline pg-btn--sm" style="display:flex; align-items:center; justify-content:center; gap:6px; padding:10px 8px; text-align:center;">
          📅 Buat Agenda
        </a>
        <a href="/surat/baru" class="pg-btn pg-btn--outline pg-btn--sm" style="display:flex; align-items:center; justify-content:center; gap:6px; padding:10px 8px; text-align:center;">
          ✉️ Buat Surat / Undangan
        </a>
      </div>
    </div>
    @endif

    <!-- Agenda Kegiatan Mendatang -->
    <div class="pg-card" style="padding:20px;">
      <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:12px;">
        <h2 style="font-size:15px; font-weight:700; margin:0;">Agenda Kegiatan Mendatang</h2>
        <a href="/kegiatan" style="font-size:12px; color:var(--pg-primary); text-decoration:none;">Lihat Semua</a>
      </div>
      @forelse ($upcoming_events as $ev)
      <div style="padding:10px 0; border-bottom:1px solid var(--pg-border-soft);">
        <div style="font-weight:600; font-size:13px; margin-bottom:2px;">
          <a href="/kegiatan/{{ $ev.id }}" style="color:var(--pg-text); text-decoration:none;">{{ $ev.name }}</a>
        </div>
        <div style="font-size:12px; color:var(--pg-muted);">🕒 {{ $ev.date_time }}</div>
        <div style="font-size:12px; color:var(--pg-muted);">📍 {{ $ev.location }}</div>
      </div>
      @empty
      <div style="text-align:center; padding:16px 0; color:var(--pg-muted); font-size:12px;">
        Belum ada jadwal kegiatan mendatang.
      </div>
      @endforelse
    </div>
  </div>
</div>
@endsection
