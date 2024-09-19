pub mod queries;

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn initialise_database(
) -> Result<Pool<Postgres>, Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")?;

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    Ok(db)
}
