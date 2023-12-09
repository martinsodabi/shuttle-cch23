use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str};

//shuttle.rs-cch23-07-task_1
// #[debug_handler] //Can't use debug_handler when HeaderMap is argument
pub async fn decode_cookie_recipe(headers: HeaderMap) -> Result<impl IntoResponse, StatusCode> {
    let encoded_recipe: String = headers
        .get("Cookie")
        .unwrap()
        .to_str()
        .unwrap()
        .replace("recipe=", "");

    let decoded_bytes: Vec<u8> = match general_purpose::STANDARD.decode(&encoded_recipe) {
        Ok(value) => value,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let decoded_recipe = match str::from_utf8(&decoded_bytes) {
        Ok(value) => value,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    return Ok(decoded_recipe.to_string());
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RecipeAndPantry {
    recipe: HashMap<String, u32>,
    pantry: HashMap<String, u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NumOfCookiesAndRemainingIngredients {
    cookies: u32,
    pantry: HashMap<String, u32>,
}

//shuttle.rs-cch23-07-task_2_and_3(bonus_tasks)
pub async fn bake_cookie(headers: HeaderMap) -> Result<impl IntoResponse, StatusCode> {
    let encoded_recipe: String = headers
        .get("Cookie")
        .unwrap()
        .to_str()
        .unwrap()
        .replace("recipe=", "");

    let decoded_bytes: Vec<u8> = match general_purpose::STANDARD.decode(&encoded_recipe) {
        Ok(value) => value,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let recipe_and_pantry: RecipeAndPantry = match serde_json::from_slice(&decoded_bytes) {
        Ok(value) => value,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let recipe = recipe_and_pantry.recipe;
    let pantry = recipe_and_pantry.pantry;

    let mut flour_ration: u32 = 0;

    if recipe.get("flour").unwrap_or(&0u32) > &0u32 {
        flour_ration = pantry.get("flour").unwrap_or(&0u32) / recipe.get("flour").unwrap_or(&0u32);
    }

    let mut remaining_ingredients = HashMap::new();

    for ingredient in pantry {
        let value = ingredient.1 - recipe.get(&ingredient.0).unwrap_or(&0u32) * flour_ration;
        remaining_ingredients.insert(ingredient.0, value);
    }

    let response_output = NumOfCookiesAndRemainingIngredients {
        cookies: flour_ration,
        pantry: remaining_ingredients,
    };

    return Ok(Json(response_output));
}
