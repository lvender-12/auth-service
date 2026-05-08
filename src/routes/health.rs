use axum::{Router, routing::get};

use crate::handlers::health_handler;

pub fn routes() -> Router {
    Router::new().route("/", get(health_handler::handler))
}
