//! src/startup.rs Responsible for building web application, including route definitions
use axum::{
    routing::{get, post},
    Router,
};
use crate::state::AppState; // shared state between routes with Arc<AppState>
use crate::routes::{health_check, complex_health_check, trait_health_check, subscribe};



pub fn app(state: AppState) -> Router {
    Router::new()
    .route("/", get(|| async { "Hello, World!"}))
    .route("/health_check", get(|| async { "OK" }))
    .route("/health", get(health_check))
    .route("/complex_health", get(complex_health_check))
    .route("/trait_health", get(trait_health_check))
    .route("/subscriptions", post(subscribe))
    .with_state(state) // delegate type specifics to state.rs module
}