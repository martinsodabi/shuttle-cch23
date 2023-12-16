use axum::http::StatusCode;

//shuttle.rs-cch23--1
pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

//shuttle.rs-cch23--1
pub async fn return_500() -> StatusCode {
    return StatusCode::INTERNAL_SERVER_ERROR;
}
