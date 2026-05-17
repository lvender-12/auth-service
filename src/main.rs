use crate::{config::load_config, db::load_db, routes::app_routes};

mod config;
mod db;
mod entity;
mod errors;
mod middleware;
mod model;
mod modules;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    let app = app_routes();
    let conf = load_config();
    load_db().await;
    let host = format!("{}:{}", conf.app.host, conf.app.port);

    println!("{} running on http://{}", conf.app.name, host);

    let listener = tokio::net::TcpListener::bind(host)
        .await
        .expect("Something went wrong on port 5000");

    axum::serve(listener, app)
        .await
        .expect("Can't run server on port 5000")
}
