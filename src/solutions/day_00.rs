use axum::{http::StatusCode, response::IntoResponse};

//shuttle.rs-cch23--1
pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

//shuttle.rs-cch23--1
pub async fn return_500() -> impl IntoResponse {
    return StatusCode::INTERNAL_SERVER_ERROR;
}
