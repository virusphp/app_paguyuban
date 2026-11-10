@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>Pengaturan Organisasi</h1>
    <p class="pg-muted">Profil identitas, struktur kepengurusan, dan informasi lingkungan {{ $org_name }}.</p>
  </div>
</div>

<div class="pg-grid-2" style="align-items:start;">
  <!-- Kartu Informasi & Ubah Nama Organisasi -->
  <div class="pg-card" style="padding:24px;">
    <h2 style="font-size:16px; font-weight:700; margin:0 0 16px;">Profil Lingkungan</h2>

    <form method="POST" action="/pengaturan">
      @csrf

      <div class="pg-field" style="margin-bottom:16px;">
        <label for="org_name_input" style="font-weight:600; font-size:13px; display:block; margin-bottom:6px;">Nama Paguyuban / RT (wajib)</label>
        <input 
          type="text" 
          id="org_name_input" 
          name="name" 
          value="{{ $org_name }}" 
          class="pg-input" 
          style="width:100%;" 
          @if (!$can_edit)disabled readonly@endif 
          required
        >
        @if (!$can_edit)
        <span class="pg-muted" style="font-size:12px; margin-top:4px; display:block;">Hanya Ketua (Admin) yang dapat mengubah nama organisasi.</span>
        @endif
      </div>

      <div class="pg-grid-2" style="gap:16px; margin-bottom:16px;">
        <div class="pg-field">
          <label style="font-weight:600; font-size:13px; display:block; margin-bottom:6px;">Jenis Organisasi</label>
          <input type="text" value="{{ $org_type }}" class="pg-input" style="width:100%;" disabled readonly>
        </div>

        <div class="pg-field">
          <label style="font-weight:600; font-size:13px; display:block; margin-bottom:6px;">Status Organisasi</label>
          <div style="padding-top:6px;">
            <span class="pg-chip pg-chip--ok">Aktif</span>
          </div>
        </div>
      </div>

      <div class="pg-field" style="margin-bottom:20px;">
        <label style="font-weight:600; font-size:13px; display:block; margin-bottom:6px;">Terdaftar di Sistem Sejak</label>
        <input type="text" value="{{ $created_date }}" class="pg-input" style="width:100%;" disabled readonly>
      </div>

      @if ($can_edit)
      <div style="display:flex; justify-content:flex-end;">
        <button type="submit" class="pg-btn">Simpan Perubahan</button>
      </div>
      @endif
    </form>
  </div>

  <!-- Kartu Susunan Pengurus Organisasi -->
  <div class="pg-card" style="padding:24px;">
    <h2 style="font-size:16px; font-weight:700; margin:0 0 16px;">Susunan Pengurus Lingkungan</h2>
    <p class="pg-muted" style="font-size:12px; margin-bottom:16px;">
      Pengurus memiliki hak akses mengelola data warga, keuangan kas, tagihan iuran, dan surat-menyurat.
    </p>

    <div style="display:flex; flex-direction:column; gap:12px;">
      @forelse ($officers as $officer)
      <div style="display:flex; justify-content:space-between; align-items:center; padding:10px 14px; background:var(--pg-surface); border:1px solid var(--pg-border); border-radius:var(--pg-r-card);">
        <div>
          <div style="font-weight:700; font-size:14px; color:var(--pg-text);">{{ $officer.name }}</div>
          <div class="pg-muted" style="font-size:12px;">📞 {{ $officer.phone }}</div>
        </div>
        <div>
          @if ($officer.role == 'admin')
            <span class="pg-chip pg-chip--ok">Ketua (Admin)</span>
          @elseif ($officer.role == 'bendahara')
            <span class="pg-chip" style="background:#fef3c7; color:#92400e;">Bendahara</span>
          @elseif ($officer.role == 'sekretaris')
            <span class="pg-chip" style="background:#e0e7ff; color:#3730a3;">Sekretaris</span>
          @else
            <span class="pg-chip">{{ $officer.role }}</span>
          @endif
        </div>
      </div>
      @empty
      <div style="text-align:center; padding:20px; color:var(--pg-muted); font-size:13px;">
        Belum ada pengurus lain yang terdaftar di organisasi ini.
      </div>
      @endforelse
    </div>
  </div>
</div>
@endsection

