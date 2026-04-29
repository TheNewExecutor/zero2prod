//! src/main.rs
use tokio::net::TcpListener;
use zero2prod::startup::app;
use zero2prod::configuration::get_configuration;
use axum;

#[tokio::main]
async fn main() {
    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    // We have removed the hard-coded port number and it now comes from our settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)
        .await
        .unwrap();
    axum::serve(listener, app())
     .await
     .unwrap();
}

