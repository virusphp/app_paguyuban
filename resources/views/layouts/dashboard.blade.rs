<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{{ $title ?? 'Dashboard Paguyuban' }}</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@400;600;700&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="/css/tokens.css">
  <link rel="stylesheet" href="/css/components.css">
</head>
<body>
  <div class="pg-app">
    <!-- Backdrop Overlay untuk Mobile Drawer -->
    <div class="pg-sidebar-backdrop" id="pg-sidebar-backdrop"></div>

    @include('components/sidebar.blade.rs')

    <div class="pg-main-wrapper">
      <header class="pg-topbar">
        <div class="pg-topbar__left">
          <!-- Tombol Hamburger Mobile Drawer -->
          <button type="button" class="pg-sidebar-toggle" id="pg-sidebar-toggle" aria-label="Buka menu navigasi">&#9776;</button>

          <div class="pg-topbar__org">
            @if ($user_orgs && count($user_orgs) > 1)
            <form action="/pilih-organisasi" method="POST" style="margin:0; display:flex; align-items:center; gap:6px;">
              @csrf
              <label for="org_select" class="pg-muted" style="font-size:12px; display:none;">Organisasi:</label>
              <select id="org_select" name="org_id" class="pg-select" onchange="this.form.submit()">
                @foreach ($user_orgs as $org)
                <option value="{{ $org.id }}" @selected($org.id == $current_org_id)>{{ $org.name }}</option>
                @endforeach
              </select>
            </form>
            @else
            <span>{{ $org_name ?? 'Paguyuban' }}</span>
            @endif
          </div>
        </div>

        <div class="pg-topbar__user">
          @if (($area ?? '') == 'sa')
          <a href="/pilih-organisasi" class="pg-btn pg-btn--outline pg-btn--sm" style="font-size:12px;">
            🏢 Dashboard Organisasi
          </a>
          @elseif (($is_superadmin ?? false) || ($role_label ?? '') == 'SUPERADMIN')
          <a href="/sa/dashboard" class="pg-btn pg-btn--primary pg-btn--sm" style="font-size:12px; font-weight:700;">
            🎛️ Portal Superadmin
          </a>
          @endif
          <span class="pg-muted">{{ $user_name ?? 'Pengguna' }}</span>
          <span class="pg-chip pg-chip--ok">{{ $role_label ?? 'Pengurus' }}</span>
          <a href="/logout" class="pg-btn pg-btn--outline pg-btn--sm">Keluar</a>
        </div>
      </header>

      <main class="pg-main">
        @if ($flash_success)
        <div class="pg-alert pg-alert--ok" role="alert">
          {{ $flash_success }}
        </div>
        @endif
        @if ($flash_error)
        <div class="pg-alert pg-alert--bad" role="alert">
          {{ $flash_error }}
        </div>
        @endif

        @yield('content')
      </main>
    </div>
  </div>

  <script>
    (function() {
      const toggleBtn = document.getElementById('pg-sidebar-toggle');
      const closeBtn = document.getElementById('pg-sidebar-close');
      const backdrop = document.getElementById('pg-sidebar-backdrop');
      const sidebar = document.querySelector('.pg-sidebar');

      function openSidebar() {
        if (sidebar) sidebar.classList.add('is-open');
        if (backdrop) backdrop.classList.add('is-open');
        document.body.style.overflow = 'hidden';
      }

      function closeSidebar() {
        if (sidebar) sidebar.classList.remove('is-open');
        if (backdrop) backdrop.classList.remove('is-open');
        document.body.style.overflow = '';
      }

      if (toggleBtn) toggleBtn.addEventListener('click', openSidebar);
      if (closeBtn) closeBtn.addEventListener('click', closeSidebar);
      if (backdrop) backdrop.addEventListener('click', closeSidebar);

      // Tutup otomatis saat link di sidebar diklik pada mode mobile
      if (sidebar) {
        sidebar.querySelectorAll('a').forEach(function(link) {
          link.addEventListener('click', function() {
            if (window.innerWidth <= 992) {
              closeSidebar();
            }
          });
        });
      }
    })();
  </script>
</body>
</html>
