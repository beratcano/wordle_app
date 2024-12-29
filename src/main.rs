#[allow(unused_imports)]
use axum::{
    routing::{get, post,get_service},
    Router,ServiceExt, Server
};
use routes::{
    auth::auth_routes,
    admin::SharedAdminResult,
};
use std::{
    sync:: {Arc, Mutex},
    net:: SocketAddr,
    collections::HashMap,
};
use tower_http::services::ServeDir;
use dotenv::dotenv;

mod routes;
mod models;
mod db;

type SharedUsers = Arc<Mutex<HashMap<String,String>>>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_pool = db::establish_connection()
        .await
        .expect("Failed to connect to DB.");
    db::initalize_db(&db_pool)
        .await
        .expect("Failed to initialize DB.");

    let admin_result: SharedAdminResult = Arc::new(Mutex::new(None));
    let static_files = ServeDir::new("static");

    let admin_routes = routes::admin::admin_routes(admin_result.clone());
    let user_routes = routes::user::user_routes(admin_result.clone());
    let static_routes = Router::new().nest_service("/static", get_service(static_files));
 
    let app = Router::new()
        .route("/", get(routes::home::home))
        .merge(admin_routes)
        .merge(user_routes)
        .merge(static_routes)
        .merge(auth_routes(db_pool.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listenin on http://{}", addr);
    Server::bind(&addr)
    .serve(app.into_make_service()) 
    .await.unwrap(); 
}