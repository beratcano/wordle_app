use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router
};
use std::sync::{Arc, Mutex};
use crate::models::AdminResult;

pub type SharedAdminResult = Arc<Mutex<Option<AdminResult>>>;

pub fn admin_routes(admin_result: SharedAdminResult) -> Router {

    Router::new()
        .route("/admin", get(admin_input))
        .route("/admin/input", post(submit_admin_result))
        .with_state(admin_result)
}

async fn admin_input(State(state): State<SharedAdminResult>) -> impl IntoResponse {
    let admin_result = state.lock().unwrap();
    if let Some(result) = &*admin_result {
        Json(result.clone())
    } else {
        Json(AdminResult { result: 7 })
    }
}

async fn submit_admin_result(
    State(state): State<SharedAdminResult>,
    Json(payload): Json<AdminResult>,
) -> impl IntoResponse {
    let mut admin_result = state.lock().unwrap();
    *admin_result = Some(payload);
    (StatusCode::OK, Json(AdminResult{
        result: 7
    }))
}