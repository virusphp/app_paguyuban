<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Masuk — Aplikasi Manajemen Paguyuban</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;600;700&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="/css/tokens.css">
  <link rel="stylesheet" href="/css/components.css">
  <style>
    .login-container {
      min-height: 100vh;
      display: flex;
      align-items: center;
      justify-content: center;
      padding: var(--pg-s4);
      background: var(--pg-bg);
    }
    .login-box {
      width: 100%;
      max-width: 420px;
      background: var(--pg-surface);
      border: 1px solid var(--pg-border);
      border-radius: var(--pg-r-card);
      padding: var(--pg-s6);
      box-shadow: 0 4px 16px rgba(0,0,0,0.04);
    }
    .login-title {
      font-size: 22px;
      font-weight: 700;
      color: var(--pg-text);
      margin-bottom: var(--pg-s1);
    }
    .login-subtitle {
      font-size: 13px;
      color: var(--pg-muted);
      margin-bottom: var(--pg-s5);
    }
  </style>
</head>
<body>
  <div class="login-container">
    <div class="login-box">
      <div class="login-title">Masuk Pengurus</div>
      <div class="login-subtitle">Aplikasi Manajemen Paguyuban, RW, dan RT</div>

      @if ($error)
      <div class="pg-alert pg-alert--bad" role="alert">
        {{ $error }}
      </div>
      @endif

      <form action="/login" method="POST">
        @csrf
        
        <div class="pg-field">
          <label for="f-phone">Nomor HP (wajib)</label>
          <input class="pg-input" id="f-phone" name="phone" type="tel" value="{{ $old_phone ?? '' }}" placeholder="Contoh: 081234567890" required autofocus>
          <div class="pg-field__help">Gunakan nomor HP yang telah terdaftar sebagai pengurus.</div>
        </div>

        <div class="pg-field">
          <label for="f-password">Kata Sandi (wajib)</label>
          <input class="pg-input" id="f-password" name="password" type="password" placeholder="Masukkan kata sandi" required>
        </div>

        <div style="margin-top: var(--pg-s5);">
          <button type="submit" class="pg-btn" style="width: 100%;">Masuk ke Dashboard</button>
        </div>
      </form>

      <div style="margin-top: var(--pg-s5); text-align: center;" class="pg-muted">
        Butuh bantuan akses? Hubungi pengurus paguyuban atau ketua RT setempat.
      </div>
    </div>
  </div>
</body>
</html>
