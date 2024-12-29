use sqlx::{
    sqlite::SqlitePoolOptions, 
    SqlitePool,
    query,
    Error,
};
use std::{
    env,
};

pub async fn establish_connection() -> Result<SqlitePool, Error> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:db.sqlite".to_string());
    let pool = SqlitePool::connect(&database_url).await?;
    Ok(pool)
}

pub async fn initalize_db(pool: &SqlitePool) -> Result<(), Error> {
    query(
        r#"
        CREATE TABLE IF NOT EXISTS users(
            username TEXT PRIMARY KEY,
            password TEXT NOT NULL
        );
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}