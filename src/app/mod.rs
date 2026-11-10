pub mod controllers;
pub mod models;
pub mod requests;
pub mod services;

/// Application Prelude Hub — Satu baris import untuk seluruh controller & service:
/// `use crate::app::prelude::*;`
#[allow(unused_imports, ambiguous_glob_reexports)]
pub mod prelude {
    pub use crate::app::models::*;
    pub use crate::app::requests::*;
    pub use lumina::prelude::*;
    // Disambiguasi simbol `Transaction` antara lumina::prelude (DB ACID tx) dan model bisnis kas
    pub use crate::app::models::Transaction;
}
