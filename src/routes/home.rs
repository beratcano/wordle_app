use axum::response::{IntoResponse, Html};
use tower_http::services::ServeDir;
use tower_service::Service;

pub async fn home() -> impl IntoResponse {
    Html(include_str!("../static/index.html"))
}