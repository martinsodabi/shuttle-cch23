mod solutions;

use axum::{
    routing::{get, post},
    Router,
};

use crate::solutions::{
    day_00::{hello_world, return_500},
    day_01::recalibrate_packet_ids,
    day_04::{calculate_reindeer_strength, reindeer_contest},
    day_06::get_elf_count,
    day_07::{bake_cookie, decode_cookie_recipe},
    day_08::{calculate_drop_momentum, get_pokemon_weight_in_kg},
};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(return_500))
        .route("/1/*packets", get(recalibrate_packet_ids))
        .route("/4/strength", post(calculate_reindeer_strength))
        .route("/4/contest", post(reindeer_contest))
        .route("/6", post(get_elf_count))
        .route("/7/decode", get(decode_cookie_recipe))
        .route("/7/bake", get(bake_cookie))
        .route("/8/weight/:pokedex_number", get(get_pokemon_weight_in_kg))
        .route("/8/drop/:pokedex_number", get(calculate_drop_momentum));

    return Ok(router.into());
}
