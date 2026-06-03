use std::sync::Arc;

use auth_service::{
    app::create_app,
    config::{config::load_config, database::db_connection},
    models::state_model::AppState,
};

#[tokio::main]
async fn main() {
    let config = load_config().await;
    let db = db_connection().await.expect("cannot connect to database");
    let host = format!("{}:{}", config.app.host, config.app.port);
    let state = AppState {
        db,
        config: Arc::new(config),
    };
    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind(host)
        .await
        .expect("something wrong in listener");

    axum::serve(listener, app)
        .await
        .expect("cannot running server");
}
