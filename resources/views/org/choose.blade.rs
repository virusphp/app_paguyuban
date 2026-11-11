<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{{ $title ?? 'Pilih Organisasi' }}</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;500;600;700;800&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="/css/tokens.css">
  <link rel="stylesheet" href="/css/components.css">
  <style>
    .choose-layout {
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      background: var(--pg-bg);
      padding: var(--pg-s6) var(--pg-s4);
      align-items: center;
      justify-content: center;
    }
    .choose-container {
      width: 100%;
      max-width: 840px;
    }
    .choose-header {
      text-align: center;
      margin-bottom: var(--pg-s6);
    }
    .choose-header h1 {
      font-size: 26px;
      font-weight: 800;
      color: var(--pg-text);
      margin: 0 0 var(--pg-s2);
      letter-spacing: -0.5px;
    }
    .choose-header p {
      color: var(--pg-muted);
      font-size: 14px;
      margin: 0;
    }
    .org-card-grid {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
      gap: var(--pg-s4);
    }
    .org-select-card {
      background: var(--pg-surface);
      border: 1px solid var(--pg-border);
      border-radius: var(--pg-r-card);
      padding: var(--pg-s5);
      transition: all 0.2s ease;
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      position: relative;
    }
    .org-select-card:hover {
      border-color: var(--pg-primary);
      box-shadow: 0 8px 24px rgba(15, 118, 110, 0.08);
      transform: translateY(-2px);
    }
    .org-select-card.is-current {
      border-color: var(--pg-primary);
      background: #f0fdf4;
    }
    .org-badge {
      display: inline-flex;
      align-items: center;
      padding: 2px 8px;
      border-radius: var(--pg-r-pill);
      font-size: 11px;
      font-weight: 700;
      text-transform: uppercase;
      letter-spacing: 0.5px;
      margin-bottom: var(--pg-s2);
    }
    .org-badge--rt { background: #e0f2fe; color: #0369a1; }
    .org-badge--rw { background: #fef3c7; color: #b45309; }
    .org-badge--paguyuban { background: #dcfce7; color: #15803d; }
  </style>
</head>
<body>
  <div class="choose-layout">
    <div class="choose-container">
      <div class="choose-header">
        <div style="display:inline-flex; align-items:center; gap:8px; margin-bottom:12px; background:var(--pg-surface); padding:6px 14px; border-radius:999px; border:1px solid var(--pg-border); font-size:12px; font-weight:600; color:var(--pg-primary);">
          <span>🏛️ Multi-Organisasi</span>
          <span>•</span>
          <span>Masuk sebagai: <strong>{{ $user_name }}</strong></span>
        </div>
        <h1>Pilih Lingkungan / Organisasi</h1>
        <p>Silakan pilih unit Paguyuban, RW, atau RT yang ingin Anda akses dan kelola saat ini.</p>
      </div>

      <div class="org-card-grid">
        @forelse ($orgs as $o)
        <div class="org-select-card @if ($o.is_active) is-current @endif">
          <div>
            <span class="org-badge @if ($o.org_type == 'rt') org-badge--rt @elseif ($o.org_type == 'rw') org-badge--rw @else org-badge--paguyuban @endif">
              {{ $o.org_type }}
            </span>
            <h2 style="font-size:17px; font-weight:700; color:var(--pg-text); margin:0 0 6px;">{{ $o.name }}</h2>
            <div style="font-size:12px; color:var(--pg-muted); margin-bottom:12px;">
              Hak Akses: <strong>{{ $o.role }}</strong>
            </div>
            <div style="font-size:12px; color:var(--pg-muted); display:flex; align-items:center; gap:6px; margin-bottom:18px;">
              <span>👥 {{ $o.members_count }} Warga Terdata</span>
            </div>
          </div>

          <form action="/pilih-organisasi" method="POST" style="margin:0;">
            @csrf
            <input type="hidden" name="org_id" value="{{ $o.id }}">
            <button type="submit" class="pg-btn @if ($o.is_active) pg-btn--primary @else pg-btn--outline @endif" style="width:100%; justify-content:center;">
              @if ($o.is_active) Buka Dashboard (Aktif) @else Masuk Kelola &rarr; @endif
            </button>
          </form>
        </div>
        @empty
        <div class="pg-card pg-empty" style="grid-column: 1 / -1; text-align:center; padding:48px 24px;">
          <p style="font-size:15px; margin-bottom:12px;">Anda belum terdaftar pada organisasi mana pun.</p>
          @if ($is_superadmin)
          <a href="/sa/organisasi/baru" class="pg-btn pg-btn--primary">+ Daftarkan Organisasi Baru</a>
          @endif
        </div>
        @endforelse
      </div>

      <div style="margin-top:var(--pg-s6); display:flex; justify-content:space-between; align-items:center; flex-wrap:wrap; gap:12px;">
        @if ($is_superadmin)
        <a href="/sa/dashboard" class="pg-btn pg-btn--outline" style="border-style:dashed;">
          🎛️ Kembali ke Portal Superadmin / Developer
        </a>
        @else
        <div></div>
        @endif
        <a href="/logout" class="pg-btn pg-btn--outline" style="color:var(--pg-bad-fg); border-color:var(--pg-bad-bg);">
          Keluar dari Akun
        </a>
      </div>
    </div>
  </div>
</body>
</html>

