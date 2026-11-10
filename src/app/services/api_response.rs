use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;
use serde_json::json;

pub struct ApiResponse;

#[allow(dead_code)]
impl ApiResponse {
    /// Format standar respons sukses data API
    pub fn success<T: Serialize>(data: T) -> Response {
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": data,
                "code": 200
            })),
        )
            .into_response()
    }

    /// Format standar respons sukses dengan pesan
    pub fn success_msg(message: &str) -> Response {
        (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": message,
                "code": 200
            })),
        )
            .into_response()
    }

    /// Format standar respons error API
    pub fn error(status: StatusCode, message: &str) -> Response {
        (
            status,
            Json(json!({
                "success": false,
                "error": message,
                "code": status.as_u16()
            })),
        )
            .into_response()
    }
}
