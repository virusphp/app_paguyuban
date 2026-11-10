@extends('layouts.app')

@section('content')
<div class="relative overflow-hidden pt-12 pb-24">
    <!-- Ambient Glow Effects -->
    <div class="absolute top-[-10%] left-[-10%] w-[50vw] h-[50vw] bg-indigo-600/15 rounded-full blur-[140px] pointer-events-none"></div>
    <div class="absolute top-[20%] right-[-10%] w-[45vw] h-[45vw] bg-pink-600/10 rounded-full blur-[140px] pointer-events-none"></div>

    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10">
        <!-- Hero Section -->
        <div class="text-center max-w-4xl mx-auto">
            <div class="inline-flex items-center gap-2 py-1 px-3.5 rounded-full text-xs font-semibold bg-indigo-500/10 text-indigo-300 border border-indigo-500/25 mb-6 uppercase tracking-wider shadow-inner">
                <span class="w-2 h-2 rounded-full bg-emerald-400 animate-ping"></span>
                <span>Lumina v{{ version }} • High-Performance Rust Backend Engine</span>
            </div>
            <h1 class="text-5xl md:text-7xl font-extrabold tracking-tight text-white mb-6 heading-font leading-tight">
                The <span class="bg-clip-text text-transparent bg-gradient-to-r from-indigo-400 via-purple-300 to-pink-400">Elegance</span> of Laravel.<br>
                The <span class="text-white underline decoration-indigo-500 decoration-wavy decoration-2">Raw Speed</span> of Rust.
            </h1>
            <p class="text-lg md:text-xl text-slate-400 max-w-3xl mx-auto mb-10 leading-relaxed">
                Lumina is a developer-first Rust web framework specifically built for modern <strong>Backend APIs</strong>, microservices, and enterprise applications. Enjoy seamless routing, unified database pooling, and rapid scaffolding.
            </p>
            <div class="flex flex-wrap justify-center gap-4">
                <a href="/api" target="_blank" class="bg-gradient-to-r from-indigo-600 to-purple-600 text-white px-7 py-3.5 rounded-xl font-bold hover:shadow-lg hover:shadow-indigo-500/25 transition-all transform hover:-translate-y-0.5 flex items-center gap-2">
                    <span>🚀 Inspect API Spec</span>
                    <span class="text-xs bg-black/30 px-2 py-0.5 rounded-md mono-font">GET /api</span>
                </a>
                <a href="https://github.com/lumina-java/framework" target="_blank" class="bg-slate-900/80 text-slate-200 border border-slate-800 px-7 py-3.5 rounded-xl font-semibold hover:bg-slate-800 hover:text-white transition-all transform hover:-translate-y-0.5 flex items-center gap-2">
                    <span>📖 Documentation</span>
                </a>
            </div>
        </div>

        <!-- Telemetry Status Grid (Laravel / Lumen Health Badges) -->
        <div class="mt-14 grid grid-cols-2 md:grid-cols-4 gap-4">
            <div class="glass-card rounded-2xl p-5 text-left">
                <div class="text-slate-400 text-xs font-semibold uppercase tracking-wider mb-1">Framework Version</div>
                <div class="text-xl font-bold text-white heading-font flex items-center gap-2">
                    <span>Lumina {{ version }}</span>
                </div>
                <div class="text-xs text-indigo-400 mt-1">Rust {{ rust_edition }} Edition</div>
            </div>

            <div class="glass-card rounded-2xl p-5 text-left">
                <div class="text-slate-400 text-xs font-semibold uppercase tracking-wider mb-1">Runtime Engine</div>
                <div class="text-xl font-bold text-white heading-font">Axum 0.7</div>
                <div class="text-xs text-emerald-400 mt-1">Tokio Multi-Threaded Async</div>
            </div>

            <div class="glass-card rounded-2xl p-5 text-left">
                <div class="text-slate-400 text-xs font-semibold uppercase tracking-wider mb-1">Database Driver</div>
                <div class="text-xl font-bold text-white heading-font flex items-center gap-2">
                    <span>{{ db_driver }}</span>
                    <span class="w-2.5 h-2.5 rounded-full {% if db_status == 'connected' %}bg-emerald-400 shadow-sm shadow-emerald-400/50{% else %}bg-amber-400{% endif %}"></span>
                </div>
                <div class="text-xs {% if db_status == 'connected' %}text-emerald-400{% else %}text-amber-400{% endif %} mt-1 font-medium capitalize">{{ db_status }}</div>
            </div>

            <div class="glass-card rounded-2xl p-5 text-left">
                <div class="text-slate-400 text-xs font-semibold uppercase tracking-wider mb-1">Environment</div>
                <div class="text-xl font-bold text-white heading-font">Local</div>
                <div class="text-xs text-slate-400 mt-1">Debug Mode Active</div>
            </div>
        </div>

        <!-- Dual Mode Lumen API Showcase -->
        <div class="mt-10 bg-slate-950/80 border border-slate-800 rounded-3xl p-6 md:p-8 backdrop-blur-md shadow-2xl text-left">
            <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 border-b border-slate-800/80 pb-5 mb-6">
                <div>
                    <div class="flex items-center gap-2">
                        <span class="px-2.5 py-0.5 rounded-full text-xs font-bold bg-pink-500/20 text-pink-300 border border-pink-500/30">Dual-Mode API</span>
                        <h2 class="text-xl font-bold text-white heading-font">Lumen-Style Headless JSON Response</h2>
                    </div>
                    <p class="text-slate-400 text-sm mt-1">Kirim header <code class="mono-font bg-slate-900 text-indigo-300 px-1.5 py-0.5 rounded">Accept: application/json</code> ke URL root (<code class="mono-font text-indigo-300">/</code>) untuk langsung menerima JSON spec.</p>
                </div>
                <div class="flex items-center gap-2">
                    <a href="/?format=json" target="_blank" class="text-xs font-semibold bg-slate-800 hover:bg-slate-700 text-indigo-300 px-3 py-1.5 rounded-lg border border-slate-700 transition flex items-center gap-1.5">
                        <span>Lihat Raw JSON</span>
                        <span>↗</span>
                    </a>
                </div>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <!-- Terminal Command -->
                <div>
                    <div class="text-xs font-semibold text-slate-400 uppercase tracking-wider mb-2">1. Coba Request via Terminal / cURL:</div>
                    <div class="bg-black/90 rounded-2xl p-4 border border-slate-800 mono-font text-sm text-slate-300 relative group">
                        <div class="flex items-center justify-between text-xs text-slate-500 pb-2 border-b border-slate-800/60 mb-3">
                            <span>bash terminal</span>
                            <span class="text-emerald-400 font-semibold">GET /</span>
                        </div>
                        <div class="overflow-x-auto text-pink-400 select-all">
                            curl -H "Accept: application/json" http://127.0.0.1:8000/
                        </div>
                    </div>
                </div>

                <!-- JSON Response Preview -->
                <div>
                    <div class="text-xs font-semibold text-slate-400 uppercase tracking-wider mb-2">2. JSON Response yang Dihasilkan:</div>
                    <pre class="bg-black/90 rounded-2xl p-4 border border-slate-800 mono-font text-xs text-emerald-400 overflow-x-auto leading-relaxed">
{
  <span class="text-slate-400">"name"</span>: <span class="text-amber-300">"Lumina Framework"</span>,
  <span class="text-slate-400">"version"</span>: <span class="text-amber-300">"{{ version }}"</span>,
  <span class="text-slate-400">"engine"</span>: <span class="text-amber-300">"Axum 0.7 + Tokio Async"</span>,
  <span class="text-slate-400">"database"</span>: {
    <span class="text-slate-400">"driver"</span>: <span class="text-amber-300">"{{ db_driver }}"</span>,
    <span class="text-slate-400">"status"</span>: <span class="text-amber-300">"{{ db_status }}"</span>
  },
  <span class="text-slate-400">"documentation"</span>: <span class="text-amber-300">"https://github.com/lumina-java/framework"</span>
}</pre>
                </div>
            </div>
        </div>

        <!-- Backend Capabilities Feature Cards -->
        <div class="mt-14 grid grid-cols-1 md:grid-cols-3 gap-6 text-left">
            <div class="glass-card rounded-3xl p-7 transition duration-300">
                <div class="w-12 h-12 bg-indigo-500/10 border border-indigo-500/20 text-indigo-400 rounded-2xl flex items-center justify-center text-2xl mb-5">
                    🛣️
                </div>
                <h3 class="text-xl font-bold text-white heading-font mb-2">Dedicated API Routing</h3>
                <p class="text-slate-400 text-sm leading-relaxed mb-4">
                    Pisahkan rute API dan Web dengan mudah. File <code class="mono-font bg-slate-900 text-indigo-300 px-1 rounded">src/routes/api.rs</code> otomatis diprefix dengan <code class="mono-font text-indigo-300">/api</code> lengkap dengan middleware CORS & Rate Limiter.
                </p>
                <a href="/api/health" target="_blank" class="text-xs font-semibold text-indigo-400 hover:text-indigo-300 flex items-center gap-1">
                    Cek /api/health <span>&rarr;</span>
                </a>
            </div>

            <div class="glass-card rounded-3xl p-7 transition duration-300">
                <div class="w-12 h-12 bg-pink-500/10 border border-pink-500/20 text-pink-400 rounded-2xl flex items-center justify-center text-2xl mb-5">
                    🗄️
                </div>
                <h3 class="text-xl font-bold text-white heading-font mb-2">Unified Multi-DB Pool</h3>
                <p class="text-slate-400 text-sm leading-relaxed mb-4">
                    Gunakan SQLite untuk development lokal cepat tanpa setup, lalu beralih ke <strong>PostgreSQL</strong> atau <strong>MySQL</strong> di production cukup dengan mengubah file <code class="mono-font bg-slate-900 text-indigo-300 px-1 rounded">.env</code>.
                </p>
                <span class="text-xs font-semibold text-pink-400 flex items-center gap-1">
                    PostgreSQL • MySQL • SQLite
                </span>
            </div>

            <div class="glass-card rounded-3xl p-7 transition duration-300">
                <div class="w-12 h-12 bg-purple-500/10 border border-purple-500/20 text-purple-400 rounded-2xl flex items-center justify-center text-2xl mb-5">
                    🪄
                </div>
                <h3 class="text-xl font-bold text-white heading-font mb-2">Active Record Models</h3>
                <p class="text-slate-400 text-sm leading-relaxed mb-4">
                    Tulis model database semudah di Laravel Eloquent dengan makro sakti <code class="mono-font bg-slate-900 text-indigo-300 px-1 rounded">#[derive(LuminaModel)]</code>. Otomatis menyediakan <code class="mono-font text-indigo-300">all()</code>, <code class="mono-font text-indigo-300">find(id)</code>, dan CRUD.
                </p>
                <a href="https://github.com/lumina-java/framework#models" target="_blank" class="text-xs font-semibold text-purple-400 hover:text-purple-300 flex items-center gap-1">
                    Pelajari Makro Model <span>&rarr;</span>
                </a>
            </div>
        </div>

        <!-- Quick Start Guide -->
        <div class="mt-14 bg-slate-900/40 border border-slate-800 rounded-3xl p-8 md:p-10 text-left">
            <h3 class="text-2xl font-bold text-white heading-font mb-6 flex items-center gap-3">
                <span class="w-8 h-8 rounded-xl bg-indigo-600 text-white flex items-center justify-center text-sm font-black">⚡</span>
                <span>Cara Cepat Membuat API Endpoint Baru:</span>
            </h3>

            <div class="space-y-6">
                <div class="flex gap-4">
                    <div class="w-8 h-8 shrink-0 bg-indigo-500/20 text-indigo-300 rounded-full flex items-center justify-center font-bold text-sm border border-indigo-500/30">1</div>
                    <div class="flex-grow">
                        <div class="text-white font-semibold mb-1">Daftarkan rute di <code class="mono-font bg-slate-950 text-indigo-300 px-2 py-0.5 rounded text-sm">src/routes/api.rs</code></div>
                        <pre class="bg-black/80 p-3.5 rounded-xl text-xs mono-font text-slate-300 overflow-x-auto border border-slate-800/80 mt-2">
pub fn router() -> Router&lt;AppState&gt; {
    Router::new()
        .get("/products", get_products)
}</pre>
                    </div>
                </div>

                <div class="flex gap-4">
                    <div class="w-8 h-8 shrink-0 bg-indigo-500/20 text-indigo-300 rounded-full flex items-center justify-center font-bold text-sm border border-indigo-500/30">2</div>
                    <div class="flex-grow">
                        <div class="text-white font-semibold mb-1">Kembalikan respons JSON dengan mudah:</div>
                        <pre class="bg-black/80 p-3.5 rounded-xl text-xs mono-font text-slate-300 overflow-x-auto border border-slate-800/80 mt-2">
async fn get_products() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "success",
        "data": ["Rust", "Axum", "Lumina"]
    }))
}</pre>
                    </div>
                </div>
            </div>
        </div>

    </div>
</div>
@endsection
