use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
//shuttle.rs-cch23--1
async fn hello_world() -> &'static str {
    "Hello, world!"
}

//shuttle.rs-cch23--1
async fn return_500() -> impl IntoResponse {
    return StatusCode::INTERNAL_SERVER_ERROR;
}

//shuttle.rs-cch23-01
async fn recalibrate_packet_ids(
    Path(packets): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let packet_vec: Vec<&str> = packets.split("/").collect();

    if packet_vec.len() > 20 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut parsed_packet_vec: Vec<i32> = vec![];

    for packet in packet_vec {
        match packet.parse::<i32>() {
            Err(_) => {
                return Err(StatusCode::BAD_REQUEST);
            }
            Ok(value) => {
                parsed_packet_vec.push(value);
            }
        }
    }

    let sled_id: i32 = parsed_packet_vec
        .into_iter()
        .reduce(|acc, e| acc ^ e)
        .unwrap()
        .pow(3);

    return Ok(sled_id.to_string());
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(return_500))
        .route("/1/*packets", get(recalibrate_packet_ids));

    return Ok(router.into());
}
