//! src/main.rs
use tokio::net::TcpListener;
use std::sync::Arc;
use zero2prod::startup::app;
use zero2prod::configuration::get_configuration;
use zero2prod::state::AppState;
use axum;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
    .await
    .expect("Failed to connect to Postgres.");
    let state = Arc::new( AppState { db_pool: connection_pool } );
    // We have removed the hard-coded port number and it now comes from our settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)
        .await
        .unwrap();
    axum::serve(listener, app(state))
     .await
     .unwrap();
}

