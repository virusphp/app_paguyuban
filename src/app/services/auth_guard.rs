use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Redirect, Response},
};
use lumina::prelude::Request;
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user_id: Uuid,
    pub user_name: String,
    pub is_superadmin: bool,
    pub current_org_id: Option<Uuid>,
    pub current_org_name: Option<String>,
    pub current_role: Option<String>,
    pub is_api: bool,
}

#[allow(dead_code)]
#[allow(clippy::result_large_err)]
impl AuthContext {
    /// Ekstrak sesi atau token otorisasi pengguna dari Request (Mendukung JWT Bearer & Sesi Web)
    pub async fn from_request(req: &Request) -> Result<Self, Response> {
        let is_api = req.uri.path().starts_with("/api/");

        // 1. Prioritas 1: Header Authorization: Bearer <token> (Mobile PWA & REST API)
        if let Some(auth_val) = req
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
        {
            if let Some(token) = auth_val.strip_prefix("Bearer ") {
                let token = token.trim();

                // Cek pembatalan token (logout blacklist)
                if lumina::core::auth::TokenBlacklist::is_revoked(token).await {
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "success": false,
                            "error": "Token otorisasi telah dibatalkan (revoked/logout)."
                        })),
                    )
                        .into_response());
                }

                match lumina::http::auth::validate_token(token) {
                    Ok(user) => {
                        if let Ok(user_id) = Uuid::parse_str(&user.sub) {
                            let mut current_org_id = None;
                            let mut current_org_name = None;

                            for perm in &user.permissions {
                                if let Some(id_str) = perm.strip_prefix("org_id:") {
                                    current_org_id = Uuid::parse_str(id_str).ok();
                                } else if let Some(name) = perm.strip_prefix("org_name:") {
                                    current_org_name = Some(name.to_string());
                                }
                            }

                            // Header X-Org-Id override jika klien mobile berganti konteks organisasi
                            if let Some(x_org) =
                                req.headers.get("x-org-id").and_then(|v| v.to_str().ok())
                            {
                                if let Ok(override_id) = Uuid::parse_str(x_org.trim()) {
                                    current_org_id = Some(override_id);
                                }
                            }

                            let is_superadmin = user.role == "superadmin"
                                || user.permissions.iter().any(|p| p == "is_superadmin:true");

                            let current_role = if user.role.is_empty() {
                                None
                            } else {
                                Some(user.role.clone())
                            };

                            return Ok(Self {
                                user_id,
                                user_name: user.name.clone(),
                                is_superadmin,
                                current_org_id,
                                current_org_name,
                                current_role,
                                is_api: true,
                            });
                        }
                    }
                    Err(e) => {
                        return Err((
                            StatusCode::UNAUTHORIZED,
                            Json(json!({
                                "success": false,
                                "error": format!("Token Bearer tidak valid atau telah kedaluwarsa: {}", e)
                            })),
                        )
                            .into_response());
                    }
                }
            }
        }

        // 2. Prioritas 2: User yang disuntikkan via middleware ekstensi
        if let Some(ref user) = req.user {
            if let Ok(user_id) = Uuid::parse_str(&user.sub) {
                let mut current_org_id = None;
                let mut current_org_name = None;
                for perm in &user.permissions {
                    if let Some(id_str) = perm.strip_prefix("org_id:") {
                        current_org_id = Uuid::parse_str(id_str).ok();
                    } else if let Some(name) = perm.strip_prefix("org_name:") {
                        current_org_name = Some(name.to_string());
                    }
                }

                let is_superadmin = user.role == "superadmin"
                    || user.permissions.iter().any(|p| p == "is_superadmin:true");

                let current_role = if user.role.is_empty() {
                    None
                } else {
                    Some(user.role.clone())
                };

                return Ok(Self {
                    user_id,
                    user_name: user.name.clone(),
                    is_superadmin,
                    current_org_id,
                    current_org_name,
                    current_role,
                    is_api,
                });
            }
        }

        // 3. Prioritas 3: Sesi Cookie Web
        let user_id_str = match req.session.get::<String>("user_id").await {
            Ok(Some(id)) => id,
            _ => {
                if is_api {
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "success": false,
                            "error": "Sesi tidak valid atau belum login"
                        })),
                    )
                        .into_response());
                } else {
                    return Err(Redirect::to("/login").into_response());
                }
            }
        };

        let user_id = match Uuid::parse_str(&user_id_str) {
            Ok(u) => u,
            Err(_) => {
                if is_api {
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "success": false,
                            "error": "Identitas pengguna tidak valid"
                        })),
                    )
                        .into_response());
                } else {
                    return Err(Redirect::to("/login").into_response());
                }
            }
        };

        let user_name = req
            .session
            .get::<String>("user_name")
            .await
            .unwrap_or(None)
            .unwrap_or_else(|| "Pengguna".to_string());

        let is_superadmin = req
            .session
            .get::<bool>("is_superadmin")
            .await
            .unwrap_or(None)
            .unwrap_or(false);

        let current_org_id = match req.session.get::<String>("current_org_id").await {
            Ok(Some(id_str)) => Uuid::parse_str(&id_str).ok(),
            _ => None,
        };

        let current_org_name = req
            .session
            .get::<String>("current_org_name")
            .await
            .unwrap_or(None);

        let current_role = req
            .session
            .get::<String>("current_role")
            .await
            .unwrap_or(None);

        Ok(Self {
            user_id,
            user_name,
            is_superadmin,
            current_org_id,
            current_org_name,
            current_role,
            is_api,
        })
    }

    /// Validasi bahwa pengguna memiliki organisasi aktif
    pub fn require_org(&self) -> Result<Uuid, Response> {
        match self.current_org_id {
            Some(org_id) => Ok(org_id),
            None => {
                if self.is_api {
                    Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({
                            "success": false,
                            "error": "Organisasi aktif belum dipilih"
                        })),
                    )
                        .into_response())
                } else {
                    Err(Redirect::to("/pilih-organisasi").into_response())
                }
            }
        }
    }

    /// Validasi bahwa pengguna memiliki salah satu dari peran yang diizinkan (admin/bendahara/sekretaris)
    pub fn require_role(&self, allowed_roles: &[&str]) -> Result<(), Response> {
        if self.is_superadmin {
            return Ok(());
        }

        if let Some(ref role) = self.current_role {
            if allowed_roles.contains(&role.as_str()) {
                return Ok(());
            }
        }

        if self.is_api {
            Err((
                StatusCode::FORBIDDEN,
                Json(json!({
                    "success": false,
                    "error": "Akses ditolak: peran tidak memadai"
                })),
            )
                .into_response())
        } else {
            Err(Redirect::to("/dashboard").into_response())
        }
    }

    /// Validasi bahwa pengguna adalah Superadmin (/sa/...)
    pub fn require_superadmin(&self) -> Result<(), Response> {
        if self.is_superadmin {
            Ok(())
        } else if self.is_api {
            Err((
                StatusCode::FORBIDDEN,
                Json(json!({
                    "success": false,
                    "error": "Akses ditolak: wewenang superadmin diperlukan"
                })),
            )
                .into_response())
        } else {
            Err(Redirect::to("/dashboard").into_response())
        }
    }

    /// Label peran yang diformat untuk tampilan UI
    pub fn role_display(&self) -> String {
        if self.is_superadmin {
            "SUPERADMIN".to_string()
        } else {
            self.current_role
                .as_deref()
                .unwrap_or("PENGURUS")
                .to_uppercase()
        }
    }
}
