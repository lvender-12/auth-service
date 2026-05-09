use axum::{Router, routing::post};

use crate::handlers::user_handler::{login_handler, register_handler};

pub fn guest_router() -> Router {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
}
