<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>404 — Halaman Tidak Ditemukan</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;600;700&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="/css/tokens.css">
  <link rel="stylesheet" href="/css/components.css">
  <style>
    .error-container {
      min-height: 100vh;
      display: flex;
      align-items: center;
      justify-content: center;
      padding: var(--pg-s4);
      background: var(--pg-bg);
      text-align: center;
    }
    .error-card {
      width: 100%;
      max-width: 480px;
      background: var(--pg-surface);
      border: 1px solid var(--pg-border);
      border-radius: var(--pg-r-card);
      padding: var(--pg-s8) var(--pg-s6);
      box-shadow: 0 4px 20px rgba(0,0,0,0.05);
    }
    .error-code {
      font-size: 72px;
      font-weight: 800;
      line-height: 1;
      color: var(--pg-brand);
      letter-spacing: -2px;
      margin-bottom: var(--pg-s3);
    }
    .error-title {
      font-size: 20px;
      font-weight: 700;
      color: var(--pg-text);
      margin-bottom: var(--pg-s2);
    }
    .error-desc {
      font-size: 14px;
      color: var(--pg-muted);
      line-height: 1.5;
      margin-bottom: var(--pg-s6);
    }
    .error-actions {
      display: flex;
      gap: var(--pg-s3);
      justify-content: center;
    }
  </style>
</head>
<body>
  <main class="error-container">
    <div class="error-card">
      <div class="error-code">404</div>
      <h1 class="error-title">Halaman Tidak Ditemukan</h1>
      <p class="error-desc">
        Mohon maaf, tautan yang Anda tuju tidak tersedia, telah dipindahkan, atau alamat URL yang dimasukkan salah.
      </p>
      <div class="error-actions">
        <a href="/dashboard" class="pg-btn pg-btn--primary">Kembali ke Dashboard</a>
        <a href="javascript:history.back()" class="pg-btn pg-btn--ghost">Halaman Sebelumnya</a>
      </div>
    </div>
  </main>
</body>
</html>

