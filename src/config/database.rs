use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;

use crate::config::config::load_config;
use crate::errors::db_error::DbError;

pub async fn db_connection() -> Result<Pool<Postgres>, DbError> {
    let config = load_config().await;
    let url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        config.database.username,
        config.database.password,
        config.database.host,
        config.database.port,
        config.database.name
    );

    let pool = PgPoolOptions::new()
        .min_connections(10)
        .max_connections(100)
        .acquire_timeout(Duration::from_mins(30))
        .idle_timeout(Duration::from_mins(60))
        .connect(&url)
        .await?;
    Ok(pool)
}
