use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

//shuttle.rs-cch23-06-task_1_and_bonus_task
pub async fn get_elf_count(body: String) -> Result<Json<Value>, StatusCode> {
    let elf_count: u32 = body
        .split(" ")
        .fold(0, |acc, e| if e.contains("elf") { acc + 1 } else { acc });

    let elf_on_shelf_count: u32 = body.matches("elf on a shelf").count() as u32;
    let shelf_no_elf_count: u32 = body.matches("shelf").count() as u32 - elf_on_shelf_count;

    if elf_on_shelf_count == 0 {
        return Ok(Json(json!({
            "elf": elf_count,
        })));
    }

    return Ok(Json(json!({
        "elf": elf_count,
        "elf on a shelf": elf_on_shelf_count,
        "shelf with no elf on it": shelf_no_elf_count,
    })));
}
