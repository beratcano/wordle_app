use axum::response::{IntoResponse, Html};

pub async fn home() -> impl IntoResponse {
    Html(include_str!("../static/index.html"))
}