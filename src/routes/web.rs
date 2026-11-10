use crate::app::controllers::{
    AnnouncementController, AuthController, CashController, DashboardController, ErrorController,
    EventController, FeeController, LetterController, MemberController, OrgController,
    SuperadminController,
};
use lumina::prelude::*;

//                            WEB ROUTING
// Rute untuk web browser dashboard pengurus Paguyuban.

pub fn router() -> Router<AppState> {
    Router::new()
        // Halaman Utama & Autentikasi Pengurus (Root URL langsung Halaman Login / Dashboard)
        .get("/", AuthController::show_login)
        .post("/", AuthController::login)
        .get("/login", AuthController::show_login)
        .post("/login", AuthController::login)
        .get("/logout", AuthController::logout)
        .post("/logout", AuthController::logout)
        // Dashboard PC Pengurus
        .get("/dashboard", DashboardController::index)
        // Pengalihan Organisasi Aktif
        .post("/pilih-organisasi", OrgController::switch_org)
        // Modul Warga & Rumah (F1.3)
        .get("/anggota", MemberController::index)
        .get("/anggota/baru", MemberController::show_create)
        .post("/anggota/baru", MemberController::create)
        .get("/anggota/impor", MemberController::show_import)
        .post("/anggota/impor", MemberController::import_csv)
        .get("/anggota/:id", MemberController::show_edit)
        .post("/anggota/:id", MemberController::update)
        .post("/anggota/:id/status", MemberController::toggle_status)
        // Modul Iuran & Matriks 12 Bulan (F1.4)
        .get("/iuran", FeeController::index)
        .post("/iuran/sinkronkan", FeeController::sync_invoices_action)
        .get("/iuran/tarif", FeeController::tariffs)
        .post("/iuran/tarif", FeeController::store_tariff)
        .get("/iuran/generate", FeeController::generate_view)
        .post("/iuran/generate", FeeController::generate_action)
        .get("/iuran/bayar/:id", FeeController::pay_view)
        .post("/iuran/bayar/:id", FeeController::pay_action)
        // Modul Kas, Pembukuan & Generator LPJ (F1.5)
        .get("/kas", CashController::index)
        .get("/kas/masuk", CashController::show_in)
        .post("/kas/masuk", CashController::store_in)
        .get("/kas/keluar", CashController::show_out)
        .post("/kas/keluar", CashController::store_out)
        .get("/kas/donasi-barang", CashController::show_inkind)
        .post("/kas/donasi-barang", CashController::store_inkind)
        .get("/laporan", CashController::index)
        .get("/laporan/lpj/:year", CashController::lpj_preview)
        .get("/laporan/lpj/:year/docx", CashController::export_docx)
        // Modul Kegiatan & Agenda Warga (F1.6)
        .get("/kegiatan", EventController::index)
        .get("/kegiatan/baru", EventController::show_create)
        .post("/kegiatan/baru", EventController::create)
        .get("/kegiatan/:id", EventController::show_detail)
        .get("/kegiatan/:id/edit", EventController::show_edit)
        .post("/kegiatan/:id", EventController::update)
        .post("/kegiatan/:id/status", EventController::update_status)
        .post("/kegiatan/:id/rsvp", EventController::store_rsvp)
        // Modul Papan Informasi & Pengumuman (F1.6)
        .get("/info", AnnouncementController::index)
        .get("/info/baru", AnnouncementController::show_create)
        .post("/info/baru", AnnouncementController::create)
        .get("/info/:id/edit", AnnouncementController::show_edit)
        .post("/info/:id", AnnouncementController::update)
        .post("/info/:id/pin", AnnouncementController::toggle_pin)
        .post("/info/:id/hapus", AnnouncementController::delete)
        // Modul Surat & Undangan Warga (F1.7)
        .get("/surat", LetterController::index)
        .get("/surat/baru", LetterController::show_create)
        .post("/surat/baru", LetterController::create)
        .get("/surat/template", LetterController::templates)
        .post("/surat/template", LetterController::store_template)
        .get("/surat/:id", LetterController::show_detail)
        .get("/surat/:id/edit", LetterController::show_edit)
        .post("/surat/:id", LetterController::update)
        .post("/surat/:id/kirim", LetterController::toggle_sent)
        .post("/surat/:id/hapus", LetterController::delete)
        .get("/surat/:id/pdf", LetterController::export_pdf)
        // Modul Pengaturan Organisasi & Profil Lingkungan
        .get("/pengaturan", OrgController::settings)
        .post("/pengaturan", OrgController::update_settings)
        // Modul Superadmin (F1.2)
        .get("/sa/organisasi", SuperadminController::list_orgs)
        .get("/sa/organisasi/baru", SuperadminController::show_create_org)
        .post("/sa/organisasi/baru", SuperadminController::create_org)
        .post(
            "/sa/organisasi/:id/status",
            SuperadminController::update_status,
        )
        .post("/sa/impersonasi/:id", SuperadminController::impersonate)
        .get("/sa/pengguna", SuperadminController::list_users)
        .get("/sa/log", SuperadminController::list_audit_logs)
        // Halaman Error (F1.8)
        .get("/error/403", ErrorController::forbidden)
        .get("/error/500", ErrorController::server_error)
        // Static Asset Serving
        .get("/css/tokens.css", serve_tokens_css)
        .get("/static/css/tokens.css", serve_tokens_css)
        .get("/css/components.css", serve_components_css)
        .get("/static/css/components.css", serve_components_css)
        // Fallback 404 Not Found
        .fallback(ErrorController::not_found)
}

async fn serve_tokens_css() -> impl IntoResponse {
    (
        [(axum::http::header::CONTENT_TYPE, "text/css; charset=utf-8")],
        include_str!("../../public/css/tokens.css"),
    )
}

async fn serve_components_css() -> impl IntoResponse {
    (
        [(axum::http::header::CONTENT_TYPE, "text/css; charset=utf-8")],
        include_str!("../../public/css/components.css"),
    )
}
