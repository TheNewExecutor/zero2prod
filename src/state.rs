//! src/state.rs This module contains the shared state between routes
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}
