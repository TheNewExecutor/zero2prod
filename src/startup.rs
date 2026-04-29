use axum::{
    routing::{get, post},
    Router,
};
use crate::routes::{health_check, complex_health_check, trait_health_check, subscribe};

pub fn app() -> Router {
    Router::new()
    .route("/", get(|| async { "Hello, World!"}))
    .route("/health_check", get(|| async { "OK" }))
    .route("/health", get(health_check))
    .route("/complex_health", get(complex_health_check))
    .route("/trait_health", get(trait_health_check))
    .route("/subscriptions", post(subscribe))
}