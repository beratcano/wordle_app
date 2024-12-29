use axum::{
    extract:: {Json, State}, 
    http:: StatusCode, 
    response:: IntoResponse,
    Router
};
use sqlx::SqlitePool;
use bcrypt::{
    hash,
    verify,
};
use crate::models::User;

pub fn auth_routes(db_pool: SqlitePool) -> Router {
    Router::new()
    .route("/register", axum::routing::post(register))
    .route("/login", axum::routing::post(login))
    .with_state(db_pool)
}

async fn register(
    State(pool): State<SqlitePool>,
    Json(payload): Json<User>,
) -> impl IntoResponse {

    let hashed_password = match hash(&payload.password, bcrypt::DEFAULT_COST) {
        Ok(hashed) => hashed,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password.").into_response()
    };
    

    let result = sqlx::query!(
        "INSERT INTO users (username, password) VALUES (?, ?)",
        payload.username,
        hashed_password
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, "Registration succesfull.").into_response(),
        Err(_) => (StatusCode::BAD_REQUEST, "Username already exists.").into_response()
    }
}

async fn login(
    State(pool): State<SqlitePool>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    let row = sqlx::query!(
        "SELECT password FROM users WHERE username == ?",
        payload.username
    )
    .fetch_one(&pool)
    .await;

    match row {
        Ok(record) => {
            if verify(&payload.password, &record.password).unwrap_or(false) {
                (StatusCode::OK, "Login successfull").into_response()
            } else {
                (StatusCode::UNAUTHORIZED, "Invalid username or password").into_response()
            }
        }
        Err(_) => (StatusCode::UNAUTHORIZED, "Invalid username or password").into_response()
    }
}