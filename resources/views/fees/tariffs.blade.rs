@extends('layouts/dashboard.blade.rs')

@section('content')
<div class="pg-page-head">
  <div>
    <h1>Pengaturan Tarif Iuran</h1>
    <p class="pg-muted">Kelola jenis dan besaran nominal iuran rutin paguyuban / RT.</p>
  </div>
  <div>
    <a href="/iuran" class="pg-btn pg-btn--outline">← Kembali ke Rekap Iuran</a>
  </div>
</div>

<div class="pg-grid-2">
  <!-- Daftar Tarif Aktif -->
  <div class="pg-card">
    <h2 class="pg-card__title">Tarif Iuran Terdaftar</h2>
    <div class="pg-table-wrap">
      <table class="pg-table" aria-label="Tabel Tarif Iuran">
        <thead>
          <tr>
            <th>Nama Iuran</th>
            <th>Periode</th>
            <th class="num">Nominal</th>
            <th>Mulai Berlaku</th>
          </tr>
        </thead>
        <tbody>
          @forelse ($fee_plans as $plan)
          <tr>
            <td style="font-weight:600;">{{ $plan.name }}</td>
            <td>
              <span class="pg-chip pg-chip--ok">{{ $plan.period }}</span>
            </td>
            <td class="num" style="font-weight:700; color:var(--pg-primary);">
              {{ $plan.amount_idr | rupiah }}
            </td>
            <td class="pg-muted">{{ $plan.active_from }}</td>
          </tr>
          @empty
          <tr>
            <td colspan="4" class="pg-empty">
              <p>Belum ada tarif iuran yang ditentukan.</p>
            </td>
          </tr>
          @endforelse
        </tbody>
      </table>
    </div>
  </div>

  <!-- Form Tambah Tarif Baru -->
  <div class="pg-card">
    <h2 class="pg-card__title">+ Tambah Tarif Iuran Baru</h2>
    <form method="POST" action="/iuran/tarif">
      @csrf

      <div class="pg-field">
        <label for="name">Nama Iuran / Pos</label>
        <input type="text" id="name" name="name" class="pg-input" placeholder="Contoh: Iuran Wajib Warga" required>
        <div class="pg-field__help">Nama pos iuran yang mudah dikenali oleh warga.</div>
      </div>

      <div class="pg-field">
        <label for="amount_idr">Nominal (Rupiah)</label>
        <input type="number" id="amount_idr" name="amount_idr" class="pg-input" placeholder="Contoh: 25000" min="1000" step="1000" required>
        <div class="pg-field__help">Nominal rupiah bulat tanpa tanda titik atau koma (contoh: 25000).</div>
      </div>

      <div class="pg-field">
        <label for="period">Periode Penagihan</label>
        <select id="period" name="period" class="pg-input" style="height:42px;">
          <option value="bulanan" selected>Bulanan (Setiap Bulan)</option>
          <option value="tahunan">Tahunan</option>
          <option value="insidental">Insidental / Khusus</option>
        </select>
      </div>

      <div class="pg-field">
        <label for="active_from">Tanggal Mulai Berlaku</label>
        <input type="date" id="active_from" name="active_from" class="pg-input" value="{{ $today_date }}" required>
      </div>

      <div style="margin-top:24px;">
        <button type="submit" class="pg-btn" style="width:100%;">Simpan Tarif Iuran</button>
      </div>
    </form>
  </div>
</div>
@endsection

