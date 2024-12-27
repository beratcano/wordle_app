use axum::{
    extract::{Json, State},
    routing::post, 
    Router,
    response::IntoResponse
};
use crate::models::UserGuess;
use crate::routes::admin::SharedAdminResult;
use tower_http::services::ServeDir;

let static_files: ServeDir = tower_http::services::ServeDir::new("static");

pub fn static_routes(admin_result: SharedAdminResult) -> Router {
    Router::new()
    .route("/static/*file", get(static_files))
    .with_state(admin_result)
}