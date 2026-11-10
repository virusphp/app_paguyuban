<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title | default("Lumina Framework") }}</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500;600&family=Plus+Jakarta+Sans:wght@300;400;500;600;700;800&family=Space+Grotesk:wght@500;600;700;800&display=swap" rel="stylesheet">
    <style>
        body {
            font-family: 'Plus Jakarta Sans', sans-serif;
            background-color: #030712;
        }
        .heading-font {
            font-family: 'Space Grotesk', sans-serif;
        }
        .mono-font {
            font-family: 'JetBrains Mono', monospace;
        }
        .glass-nav {
            background: rgba(10, 15, 30, 0.75);
            backdrop-filter: blur(16px);
            border-bottom: 1px solid rgba(255, 255, 255, 0.08);
        }
        .glass-card {
            background: rgba(17, 24, 39, 0.6);
            backdrop-filter: blur(12px);
            border: 1px solid rgba(255, 255, 255, 0.08);
        }
        .glass-card:hover {
            border-color: rgba(99, 102, 241, 0.4);
            box-shadow: 0 10px 30px -10px rgba(99, 102, 241, 0.2);
        }
    </style>
</head>
<body class="text-slate-100 min-h-screen flex flex-col justify-between selection:bg-indigo-500 selection:text-white">

    <!-- Top Navigation Header -->
    <nav class="glass-nav sticky top-0 z-50">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex justify-between h-16 items-center">
                <div class="flex items-center space-x-3">
                    <a href="/" class="flex items-center space-x-3">
                        <div class="w-9 h-9 bg-gradient-to-tr from-indigo-500 via-purple-500 to-pink-500 rounded-xl flex items-center justify-center shadow-lg shadow-indigo-500/25">
                            <span class="text-white font-black heading-font text-xl">⚡</span>
                        </div>
                        <div class="flex items-center gap-2">
                            <span class="text-xl font-bold tracking-tight text-white heading-font">Lumina</span>
                            <span class="text-[10px] uppercase font-bold tracking-widest px-2 py-0.5 rounded-full bg-indigo-500/20 text-indigo-300 border border-indigo-500/30">Backend</span>
                        </div>
                    </a>
                </div>
                <div class="flex items-center space-x-6">
                    <a href="/api" target="_blank" class="text-slate-300 hover:text-indigo-400 font-medium text-sm transition flex items-center gap-1.5">
                        <span class="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
                        API Endpoints
                    </a>
                    <a href="https://github.com/lumina-java/framework" target="_blank" class="text-slate-300 hover:text-indigo-400 font-medium text-sm transition">Docs</a>
                    <a href="https://github.com/lumina-java/framework" target="_blank" class="bg-indigo-600 hover:bg-indigo-500 text-white px-4 py-2 rounded-xl text-sm font-semibold shadow-lg shadow-indigo-500/20 transition-all">GitHub Repo</a>
                </div>
            </div>
        </div>
    </nav>

    <!-- Main Content Slot -->
    <main class="flex-grow">
        @yield('content')
    </main>

    <!-- Footer Segment -->
    <footer class="border-t border-slate-900 bg-black/60 py-8">
        <div class="max-w-7xl mx-auto px-4 text-center text-slate-500 text-sm flex flex-col sm:flex-row justify-between items-center gap-4">
            <div class="flex items-center gap-2">
                <span>&copy; 2026 <strong>Lumina Framework</strong>. All rights reserved.</span>
            </div>
            <div class="flex items-center gap-1.5 text-slate-400">
                Engineered with <span class="text-red-500">❤️</span> in <strong>Rust</strong> (Axum 0.7 + Tokio) for ultra-fast APIs.
            </div>
        </div>
    </footer>

</body>
</html>
