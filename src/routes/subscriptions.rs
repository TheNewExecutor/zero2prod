//! src/routes/subscriptions.rs
use crate::state::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    State(state): State<AppState>,
    Form(payload): Form<FormData>,
) -> impl IntoResponse {
    println!(
        "Registering new subscriber {}, with email {}.",
        payload.name, payload.email
    );
    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        payload.email,
        payload.name,
        Utc::now(),
    )
    .execute(&state.db_pool)
    .await;
    match result {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to insert subscription into the database",
        )
            .into_response(),
    }
}
