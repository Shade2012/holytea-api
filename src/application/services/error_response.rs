use axum::{http::StatusCode, Json};
use reqwest::StatusCode as ReqwestStatusCode;
use serde_json::json;

pub fn error_response(message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({
            "status": "error",
            "message": message,
        })),
    )
}

pub fn error_response_reqwest(message: &str) -> (ReqwestStatusCode, Json<serde_json::Value>) {
    (
        ReqwestStatusCode::BAD_REQUEST,
        Json(json!({
            "status": "error",
            "message": message,
        })),
    )
}