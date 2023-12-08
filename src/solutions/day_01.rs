use axum::{extract::Path, http::StatusCode, response::IntoResponse};

//shuttle.rs-cch23-01-task_1_and_bonus_task
pub async fn recalibrate_packet_ids(
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
