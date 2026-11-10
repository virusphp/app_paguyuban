@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>Surat & Undangan Warga</h1>
    <p class="pg-muted">Kelola persuratan resmi, surat pengantar domisili, edaran, dan surat undangan pertemuan lingkungan.</p>
  </div>
  @if ($can_manage)
  <div style="display:flex; gap:var(--pg-s2);">
    <a href="/surat/template" class="pg-btn pg-btn--outline">Template Surat</a>
    <a href="/surat/baru" class="pg-btn">+ Buat Surat Baru</a>
  </div>
  @endif
</div>

<!-- 4 Kartu Statistik Surat -->
<div class="pg-grid-4">
  <div class="pg-card">
    <div class="pg-card__title">Total Arsip Surat</div>
    <div class="pg-stat__value">{{ $total_count }}</div>
    <div class="pg-muted">Seluruh dokumen teregistrasi</div>
  </div>
  <div class="pg-card">
    <div class="pg-card__title">Surat Pengantar & Edaran</div>
    <div class="pg-stat__value" style="color:var(--pg-primary);">{{ $surat_count }}</div>
    <div class="pg-muted">Korespondensi dinas / warga</div>
  </div>
  <div class="pg-card">
    <div class="pg-card__title">Undangan Kegiatan</div>
    <div class="pg-stat__value" style="color:var(--pg-warn-fg);">{{ $undangan_count }}</div>
    <div class="pg-muted">Pertemuan RT & musyawarah</div>
  </div>
  <div class="pg-card">
    <div class="pg-card__title">Status Terkirim</div>
    <div class="pg-stat__value" style="color:var(--pg-ok-fg);">{{ $sent_count }}</div>
    <div class="pg-muted">Telah didistribusikan ke warga</div>
  </div>
</div>

<!-- Filter & Tabel Arsip Surat -->
<div class="pg-card" style="padding:0; overflow:hidden;">
  <div style="padding:16px 20px; border-bottom:1px solid var(--pg-border); display:flex; justify-content:space-between; align-items:center; flex-wrap:wrap; gap:12px;">
    <div>
      <h2 style="font-size:16px; margin:0 0 4px;">Daftar Registrasi Surat & Undangan</h2>
      <span class="pg-muted">Organisasi: {{ $org_name }}</span>
    </div>
    
    <!-- Filter Pencarian & Kategori -->
    <form method="GET" action="/surat" style="display:flex; gap:8px; align-items:center; flex-wrap:wrap;">
      <select name="kind" class="pg-select" style="height:36px; font-size:13px;" onchange="this.form.submit()">
        <option value="" @selected($filter_kind == '')>Semua Jenis</option>
        <option value="surat" @selected($filter_kind == 'surat')>Surat Resmi</option>
        <option value="undangan" @selected($filter_kind == 'undangan')>Undangan</option>
      </select>
      <input type="text" name="q" class="pg-input" style="height:36px; font-size:13px; width:200px;" placeholder="Cari perihal / nomor..." value="{{ $search_query }}">
      <button type="submit" class="pg-btn pg-btn--sm">Cari</button>
      @if ($filter_kind != '' || $search_query != '')
      <a href="/surat" class="pg-btn pg-btn--outline pg-btn--sm">Reset</a>
      @endif
    </form>
  </div>

  <div class="pg-table-wrap">
    <table class="pg-table" aria-label="Tabel Registrasi Surat">
      <thead>
        <tr>
          <th style="min-width:170px; padding-left:16px;">Nomor Surat</th>
          <th style="width:110px;">Jenis</th>
          <th style="min-width:220px;">Perihal & Penerima</th>
          <th style="min-width:130px;">Tanggal</th>
          <th style="width:110px;">Status</th>
          <th style="width:150px; text-align:right; padding-right:16px;">Aksi</th>
        </tr>
      </thead>
      <tbody>
        @forelse ($letters as $l)
        <tr>
          <td style="padding-left:16px;">
            <a href="/surat/{{ $l.id }}" style="font-weight:700; color:var(--pg-primary); text-decoration:none;">
              {{ $l.number }}
            </a>
          </td>
          <td>
            @if ($l.kind == 'undangan')
              <span class="pg-chip pg-chip--warn">Undangan</span>
            @else
              <span class="pg-chip">Surat Resmi</span>
            @endif
          </td>
          <td>
            <div style="font-weight:600; color:var(--pg-text);">{{ $l.subject }}</div>
            <div class="pg-muted" style="font-size:12px;">Kepada: {{ $l.recipient }}</div>
          </td>
          <td>
            <div>{{ $l.date_formatted }}</div>
          </td>
          <td>
            @if ($l.status == 'terkirim')
              <span class="pg-chip pg-chip--ok">✓ Terkirim</span>
            @else
              <span class="pg-chip" style="opacity:0.75;">- Draf</span>
            @endif
          </td>
          <td style="text-align:right; padding-right:16px;">
            <div style="display:inline-flex; gap:6px;">
              <a href="/surat/{{ $l.id }}" class="pg-btn pg-btn--outline pg-btn--sm" title="Lihat Pratinjau Surat">Lihat</a>
              <a href="/surat/{{ $l.id }}/pdf" class="pg-btn pg-btn--outline pg-btn--sm" title="Unduh Berkas PDF Resmi" target="_blank">PDF</a>
              @if ($can_manage)
              <a href="/surat/{{ $l.id }}/edit" class="pg-btn pg-btn--outline pg-btn--sm">Ubah</a>
              @endif
            </div>
          </td>
        </tr>
        @empty
        <tr>
          <td colspan="6" class="pg-empty">
            <p>Belum ada arsip surat atau undangan yang tercatat.</p>
            @if ($can_manage)
            <a href="/surat/baru" class="pg-btn pg-btn--sm">+ Buat Surat / Undangan Pertama</a>
            @endif
          </td>
        </tr>
        @endforelse
      </tbody>
    </table>
  </div>
</div>
@endsection

