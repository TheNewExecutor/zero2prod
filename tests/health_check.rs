//! tests/health_check.rs
use sqlx::{PgConnection, Connection};
use zero2prod::configuration::get_configuration;
use std::net::TcpListener;
use serde::Deserialize;
use zero2prod::startup::app;

// Define a test struct that matches the HealthResponse struct
#[derive(Debug, Deserialize, PartialEq)]
struct TestHealthResponse {
    status: String,
    version: String,
}


#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    // We need to bring in `request` to perform http requests against our application
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("http://{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let body = response.text().await.expect("Failed to read response body.");
    assert_eq!(body, "OK");
}

#[tokio::test]
async fn trait_health_check_works() {
    // Arrange
    let address = spawn_app();
    // We need to bring in `request` to perform http requests against our application
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("http://{}/trait_health", address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let body = response.text().await.expect("Failed to read response body.");
    assert_eq!(body, "Trait OK");
}

#[tokio::test]
async fn complex_health_check_works() {
    // Arrange
    let address = spawn_app();
    // We need to bring in `request` to perform http requests against our application
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("http://{}/complex_health", address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let body: TestHealthResponse = response
        .json()
        .await
        .expect("Failed to read response body.");
    let expected_body = TestHealthResponse {
        status: "up".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    assert_eq!(body, expected_body);
}


#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_string = configuration.database.connection_string();
    // The  `Connection` trait must be in scope for us to invoke
    // `PgConnection::connect` - it is not an inherent method of the struct!
    let connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("http://{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());
}


#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("http://{}/subscriptions", app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(422,
        response.status().as_u16(),
        // Additional customised error message on test failure
        "The API did not fail with 400 Bad Request when the payload was {}.",
        error_message
        );
    }
}
// Helper function to launch our application in the background 
fn spawn_app() -> String {
    let std_listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = std_listener.local_addr().unwrap().port();
    std_listener
        .set_nonblocking(true)
        .expect("Failed to set listener to non-blocking mode");
    let listener =
        tokio::net::TcpListener::from_std(std_listener).expect("Failed to convert listener");

    let server_router = app();

    tokio::spawn(async move {
        axum::serve(listener, server_router).await.unwrap();
    });

    format!("127.0.0.1:{}", port)
}