use axum::{
    routing::{get, post},
    Router,
};

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/",get(home))
    .route("/admin", post(admin_input))
    .route("/user", post(user_input));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listenin on http://{}", addr);
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await.unwrap();
}

async fn home() -> &'static str {
    "Welcome to the Wordle Replica"
}
async fn admin_input() -> &'static str {
    "Admin Results"
}
async fn user_input() -> &'static str {
    "User Results"
}