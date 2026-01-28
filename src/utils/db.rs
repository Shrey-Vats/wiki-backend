use std::time::Duration;

use sqlx::{PgPool, Result as Result_db, postgres::PgPoolOptions};


pub async fn init_db_pool(db_url: &str) -> Result_db<PgPool> {
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(db_url).await?;

    Ok(pool)
}