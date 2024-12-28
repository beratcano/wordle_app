#[allow(unused_imports)]
use axum::{
    routing::{get, post,get_service},
    Router,ServiceExt, Server
};
use std::sync::{Arc, Mutex};
use crate::routes::admin::SharedAdminResult;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod routes;
mod models;

#[tokio::main]
async fn main() {
    let admin_result: SharedAdminResult = Arc::new(Mutex::new(None));
    let static_files = ServeDir::new("static");

    let admin_routes = routes::admin::admin_routes(admin_result.clone());
    let user_routes = routes::user::user_routes(admin_result.clone());
    let static_routes = Router::new().nest_service("/static", get_service(static_files));
 
    let app = Router::new()
        .route("/", get(routes::home::home))
        .merge(admin_routes)
        .merge(user_routes)
        .merge(static_routes);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listenin on http://{}", addr);
    Server::bind(&addr)
    .serve(app.into_make_service()) 
    .await.unwrap(); 

}