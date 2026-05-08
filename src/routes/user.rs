use axum::{Router, routing::post};

use crate::handlers::user_handler::register_handler;

pub fn routes() -> Router {
    Router::new().route("/create", post(register_handler))
}
