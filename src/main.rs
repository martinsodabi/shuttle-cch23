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
        .route("/7/bake", get(bake_cookie));

    return Ok(router.into());
}
