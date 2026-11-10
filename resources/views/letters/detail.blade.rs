@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <div style="display:flex; align-items:center; gap:8px; margin-bottom:4px;">
      <h1>Surat: {{ $letter.number }}</h1>
      @if ($letter.status == 'terkirim')
        <span class="pg-chip pg-chip--ok">✓ Terkirim</span>
      @else
        <span class="pg-chip" style="opacity:0.75;">- Draf</span>
      @endif
    </div>
    <p class="pg-muted">Pratinjau lembar surat resmi paguyuban dan unduhan berkas PDF cetak.</p>
  </div>
  <div style="display:flex; gap:8px; flex-wrap:wrap; align-items:center;">
    <a href="/surat" class="pg-btn pg-btn--outline">← Kembali ke Arsip</a>
    <a href="/surat/{{ $letter.id }}/pdf" class="pg-btn" target="_blank">🖨️ Unduh PDF Resmi</a>
    @if ($can_manage)
    <a href="/surat/{{ $letter.id }}/edit" class="pg-btn pg-btn--outline">Ubah</a>
    <form method="POST" action="/surat/{{ $letter.id }}/kirim" style="margin:0;">
      @csrf
      <button type="submit" class="pg-btn pg-btn--outline">
        @if ($letter.status == 'terkirim')Kembalikan ke Draf@else Tandai Terkirim@endif
      </button>
    </form>
    <form method="POST" action="/surat/{{ $letter.id }}/hapus" style="margin:0;" onsubmit="return confirm('Apakah Anda yakin ingin menghapus arsip surat ini?');">
      @csrf
      <button type="submit" class="pg-btn pg-btn--outline" style="color:var(--pg-bad-fg); border-color:var(--pg-bad-fg);">Hapus</button>
    </form>
    @endif
  </div>
</div>

<!-- Lembar Format Surat Resmi Standar Indonesia -->
<div class="pg-letter-sheet">
  
  <!-- Kop Surat Resmi -->
  <div style="text-align:center; border-bottom:3px double #2d3748; padding-bottom:14px; margin-bottom:28px;">
    <div style="font-size:14px; font-weight:700; text-transform:uppercase; letter-spacing:1px; color:#4a5568;">PENGURUS LINGKUNGAN WARGA</div>
    <div style="font-size:22px; font-weight:800; text-transform:uppercase; letter-spacing:1px; margin:2px 0; color:#1a202c;">{{ $org_name }}</div>
    <div style="font-size:12px; color:#718096;">Sekretariat: Komplek Perumahan Pedalangan, Banyumanik, Kota Semarang • Jawa Tengah</div>
  </div>

  <!-- Header Surat: Nomor, Lampiran, Hal, dan Tanggal -->
  <div class="pg-letter-header">
    <div style="flex:1;">
      <div style="display:flex; gap:8px;">
        <span style="width:90px; color:#4a5568;">Nomor</span>
        <span>: <strong>{{ $letter.number }}</strong></span>
      </div>
      <div style="display:flex; gap:8px;">
        <span style="width:90px; color:#4a5568;">Lampiran</span>
        <span>: - (Satu Berkas)</span>
      </div>
      <div style="display:flex; gap:8px;">
        <span style="width:90px; color:#4a5568;">Perihal</span>
        <span>: <strong>{{ $letter.subject }}</strong></span>
      </div>
    </div>
    <div style="text-align:right;">
      <span>Semarang, {{ $letter.date_formatted }}</span>
    </div>
  </div>

  <!-- Tujuan Penerima Surat -->
  <div style="margin-bottom:24px; font-size:14px; line-height:1.6;">
    <div>Kepada Yth.</div>
    <div style="font-weight:700; font-size:15px;">{{ $letter.recipient }}</div>
    <div>di Tempat</div>
  </div>

  <!-- Isi / Badan Surat -->
  <div style="font-size:14px; line-height:1.8; text-align:justify; margin-bottom:40px; white-space:pre-line; color:#2d3748;">
    {{ $letter.body }}
  </div>

  <!-- Blok Tanda Tangan Pengurus (2 Kolom) -->
  <div class="pg-letter-signatures">
    <div style="width:240px;">
      <div>Mengetahui,</div>
      <div style="font-weight:600;">Sekretaris Paguyuban</div>
      <div style="height:70px;"></div>
      <div style="font-weight:700; text-decoration:underline;">( Ahmad Fauzi )</div>
    </div>
    <div style="width:240px;">
      <div>Hormat kami,</div>
      <div style="font-weight:600;">Ketua {{ $org_name }}</div>
      <div style="height:70px;"></div>
      <div style="font-weight:700; text-decoration:underline;">( Bambang Sudarsono )</div>
    </div>
  </div>

</div>
@endsection

