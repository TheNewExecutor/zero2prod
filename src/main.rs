//! src/main.rs
use axum;
use sqlx::PgPool;
use tokio::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::app;
use zero2prod::state::AppState;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Set up logging subscriber (filtering, formatting and global init)
    let subscriber = get_subscriber("zero2prod".into(), configuration.log_level, std::io::stdout);
    init_subscriber(subscriber);

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let state = AppState {
        db_pool: connection_pool,
    };
    // We have removed the hard-coded port number and it now comes from our settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app(state)).await.unwrap();
}
