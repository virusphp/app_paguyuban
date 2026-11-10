@extends('layouts/dashboard.blade.rs')

@section('content')
<header class="pg-page-head">
  <div>
    <h1 class="pg-page-head__title">@if ($is_edit) Ubah Data Warga @else Tambah Warga Baru @endif</h1>
    <p class="pg-page-head__subtitle">Kelola informasi identitas warga dan hunian tempat tinggal di {{ $org_name }}.</p>
  </div>
  <div class="pg-page-head__actions">
    <a href="/anggota" class="pg-btn pg-btn--secondary">
      Kembali ke Daftar
    </a>
  </div>
</header>

<div style="max-width: 640px;">
  <div class="pg-card">
    <form action="@if ($is_edit) /anggota/{{ $member.id }} @else /anggota/baru @endif" method="POST" style="display:flex; flex-direction:column; gap:var(--pg-s5);">
      @csrf

      @if ($error)
      <div class="pg-notice pg-notice--bad">
        {{ $error }}
      </div>
      @endif

      <div>
        <h2 style="font-size:16px; font-weight:700; color:var(--pg-text); margin-bottom:var(--pg-s3); border-bottom:1px solid var(--pg-border-soft); padding-bottom:var(--pg-s2);">
          1. Identitas Warga
        </h2>

        <div style="display:flex; flex-direction:column; gap:var(--pg-s4);">
          <div class="pg-field">
            <label for="full_name" class="pg-label">Nama Lengkap Warga (wajib)</label>
            <input type="text" id="full_name" name="full_name" class="pg-input" required placeholder="Contoh: Bpk. Bambang Sudarsono" value="{{ $member.full_name ?? '' }}">
          </div>

          <div class="pg-field">
            <label for="phone" class="pg-label">Nomor HP / WhatsApp (opsional)</label>
            <input type="tel" id="phone" name="phone" class="pg-input" placeholder="Contoh: 081234567890" value="{{ $member.phone ?? '' }}">
            <span class="pg-help">Nomor ini berguna untuk pemberitahuan tagihan dan undangan.</span>
          </div>

          <div class="pg-field">
            <label for="status" class="pg-label">Status Warga (wajib)</label>
            <select id="status" name="status" class="pg-input" required>
              <option value="aktif" @selected($member.status == 'aktif' || !$member.status)>Aktif (Tinggal Menetap)</option>
              <option value="menunggu" @selected($member.status == 'menunggu')>Menunggu Verifikasi</option>
              <option value="nonaktif" @selected($member.status == 'nonaktif')>Nonaktif (Pindah / Tidak Aktif)</option>
            </select>
          </div>

          <div class="pg-field">
            <label for="joined_on" class="pg-label">Tanggal Bergabung</label>
            <input type="date" id="joined_on" name="joined_on" class="pg-input" value="{{ $member.joined_on ?? $today_date }}">
          </div>
        </div>
      </div>

      <div style="margin-top:var(--pg-s2);">
        <h2 style="font-size:16px; font-weight:700; color:var(--pg-text); margin-bottom:var(--pg-s3); border-bottom:1px solid var(--pg-border-soft); padding-bottom:var(--pg-s2);">
          2. Data Hunian / Rumah (KK)
        </h2>

        <div style="display:flex; flex-direction:column; gap:var(--pg-s4);">
          <div class="pg-field">
            <label for="house_no" class="pg-label">Nomor Rumah / Blok (wajib)</label>
            <input type="text" list="house_list" id="house_no" name="house_no" class="pg-input" required placeholder="Contoh: Blok B-12 atau RT 05 No. 10" value="{{ $member.house_no ?? '' }}">
            <datalist id="house_list">
              @foreach ($existing_houses as $h)
                <option value="{{ $h.house_no }}">{{ $h.house_no }} @if ($h.note) ({{ $h.note }}) @endif</option>
              @endforeach
            </datalist>
            <span class="pg-help">Bisa memilih nomor rumah yang sudah ada atau mengetik nomor rumah baru.</span>
          </div>

          <div class="pg-field">
            <label for="house_note" class="pg-label">Catatan Rumah / Status Keluarga (opsional)</label>
            <input type="text" id="house_note" name="house_note" class="pg-input" placeholder="Contoh: Keluarga Bpk. Bambang, Warga Tetap" value="{{ $member.house_note ?? '' }}">
          </div>
        </div>
      </div>

      <div style="display:flex; gap:var(--pg-s3); margin-top:var(--pg-s4); border-top:1px solid var(--pg-border-soft); padding-top:var(--pg-s4); flex-wrap:wrap;">
        <button type="submit" class="pg-btn pg-btn--primary">
          @if ($is_edit) Perbarui Data Warga @else Simpan Data Warga @endif
        </button>
        <a href="/anggota" class="pg-btn pg-btn--secondary">
          Batal
        </a>
      </div>

    </form>
  </div>
</div>
@endsection

