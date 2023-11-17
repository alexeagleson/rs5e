use axum::{extract::State, http::StatusCode, Json};
use crate::AppState;

pub(crate) async fn get_weapons(
    // access the state via the `State` extractor
    // extracting a state of the wrong type results in a compile error
    State(state): State<AppState>,
) -> (StatusCode, Json<String>) {
    let weapon_list = state.weapon_model_map.values().collect::<Vec<_>>();
    let weapons_list_response = serde_json::to_string(&weapon_list).unwrap();

    (StatusCode::OK, axum::Json(weapons_list_response))
}
