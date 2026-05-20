//! src/routes/subscriptions.rs
use crate::state::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use chrono::Utc;
use serde::Deserialize;
use tracing::instrument;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[instrument(
    name = "subscribe_handler",
    skip_all,
    fields(
        user.name = %payload.name,
        user.email = %payload.email,
    )
)]
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
        Ok(_) => {
            tracing::info!("New subscriber details have been saved.");
            (StatusCode::OK, "OK").into_response()
        }
        Err(_) => {
            tracing::error!("Failed to execute query.");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert subscription into the database",
            )
                .into_response()
        }
    }
}
