pub mod queries;

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn initialise_database() -> Pool<Postgres> {
    dotenv().ok();

    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("Failed to retrieve the DATABASE_URL env variable");
            std::process::exit(1);
        }
    };

    let db = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            std::process::exit(1);
        }
    };

    println!("Connected to database");
    db
}
