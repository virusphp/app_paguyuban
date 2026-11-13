@extends('layouts/dashboard.blade.rs')

@section('content')
<div style="max-width: 860px; margin: 0 auto; display: flex; flex-direction: column; gap: var(--pg-s5);">

  <!-- Header Halaman -->
  <header class="pg-page-head">
    <div>
      <div style="display:flex; align-items:center; gap:var(--pg-s2); margin-bottom:var(--pg-s1);">
        <a href="/anggota" class="pg-muted" style="text-decoration:none; font-size:13px; display:inline-flex; align-items:center; gap:4px;">
          &larr; Warga & Anggota
        </a>
        <span class="pg-muted" style="font-size:12px;">/</span>
        <span style="font-size:13px; font-weight:600; color:var(--pg-primary);">@if ($is_edit) Ubah Data @else Pendaftaran Baru @endif</span>
      </div>
      <h1 class="pg-page-head__title">@if ($is_edit) Ubah Data Warga @else Tambah Warga Baru @endif</h1>
      <p class="pg-page-head__subtitle">Kelola informasi identitas warga, domisili keluarga, dan administrasi lingkungan di {{ $org_name }}.</p>
    </div>
    <div class="pg-page-head__actions">
      <a href="/anggota" class="pg-btn pg-btn--outline">
        Batal & Kembali
      </a>
    </div>
  </header>

  @if ($error)
  <div class="pg-alert pg-alert--bad">
    <strong>Perhatian:</strong> {{ $error }}
  </div>
  @endif

  <!-- Form Pendaftaran / Pengubahan Warga -->
  <form action="@if ($is_edit) /anggota/{{ $member.id }} @else /anggota/baru @endif" method="POST" style="display:flex; flex-direction:column; gap:var(--pg-s5);">
    @csrf

    <!-- SECTION 1: IDENTITAS PRIBADI WARGA -->
    <div class="pg-card" style="display:flex; flex-direction:column; gap:var(--pg-s4);">
      <div style="display:flex; align-items:center; justify-content:space-between; border-bottom:1px solid var(--pg-border-soft); padding-bottom:var(--pg-s3);">
        <div>
          <h2 style="font-size:16px; font-weight:700; color:var(--pg-text); margin:0;">
            🪪 1. Identitas Pribadi Warga
          </h2>
          <p class="pg-muted" style="font-size:12px; margin:4px 0 0;">Informasi dasar kependudukan sesuai KTP / identitas resmi.</p>
        </div>
        <span class="pg-chip" style="font-size:11px;">Identitas Resmi</span>
      </div>

      <!-- Nama Lengkap Warga -->
      <div class="pg-field">
        <label for="full_name" class="pg-label">Nama Lengkap Warga (wajib)</label>
        <input type="text" id="full_name" name="full_name" class="pg-input" required placeholder="Contoh: Bpk. Bambang Sudarsono, S.T." value="{{ $member.full_name }}">
        <span class="pg-help">Tuliskan nama lengkap beserta gelar jika ada.</span>
      </div>

      <!-- Grid 2 Kolom: NIK & Jenis Kelamin -->
      <div style="display:grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap:var(--pg-s4);">
        <div class="pg-field">
          <label for="nik" class="pg-label">Nomor Induk Kependudukan / NIK (opsional)</label>
          <input type="text" id="nik" name="nik" class="pg-input" maxlength="20" placeholder="16 Digit Nomor KTP (misal: 3326...)" value="{{ $member.nik }}">
          <span class="pg-help">🛡️ Privasi aman: NIK hanya digunakan untuk keperluan surat pengantar warga.</span>
        </div>

        <div class="pg-field">
          <label for="gender" class="pg-label">Jenis Kelamin</label>
          <select id="gender" name="gender" class="pg-input">
            <option value="Laki-laki" @if ($member.gender == 'Laki-laki' || $member.gender == 'L') selected @endif>👨 Laki-laki</option>
            <option value="Perempuan" @if ($member.gender == 'Perempuan' || $member.gender == 'P') selected @endif>👩 Perempuan</option>
          </select>
        </div>
      </div>

      <!-- Grid 2 Kolom: Tempat Lahir & Tanggal Lahir -->
      <div style="display:grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap:var(--pg-s4);">
        <div class="pg-field">
          <label for="birth_place" class="pg-label">Tempat Lahir</label>
          <input type="text" id="birth_place" name="birth_place" class="pg-input" placeholder="Kota / Kabupaten Kelahiran (misal: Pekalongan)" value="{{ $member.birth_place }}">
        </div>

        <div class="pg-field">
          <label for="birth_date" class="pg-label">Tanggal Lahir</label>
          <input type="date" id="birth_date" name="birth_date" class="pg-input" value="{{ $member.birth_date }}">
        </div>
      </div>

      <!-- Grid 2 Kolom: Agama & Pekerjaan -->
      <div style="display:grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap:var(--pg-s4);">
        <div class="pg-field">
          <label for="religion" class="pg-label">Agama</label>
          <select id="religion" name="religion" class="pg-input">
            <option value="Islam" @if ($member.religion == 'Islam') selected @endif>Islam</option>
            <option value="Kristen Protestan" @if ($member.religion == 'Kristen Protestan' || $member.religion == 'Kristen') selected @endif>Kristen Protestan</option>
            <option value="Katolik" @if ($member.religion == 'Katolik') selected @endif>Katolik</option>
            <option value="Hindu" @if ($member.religion == 'Hindu') selected @endif>Hindu</option>
            <option value="Buddha" @if ($member.religion == 'Buddha') selected @endif>Buddha</option>
            <option value="Konghucu" @if ($member.religion == 'Konghucu') selected @endif>Konghucu</option>
            <option value="Lainnya" @if ($member.religion == 'Lainnya') selected @endif>Lainnya</option>
          </select>
        </div>

        <div class="pg-field">
          <label for="occupation" class="pg-label">Pekerjaan / Profesi</label>
          <input type="text" id="occupation" name="occupation" class="pg-input" placeholder="Misal: Karyawan Swasta, PNS, Wiraswasta, Guru" value="{{ $member.occupation }}">
        </div>
      </div>

      <!-- Grid 2 Kolom: Nomor WhatsApp & Kontak Darurat -->
      <div style="display:grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap:var(--pg-s4);">
        <div class="pg-field">
          <label for="phone" class="pg-label">Nomor WhatsApp / HP</label>
          <input type="tel" id="phone" name="phone" class="pg-input" placeholder="Contoh: 081234567890" value="{{ $member.phone }}">
          <span class="pg-help">Nomor ini dipakai untuk pengiriman rincian iuran dan warta lingkungan.</span>
        </div>

        <div class="pg-field">
          <label for="emergency_phone" class="pg-label">Nomor Kontak Darurat (opsional)</label>
          <input type="tel" id="emergency_phone" name="emergency_phone" class="pg-input" placeholder="No HP kerabat dekat / keluarga" value="{{ $member.emergency_phone }}">
          <span class="pg-help">Dihubungi saat keadaan genting bila nomor utama tidak aktif.</span>
        </div>
      </div>
    </div>

    <!-- SECTION 2: DATA TEMPAT TINGGAL & KELUARGA (KK) -->
    <div class="pg-card" style="display:flex; flex-direction:column; gap:var(--pg-s4);">
      <div style="display:flex; align-items:center; justify-content:space-between; border-bottom:1px solid var(--pg-border-soft); padding-bottom:var(--pg-s3);">
        <div>
          <h2 style="font-size:16px; font-weight:700; color:var(--pg-text); margin:0;">
            🏠 2. Data Rumah & Kartu Keluarga (KK)
          </h2>
          <p class="pg-muted" style="font-size:12px; margin:4px 0 0;">Hubungan keluarga dan kepemilikan unit hunian tempat tinggal di lingkungan.</p>
        </div>
        <span class="pg-chip" style="font-size:11px;">Domisili</span>
      </div>

      <!-- Grid 2 Kolom: Nomor Rumah & No KK -->
      <div style="display:grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap:var(--pg-s4);">
        <div class="pg-field">
          <label for="house_no" class="pg-label">Nomor Rumah / Blok (wajib)</label>
          <input type="text" list="house_list" id="house_no" name="house_no" class="pg-input" required placeholder="Contoh: Blok B-12 atau RT 05 No. 24" value="{{ $member.house_no }}">
          <datalist id="house_list">
            @foreach ($existing_houses as $h)
              <option value="{{ $h.house_no }}">{{ $h.house_no }} @if ($h.note) ({{ $h.note }}) @endif</option>
            @endforeach
          </datalist>
          <span class="pg-help">Bisa memilih nomor rumah yang sudah ada atau mengetik nomor rumah baru.</span>
        </div>

        <div class="pg-field">
          <label for="family_card_no" class="pg-label">Nomor Kartu Keluarga / No. KK (opsional)</label>
          <input type="text" id="family_card_no" name="family_card_no" class="pg-input" maxlength="20" placeholder="16 Digit Nomor KK" value="{{ $member.family_card_no }}">
          <span class="pg-help">Nomor Kartu Keluarga tempat warga ini terdaftar.</span>
        </div>
      </div>

      <!-- Grid 2 Kolom: Status di Keluarga & Status Kepemilikan -->
      <div style="display:grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap:var(--pg-s4);">
        <div class="pg-field">
          <label for="family_role" class="pg-label">Status dalam Keluarga</label>
          <select id="family_role" name="family_role" class="pg-input">
            <option value="Kepala Keluarga" @if ($member.family_role == 'Kepala Keluarga') selected @endif>Kepala Keluarga (KK)</option>
            <option value="Istri" @if ($member.family_role == 'Istri') selected @endif>Istri</option>
            <option value="Anak" @if ($member.family_role == 'Anak') selected @endif>Anak</option>
            <option value="Orang Tua" @if ($member.family_role == 'Orang Tua') selected @endif>Orang Tua / Mertua</option>
            <option value="Famili Lain" @if ($member.family_role == 'Famili Lain') selected @endif>Famili / Kerabat Lain</option>
          </select>
        </div>

        <div class="pg-field">
          <label for="ownership_status" class="pg-label">Status Kepemilikan Rumah</label>
          <select id="ownership_status" name="ownership_status" class="pg-input">
            <option value="Milik Sendiri" @if ($member.ownership_status == 'Milik Sendiri') selected @endif>Milik Sendiri (Hak Milik)</option>
            <option value="Sewa / Kontrak" @if ($member.ownership_status == 'Sewa / Kontrak') selected @endif>Sewa / Kontrak</option>
            <option value="Menumpang" @if ($member.ownership_status == 'Menumpang') selected @endif>Menumpang Keluarga</option>
            <option value="Kos" @if ($member.ownership_status == 'Kos') selected @endif>Kos / Singgah</option>
            <option value="Rumah Dinas" @if ($member.ownership_status == 'Rumah Dinas') selected @endif>Rumah Dinas</option>
          </select>
        </div>
      </div>

      <!-- Grid 2 Kolom: RT/RW & Catatan Hunian -->
      <div style="display:grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap:var(--pg-s4);">
        <div class="pg-field">
          <label for="rt_rw" class="pg-label">RT / RW Lingkungan (opsional)</label>
          <input type="text" id="rt_rw" name="rt_rw" class="pg-input" placeholder="Contoh: RT 05 / RW 09" value="{{ $member.rt_rw }}">
        </div>

        <div class="pg-field">
          <label for="house_note" class="pg-label">Catatan Ciri Rumah (opsional)</label>
          <input type="text" id="house_note" name="house_note" class="pg-input" placeholder="Contoh: Rumah Pagar Hitam, Samping Pos Ronda" value="{{ $member.house_note }}">
        </div>
      </div>
    </div>

    <!-- SECTION 3: STATUS KEANGGOTAAN & ADMINISTRASI IURAN -->
    <div class="pg-card" style="display:flex; flex-direction:column; gap:var(--pg-s4);">
      <div style="display:flex; align-items:center; justify-content:space-between; border-bottom:1px solid var(--pg-border-soft); padding-bottom:var(--pg-s3);">
        <div>
          <h2 style="font-size:16px; font-weight:700; color:var(--pg-text); margin:0;">
            ⚙️ 3. Keanggotaan & Administrasi Iuran
          </h2>
          <p class="pg-muted" style="font-size:12px; margin:4px 0 0;">Pengaturan status aktif warga dan otomatisasi invoice penagihan kas paguyuban.</p>
        </div>
        <span class="pg-chip" style="font-size:11px;">Administrasi</span>
      </div>

      <!-- Grid 2 Kolom: Status Warga & Tanggal Bergabung -->
      <div style="display:grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap:var(--pg-s4);">
        <div class="pg-field">
          <label for="status" class="pg-label">Status Keaktifan Warga (wajib)</label>
          <select id="status" name="status" class="pg-input" required>
            <option value="aktif" @if ($member.status == 'aktif') selected @endif>🟢 Aktif (Tinggal Menetap)</option>
            <option value="menunggu" @if ($member.status == 'menunggu') selected @endif>🟡 Menunggu Verifikasi Pengurus</option>
            <option value="nonaktif" @if ($member.status == 'nonaktif') selected @endif>⚪ Nonaktif (Pindah / Tidak Aktif)</option>
          </select>
          <span class="pg-help">Warga dengan status aktif akan otomatis menerima penerbitan tagihan iuran.</span>
        </div>

        <div class="pg-field">
          <label for="joined_on" class="pg-label">Tanggal Mulai Tinggal / Bergabung</label>
          <input type="date" id="joined_on" name="joined_on" class="pg-input" value="{{ $member.joined_on }}">
          <span class="pg-help">Digunakan sebagai titik awal penerbitan tagihan iuran bulanan.</span>
        </div>
      </div>

      <!-- Checkbox Penanggung Jawab Iuran -->
      <div style="background:var(--pg-bg); padding:var(--pg-s3) var(--pg-s4); border-radius:var(--pg-r-ctl); border:1px solid var(--pg-border-soft); display:flex; align-items:flex-start; gap:var(--pg-s3);">
        <input type="checkbox" id="is_fee_payer" name="is_fee_payer" value="1" @if ($member.is_fee_payer) checked @endif style="width:18px; height:18px; margin-top:2px; accent-color:var(--pg-primary); cursor:pointer;">
        <div>
          <label for="is_fee_payer" style="font-size:13px; font-weight:700; color:var(--pg-text); cursor:pointer; display:block;">
            Penanggung Jawab Tagihan Iuran Rumah
          </label>
          <span class="pg-muted" style="font-size:12px; display:block; margin-top:2px;">
            Centang opsi ini jika tagihan iuran paguyuban/RT untuk unit rumah ini dibebankan atas nama warga ini (biasanya Kepala Keluarga).
          </span>
        </div>
      </div>

      <!-- Catatan Khusus Pengurus -->
      <div class="pg-field">
        <label for="notes" class="pg-label">Catatan Khusus Pengurus (opsional)</label>
        <textarea id="notes" name="notes" rows="2" class="pg-input" style="height:auto; padding:10px var(--pg-s3);" placeholder="Contoh: Anggota tim ronda RT, memiliki keahlian medis, atau catatan khusus lainnya...">{{ $member.notes }}</textarea>
        <span class="pg-help">Catatan ini bersifat internal dan hanya dapat dibaca oleh pengurus paguyuban / RT.</span>
      </div>
    </div>

    <!-- ACTION BUTTONS -->
    <div class="pg-card" style="display:flex; justify-content:space-between; align-items:center; flex-wrap:wrap; gap:var(--pg-s3); padding:var(--pg-s4);">
      <div style="display:flex; align-items:center; gap:var(--pg-s2);">
        <span style="font-size:16px;">🛡️</span>
        <span class="pg-muted" style="font-size:12px;">
          Data kependudukan warga dilindungi oleh isolasi Row Level Security (RLS) PostgreSQL.
        </span>
      </div>

      <div style="display:flex; gap:var(--pg-s3); align-items:center; flex-wrap:wrap;">
        <a href="/anggota" class="pg-btn pg-btn--outline">
          Batal
        </a>
        <button type="submit" class="pg-btn pg-btn--primary" style="min-width:180px;">
          💾 @if ($is_edit) Perbarui Data Warga @else Daftarkan Warga Baru @endif
        </button>
      </div>
    </div>

  </form>
</div>
@endsection
