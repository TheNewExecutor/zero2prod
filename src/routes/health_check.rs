use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;

pub async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}

pub async fn trait_health_check() -> impl IntoResponse {
    (StatusCode::OK, "Trait OK").into_response()
}

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
}

pub async fn complex_health_check() -> impl IntoResponse {
    // logic to check dependencies like database, redis, etc.
    let is_healthy = true;

    if is_healthy {
        let body = Json(HealthResponse {
            status: "up".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        });
        (StatusCode::OK, body).into_response()
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "Service Unavailable").into_response()
    }
}