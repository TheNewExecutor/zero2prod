//! lib.rs

use axum::{
    Form,
    routing::{get, post},
    Router,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Serialize, Deserialize};

pub fn app() -> Router {
    Router::new()
    .route("/", get(|| async { "Hello, World!"}))
    .route("/health_check", get(|| async { "OK" }))
    .route("/health", get(health_check))
    .route("/complex_health", get(complex_health_check))
    .route("/trait_health", get(trait_health_check))
    .route("/subscriptions", post(subscribe))
}


async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}

async fn trait_health_check() -> impl IntoResponse {
    (StatusCode::OK, "Trait OK").into_response()
}

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    version: String,
}

async fn complex_health_check() -> impl IntoResponse {
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

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(Form(payload): Form<FormData>) -> impl IntoResponse {
    println!("Registering new subscriber {}, with email {}.", payload.name, payload.email);
    (StatusCode::OK, "OK").into_response()
}