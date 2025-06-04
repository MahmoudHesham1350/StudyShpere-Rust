use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use anyhow::Result;

pub async fn init_db_pool() -> Result<PgPool> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}