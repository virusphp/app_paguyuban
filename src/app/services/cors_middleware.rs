use axum::{
    extract::Request,
    http::{header, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

/// Middleware CORS untuk API Mobile SvelteKit PWA & REST Client
pub async fn cors_middleware(req: Request, next: Next) -> Response {
    let origin = req
        .headers()
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("*")
        .to_string();

    // 1. Tangani preflight OPTIONS request
    if req.method() == Method::OPTIONS {
        let mut response = StatusCode::NO_CONTENT.into_response();
        let headers = response.headers_mut();

        headers.insert(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            header::HeaderValue::from_str(&origin)
                .unwrap_or_else(|_| header::HeaderValue::from_static("*")),
        );
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            header::HeaderValue::from_static("GET, POST, PUT, DELETE, PATCH, OPTIONS"),
        );
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            header::HeaderValue::from_static(
                "Authorization, Content-Type, Accept, X-Requested-With, X-Org-Id",
            ),
        );
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            header::HeaderValue::from_static("true"),
        );
        headers.insert(
            header::ACCESS_CONTROL_MAX_AGE,
            header::HeaderValue::from_static("86400"),
        );

        return response;
    }

    // 2. Jalankan request selanjutnya
    let mut response = next.run(req).await;

    // 3. Tambahkan header CORS ke semua respons aktual
    let headers = response.headers_mut();
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        header::HeaderValue::from_str(&origin)
            .unwrap_or_else(|_| header::HeaderValue::from_static("*")),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        header::HeaderValue::from_static("GET, POST, PUT, DELETE, PATCH, OPTIONS"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        header::HeaderValue::from_static(
            "Authorization, Content-Type, Accept, X-Requested-With, X-Org-Id",
        ),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        header::HeaderValue::from_static("true"),
    );

    response
}
