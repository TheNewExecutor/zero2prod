//! tests/health_check.rs
use serde::Deserialize;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::app;
use zero2prod::state::AppState;
// Define a test struct that matches the HealthResponse struct
#[derive(Debug, Deserialize, PartialEq)]
struct TestHealthResponse {
    status: String,
    version: String,
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    // We need to bring in `request` to perform http requests against our application
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("http://{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let body = response
        .text()
        .await
        .expect("Failed to read response body.");
    assert_eq!(body, "OK");
}

#[tokio::test]
async fn trait_health_check_works() {
    // Arrange
    let app = spawn_app().await;
    // We need to bring in `request` to perform http requests against our application
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("http://{}/trait_health", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    let body = response
        .text()
        .await
        .expect("Failed to read response body.");
    assert_eq!(body, "Trait OK");
}

#[tokio::test]
async fn complex_health_check_works() {
    // Arrange
    let app = spawn_app().await;
    // We need to bring in `request` to perform http requests against our application
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("http://{}/complex_health", app.address))
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
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("http://{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("http://{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            422,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

// Test struct to hold essential setup info to run tests
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}
// Helper function to launch our application in the background
async fn spawn_app() -> TestApp {
    let std_listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = std_listener.local_addr().unwrap().port();
    std_listener
        .set_nonblocking(true)
        .expect("Failed to set listener to non-blocking mode");
    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let listener =
        tokio::net::TcpListener::from_std(std_listener).expect("Failed to convert listener");
    let connection_pool = configure_database(&configuration.database).await;

    let state = AppState {
        db_pool: connection_pool.clone(),
    };
    let server_router = app(state);

    tokio::spawn(async move {
        axum::serve(listener, server_router).await.unwrap();
    });

    let address = format!("127.0.0.1:{}", port);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database.");

    connection_pool
}
