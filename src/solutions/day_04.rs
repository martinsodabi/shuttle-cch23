use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;

//shuttle.rs-cch23-04-task_1
pub async fn calculate_reindeer_strength(
    Json(body): Json<Vec<BareAttributeReindeer>>,
) -> Result<impl IntoResponse, StatusCode> {
    let reindeers_strength = body
        .iter()
        .map(|reindeer| reindeer.strength)
        .reduce(|acc, e| acc + e) //Can also use sum()
        .unwrap();

    return Ok(reindeers_strength.to_string());
}

#[derive(Deserialize)]
pub struct BareAttributeReindeer {
    name: String,
    strength: i32,
}

//shuttle.rs-cch23-04-bonus_task
pub async fn reindeer_contest(
    Json(body): Json<Vec<FullAttributeReindeer>>,
) -> Result<impl IntoResponse, StatusCode> {
    if body.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut fastest_reindeer = &body[0];
    let mut tallest_reindeer = &body[0];
    let mut archmage_reindeer = &body[0];
    let mut foodie_reindeer = &body[0];

    for reindeer in &body {
        if reindeer.speed > fastest_reindeer.speed {
            fastest_reindeer = reindeer
        }

        if reindeer.height > tallest_reindeer.height {
            tallest_reindeer = reindeer;
        }

        if reindeer.snow_magic_power > archmage_reindeer.snow_magic_power {
            archmage_reindeer = reindeer;
        }

        if reindeer.candies_eaten_yesterday > foodie_reindeer.candies_eaten_yesterday {
            foodie_reindeer = reindeer;
        }
    }

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
pub struct FullAttributeReindeer {
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
