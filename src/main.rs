use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
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

//shuttle.rs-cch23-04-task_1
async fn calculate_reindeer_strength(
    Json(body): Json<Vec<BareAttributeReindeer>>,
) -> Result<impl IntoResponse, StatusCode> {
    let reindeers_strength = body
        .iter()
        .map(|reindeer| reindeer.strength)
        .reduce(|acc, e| acc + e)
        .unwrap();

    return Ok(reindeers_strength.to_string());
}

#[derive(Deserialize)]
struct BareAttributeReindeer {
    name: String,
    strength: i32,
}

//shuttle.rs-cch23-04-bonus_task
async fn reindeer_contest(
    Json(body): Json<Vec<FullAttributeReindeer>>,
) -> Result<impl IntoResponse, StatusCode> {
    if body.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let fastest_reindeer = body
        .iter()
        .reduce(|acc, e| if e.speed > acc.speed { e } else { acc })
        .unwrap();

    let tallest_reindeer = body
        .iter()
        .reduce(|acc, e| if e.height > acc.height { e } else { acc })
        .unwrap();

    let archmage_reindeer = body
        .iter()
        .reduce(|acc, e| {
            if e.snow_magic_power > acc.snow_magic_power {
                e
            } else {
                acc
            }
        })
        .unwrap();

    let foodie_reindeer = body
        .iter()
        .reduce(|acc, e| {
            if e.candies_eaten_yesterday > acc.candies_eaten_yesterday {
                e
            } else {
                acc
            }
        })
        .unwrap();

    let fastest_msg = format!(
        "Speeding past the finish line with a strength of {} is {}",
        fastest_reindeer.strength, fastest_reindeer.name
    );

    let tallest_msg = format!(
        "{} is standing tall with his {} cm wide antlers",
        tallest_reindeer.name, tallest_reindeer.antler_width
    );

    let archmage_msg = format!(
        "{} could blast you away with a snow magic power of {}",
        archmage_reindeer.name, archmage_reindeer.snow_magic_power
    );

    let foodie_msg = format!(
        "{} ate lots of candies, but also some {}",
        foodie_reindeer.name, foodie_reindeer.favorite_food
    );

    return Ok(Json(json!({
        "fastest": fastest_msg,
        "tallest": tallest_msg,
        "magician": archmage_msg,
        "consumer": foodie_msg,
    })));
}

#[derive(Deserialize)]
struct FullAttributeReindeer {
    name: String,
    strength: u32,
    speed: f32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(alias = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u32,
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(return_500))
        .route("/1/*packets", get(recalibrate_packet_ids))
        .route("/4/strength", post(calculate_reindeer_strength))
        .route("/4/contest", post(reindeer_contest));

    return Ok(router.into());
}
