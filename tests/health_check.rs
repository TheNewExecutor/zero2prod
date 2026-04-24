//! tests/health_check.rs
use std::net::TcpListener;
use serde::Deserialize;
use zero2prod::app;

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