use axum::{
    routing::get,
    Router,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(|| async { "Hello, World!"}))
    .route("/health_check", get(|| async { "OK" }))
    .route("/health", get(health_check))
    .route("/complex_health", get(complex_health_check))
    .route("/trait_health", get(trait_health_check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}

async fn trait_health_check() -> impl IntoResponse {
    (StatusCode::OK, "Trait OK").into_response()
}

#[derive(Serialize)]
struct HealthResponse {
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