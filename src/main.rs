//! src/main.rs
use axum;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
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

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duraction::from_secs(2))
        .connect_lazy(&configuration.database.connection_string())
        .expect("Failed to connect to Postgres.");
    let state = AppState {
        db_pool: connection_pool,
    };
    // We have removed the hard-coded port number and it now comes from our settings
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app(state)).await.unwrap();
}
