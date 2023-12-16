use axum::extract::Path;
use axum::http::StatusCode;
use serde::Deserialize;

pub async fn get_pokemon_weight_in_kg(
    Path(pokedex_number): Path<u32>,
) -> Result<String, StatusCode> {
    let resp = reqwest::get(format!(
        "https://pokeapi.co/api/v2/pokemon/{pokedex_number}"
    ))
    .await
    .map_err(|err| {
        dbg!(err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    })?
    .json::<PokemonWeight>()
    .await
    .map_err(|err| {
        dbg!(err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    })?;
    // println!("{:#?}", resp);
    return Ok(format!("{}", resp.weight / 10.0));
}

#[derive(Deserialize)]
struct PokemonWeight {
    weight: f32,
}

pub async fn calculate_drop_momentum(
    Path(pokedex_number): Path<u32>,
) -> Result<String, StatusCode> {
    const VELOCITY_AFTER_10M: f32 = 14.0178457689;

    let weight = get_pokemon_weight_in_kg(Path(pokedex_number))
        .await
        .map_err(|err| {
            dbg!(err);
            return err;
        })?
        .parse::<f32>()
        .map_err(|err| {
            dbg!(err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        })?;

    let drop_momentum: f32 = weight * VELOCITY_AFTER_10M;

    return Ok(drop_momentum.to_string());
}
