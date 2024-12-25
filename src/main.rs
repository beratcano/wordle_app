#[allow(unused_imports)]
use axum::{routing::{get, post}, Router};
use std::sync::{Arc, Mutex};
use crate::routes::admin::SharedAdminResult;
use std::net::SocketAddr;
mod routes;
mod models;

#[tokio::main]
async fn main() {
    let admin_result: SharedAdminResult = Arc::new(Mutex::new(None));
    let admin_routes = routes::admin::admin_routes(admin_result.clone());
    let user_routes = routes::user::user_routes(admin_result.clone());
    let app = Router::new()
    .route("/", get(routes::home::home))
    .merge(admin_routes)
    .merge(user_routes);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listenin on http://{}", addr);
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await.unwrap();
}