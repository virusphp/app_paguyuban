@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>Papan Informasi & Pengumuman Warga</h1>
    <p class="pg-muted">Pemberitahuan resmi pengurus lingkungan, jadwal layanan, dan informasi penting paguyuban/RT.</p>
  </div>
  @if ($can_manage)
  <div>
    <a href="/info/baru" class="pg-btn">+ Terbitkan Pengumuman Baru</a>
  </div>
  @endif
</div>

<!-- Daftar Pengumuman -->
<div class="pg-stack">
  @forelse ($announcements as $info)
  <div class="pg-card" style="padding:24px; @if ($info.pinned)border:2px solid var(--pg-primary); background:#fafdfe;@endif">
    <div style="display:flex; justify-content:space-between; align-items:flex-start; gap:16px; margin-bottom:12px; flex-wrap:wrap;">
      <div>
        @if ($info.pinned)
        <span class="pg-chip pg-chip--ok" style="margin-bottom:8px; display:inline-flex; align-items:center; gap:4px;">
          📌 PENGUMUMAN TERSEMAT
        </span>
        @endif
        <h2 style="font-size:18px; font-weight:700; margin:0 0 6px; color:var(--pg-text);">{{ $info.title }}</h2>
        <div class="pg-muted" style="font-size:12px;">
          Diterbitkan pada {{ $info.published_at_formatted }} WIB • Oleh: {{ $info.author_name }}
        </div>
      </div>

      @if ($can_manage)
      <div style="display:flex; gap:6px; align-items:center; flex-wrap:wrap;">
        <form method="POST" action="/info/{{ $info.id }}/pin" style="margin:0;">
          @csrf
          <button type="submit" class="pg-btn pg-btn--outline pg-btn--sm" title="@if ($info.pinned)Lepaskan Sematan@else Sematkan ke Atas@endif">
            @if ($info.pinned)📌 Lepas Pin@else📌 Sematkan@endif
          </button>
        </form>
        <a href="/info/{{ $info.id }}/edit" class="pg-btn pg-btn--outline pg-btn--sm">Ubah</a>
        <form method="POST" action="/info/{{ $info.id }}/hapus" style="margin:0;" onsubmit="return confirm('Apakah Anda yakin ingin menghapus pengumuman ini?');">
          @csrf
          <button type="submit" class="pg-btn pg-btn--outline pg-btn--sm" style="color:var(--pg-bad-fg); border-color:var(--pg-bad-fg);">Hapus</button>
        </form>
      </div>
      @endif
    </div>

    <!-- Isi Pengumuman -->
    <div style="font-size:14px; line-height:1.6; color:var(--pg-text); white-space:pre-line;">
      {{ $info.body }}
    </div>
  </div>
  @empty
  <div class="pg-card pg-empty">
    <p>Belum ada pengumuman yang diterbitkan.</p>
    @if ($can_manage)
    <a href="/info/baru" class="pg-btn pg-btn--sm">+ Tulis Pengumuman Pertama</a>
    @endif
  </div>
  @endforelse
</div>
@endsection

