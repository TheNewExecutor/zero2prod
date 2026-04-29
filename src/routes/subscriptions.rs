use axum::{
    http::StatusCode,
    response::IntoResponse,
    Form,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(Form(payload): Form<FormData>) -> impl IntoResponse {
    println!("Registering new subscriber {}, with email {}.", payload.name, payload.email);
    (StatusCode::OK, "OK").into_response()
}