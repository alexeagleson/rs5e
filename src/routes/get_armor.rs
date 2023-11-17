use crate::AppState;
use axum::{extract::State, http::StatusCode, Json};

pub(crate) async fn get_armor(State(state): State<AppState>) -> (StatusCode, Json<String>) {
    let armor_list = state.armor_model_map.values().collect::<Vec<_>>();
    let armor_list_response = serde_json::to_string(&armor_list).unwrap();

    (StatusCode::OK, axum::Json(armor_list_response))
}
