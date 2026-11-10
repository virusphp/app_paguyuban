@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>@if ($is_edit)Ubah Surat / Undangan@else Buat Surat & Undangan Baru@endif</h1>
    <p class="pg-muted">Terbitkan surat pengantar warga, edaran resmi pengurus, atau undangan kegiatan lingkungan.</p>
  </div>
  <div>
    <a href="/surat" class="pg-btn pg-btn--outline">← Kembali ke Arsip Surat</a>
  </div>
</div>

<div class="pg-card" style="max-width:760px; margin:0 auto;">
  <form method="POST" action="@if ($is_edit)/surat/{{ $letter.id }}@else/surat/baru@endif">
    @csrf

    <div class="pg-grid-2">
      <div class="pg-field">
        <label for="kind">Jenis Dokumen (wajib)</label>
        <select id="kind" name="kind" class="pg-select" style="height:42px;" required>
          <option value="surat" @selected($letter.kind == 'surat')>Surat Resmi / Pengantar / Edaran</option>
          <option value="undangan" @selected($letter.kind == 'undangan')>Surat Undangan Acara Warga</option>
        </select>
      </div>

      <div class="pg-field">
        <label for="number">Nomor Registrasi Surat (wajib)</label>
        <input type="text" id="number" name="number" class="pg-input" value="{{ $letter.number }}" required>
        <span class="pg-muted" style="font-size:12px;">Format penomoran standar organisasi (dapat disesuaikan).</span>
      </div>
    </div>

    <!-- Pilihan Template Bawaan / Kustom -->
    <div class="pg-field" style="background:var(--pg-track); padding:12px 16px; border-radius:var(--pg-r-card); margin-bottom:16px;">
      <label for="template_select" style="font-size:13px; font-weight:700;">Gunakan Template Cepat (Opsional)</label>
      <div style="display:flex; gap:8px;">
        <select id="template_select" class="pg-select" style="height:38px; flex:1; font-size:13px;">
          <option value="">-- Pilih Format Template Cepat --</option>
          <option value="pengantar">Surat Pengantar RT / Pembuatan KTP / Domisili</option>
          <option value="undangan_rapat">Surat Undangan Musyawarah Warga / Rapat RT</option>
          <option value="edaran_keamanan">Surat Edaran Keamanan & Ketertiban Lingkungan</option>
          <option value="keterangan_bantuan">Surat Keterangan Pengantar Bantuan Warga</option>
          @foreach ($custom_templates as $tmpl)
          <option value="custom_{{ $tmpl.id }}" data-body="{{ $tmpl.body }}">{{ $tmpl.name }}</option>
          @endforeach
        </select>
        <button type="button" class="pg-btn pg-btn--outline pg-btn--sm" onclick="applyTemplate()">Terapkan</button>
      </div>
    </div>

    <div class="pg-grid-2">
      <div class="pg-field">
        <label for="recipient">Ditujukan Kepada (Penerima) (wajib)</label>
        <input type="text" id="recipient" name="recipient" class="pg-input" value="{{ $letter.recipient }}" placeholder="Contoh: Bpk. Bambang Sudarsono / Seluruh Warga RT 03" required>
      </div>

      <div class="pg-field">
        <label for="status">Status Surat</label>
        <select id="status" name="status" class="pg-select" style="height:42px;">
          <option value="draft" @selected($letter.status == 'draft')>Draf (Konsep Belum Terkirim)</option>
          <option value="terkirim" @selected($letter.status == 'terkirim')>Terkirim (Resmi Didistribusikan)</option>
        </select>
      </div>
    </div>

    <div class="pg-field">
      <label for="subject">Perihal / Hal Surat (wajib)</label>
      <input type="text" id="subject" name="subject" class="pg-input" value="{{ $letter.subject }}" placeholder="Contoh: Undangan Rapat Evaluasi Kas Lingkungan & Tirakatan" required>
    </div>

    <div class="pg-field">
      <label for="body">Badan / Isi Lengkap Surat (wajib)</label>
      <textarea id="body" name="body" class="pg-input" style="height:260px; padding:12px; line-height:1.6; font-family:inherit;" placeholder="Tuliskan isi surat lengkap di sini..." required>{{ $letter.body }}</textarea>
    </div>

    <div class="pg-form-actions">
      <a href="/surat" class="pg-btn pg-btn--outline">Batal</a>
      <button type="submit" class="pg-btn">@if ($is_edit)Simpan Perubahan@else Terbitkan Surat@endif</button>
    </div>
  </form>
</div>

<script>
function applyTemplate() {
  const sel = document.getElementById('template_select');
  const val = sel.value;
  const subj = document.getElementById('subject');
  const body = document.getElementById('body');
  const kind = document.getElementById('kind');
  const recip = document.getElementById('recipient');

  if (val === 'pengantar') {
    kind.value = 'surat';
    subj.value = 'Surat Pengantar Warga / Kependudukan';
    recip.value = 'Ketua RW 05 / Petugas Kelurahan Pedalangan';
    body.value = `Dengan hormat,

Yang bertanda tangan di bawah ini Pengurus Paguyuban / RT, menerangkan bahwa:
Nama Lengkap : [Nama Warga]
Tempat/Tgl Lahir : [Tempat, Tanggal Lahir]
No. KTP / NIK : [Nomor Induk Kependudukan]
Alamat Tinggal : [Nomor Rumah / Blok / RT]

Adalah benar warga yang bertempat tinggal dan berdomisili sah di lingkungan kami. Surat pengantar ini diterbitkan guna keperluan pengurusan berkas administrasi kependudukan.

Demikian surat pengantar ini kami buat dengan sebenarnya untuk dapat dipergunakan sebagaimana mestinya.`;
  } else if (val === 'undangan_rapat') {
    kind.value = 'undangan';
    subj.value = 'Undangan Musyawarah Warga & Rapat Evaluasi Kas';
    recip.value = 'Seluruh Bpk/Ibu Warga RT';
    body.value = `Dengan hormat,

Mengharap kehadiran Bapak / Ibu warga dalam pertemuan musyawarah lingkungan yang akan diselenggarakan pada:
Hari / Tanggal : [Hari, Tanggal Bulan Tahun]
Waktu : Pukul 19.30 WIB s.d. selesai
Tempat : Balai Warga / Posko Paguyuban
Agenda Acara :
1. Laporan keuangan kas dan pertanggungjawaban iuran warga.
2. Pembahasan rencana kerja bakti lingkungan.
3. Lain-lain dan sesi tanya jawab warga.

Mengingat pentingnya agenda musyawarah ini, kami sangat mengharapkan kehadiran tepat pada waktunya. Atas perhatian dan partisipasinya kami sampaikan terima kasih.`;
  } else if (val === 'edaran_keamanan') {
    kind.value = 'surat';
    subj.value = 'Surat Edaran Peningkatan Keamanan & Ketertiban Lingkungan';
    recip.value = 'Seluruh Warga Lingkungan';
    body.value = `Dengan hormat,

Sehubungan dengan upaya bersama menjaga ketertiban, ketenteraman, dan keamanan lingkungan, bersama ini Pengurus menyampaikan beberapa hal penting sebagai berikut:
1. Pintu gerbang utama lingkungan diberlakukan sistem satu pintu mulai pukul 23.00 WIB.
2. Setiap tamu yang menginap lebih dari 1x24 jam wajib melapor kepada Pengurus RT.
3. Petugas ronda malam diharapkan menjalankan jadwal jaga pos kamling sesuai giliran.

Demikian surat edaran ini disampaikan untuk dipedomani dan dilaksanakan bersama demi kenyamanan seluruh warga.`;
  } else if (val === 'keterangan_bantuan') {
    kind.value = 'surat';
    subj.value = 'Surat Keterangan Warga Layak Menerima Bantuan';
    recip.value = 'Dinas Sosial / Tim Penyalur Bantuan';
    body.value = `Dengan hormat,

Menerangkan dengan sebenarnya bahwa warga yang tercantum di bawah ini:
Nama : [Nama Warga]
Alamat : [Nomor Rumah / RT]

Berdasarkan data dan evaluasi pengurus lingkungan, tergolong warga yang berhak dan memenuhi syarat untuk menerima program bantuan sosial / sembako.

Surat keterangan ini kami buat dengan penuh rasa tanggung jawab.`;
  }
}
</script>
@endsection

