use std::time::Duration;

use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};

use crate::config::load_config;

pub async fn load_db() -> Pool<MySql> {
    let conf = load_config().db;
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        conf.username, conf.password, conf.host, conf.port, conf.name
    );

    MySqlPoolOptions::new()
        .max_connections(100)
        .min_connections(10)
        .acquire_timeout(Duration::from_mins(30))
        .idle_timeout(Duration::from_mins(5))
        .connect(&url)
        .await
        .expect("can't connect to database")
}
