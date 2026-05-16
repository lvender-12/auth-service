use axum::{Router, routing::get};

pub async fn app_routes() -> Router {
    Router::new().route("/", get(|| async { "Hello, World!" }))
}
