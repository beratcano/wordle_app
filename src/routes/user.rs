use axum::{
    extract::{Json, State},
    routing::post, 
    Router,
    response::IntoResponse
};
use crate::models::UserGuess;
use crate::routes::admin::SharedAdminResult;

pub fn user_routes(admin_result: SharedAdminResult) -> Router {
    Router::new()
    .route("/user", post(user_input))
    .with_state(admin_result)
}

pub async fn user_input(
    State(state): State<SharedAdminResult>,
    Json(payload): Json<UserGuess>,
) -> impl IntoResponse {
    let admin_result = state.lock().unwrap();
    if let Some(result) = &*admin_result {
        if result.result == 7 {
            Json("Admin result not set yet".to_string())
        } else {
            let feedback = compare_guess(&payload.guess, &result.result);
            Json(feedback)
        }
    }
    else {
        Json("Admin result not set yet.".to_string())
    }
}

pub fn compare_guess(user_guess: &u8, admin_result: &u8) -> String {
    if user_guess > admin_result {
        return "You Lose!".to_string();
    } else if user_guess == admin_result {
        return "It's a tie!".to_string();
    } else {
        return "You win!".to_string();
    }
}