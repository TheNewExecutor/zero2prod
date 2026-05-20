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
        request_id = %Uuid::new_v4(),
        user.name = %payload.name,
        user.email = %payload.email,
    )
)]
pub async fn subscribe(
    State(state): State<AppState>,
    Form(payload): Form<FormData>,
) -> impl IntoResponse {
    let query_span = tracing::info_span!(
        "Saving new subscriber details in the database"
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
    .instrument(query_span)
    .await;
    
    match result {
        Ok(_) => {
            tracing::info!("New subscriber details have been saved.");
            (StatusCode::OK, "OK").into_response()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}.", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert subscription into the database",
            )
                .into_response()
        }
    }
}
