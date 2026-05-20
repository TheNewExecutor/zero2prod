//! src/main.rs
use axum;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::app;
use zero2prod::state::AppState;

#[tokio::main]
async fn main() {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Set up logging subscriber (filtering, formatting and global init)
    LogTracer::init().expect("Failed to set logger.");
    let env_filter = EnvFilter::new(&configuration.log_level);
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber.");

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
