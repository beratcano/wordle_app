use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct AdminResult {
    pub result: u8,
}

#[derive(Deserialize)]
pub struct UserGuess {
    pub guess: u8,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}