use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str};

//shuttle.rs-cch23-07-task_1
// #[debug_handler] //Can't use debug_handler when HeaderMap is argument
pub async fn decode_cookie_recipe(headers: HeaderMap) -> Result<String, StatusCode> {
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
    recipe: HashMap<String, i32>,
    pantry: HashMap<String, i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NumOfCookiesAndRemainingIngredients {
    cookies: i32,
    pantry: HashMap<String, i32>,
}

//shuttle.rs-cch23-07-task_2_and_3(bonus_tasks)
pub async fn bake_cookie(
    headers: HeaderMap,
) -> Result<Json<NumOfCookiesAndRemainingIngredients>, StatusCode> {
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

    // println!("Recipe {:#?}", recipe);
    // println!("Pantry {:#?}", pantry);

    let num_of_cookies: i32 = recipe
        .iter()
        .map(|(k, &v)| {
            if v == 0 {
                i32::MAX
            } else {
                pantry.get(k).map(|&p_v| p_v / v).unwrap_or_default()
            }
        })
        .min()
        .unwrap_or_default();

    let remaining_ingredients: HashMap<String, i32> = pantry
        .iter()
        .map(|(k, &v)| {
            (
                k.to_owned(),
                v - (recipe.get(k).unwrap_or(&0) * num_of_cookies),
            )
        })
        .collect();

    let response_output = NumOfCookiesAndRemainingIngredients {
        cookies: num_of_cookies,
        pantry: remaining_ingredients,
    };

    // println!("{:#?}", response_output);

    return Ok(Json(response_output));
}
