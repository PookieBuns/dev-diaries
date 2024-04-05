use crate::Result;
use sqlx::postgres::{PgPool, PgPoolOptions};

const DATABASE_URL: &str = env!("DATABASE_URL");

pub async fn db_pool() -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await?;
    Ok(pool)
}
