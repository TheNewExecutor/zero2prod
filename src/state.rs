// src/state.rs
use sqlx::PgPool;
use std::sync:: Arc;

pub struct AppState {
    pub db_pool: PgPool,
}

// A type alias for shorthand in handlers
pub type SharedState = Arc<AppState>;
