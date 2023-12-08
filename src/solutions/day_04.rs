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
        .reduce(|acc, e| acc + e)
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
