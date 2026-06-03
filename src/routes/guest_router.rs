use axum::{Router, routing::post};

use crate::{
    handlers::user_handler::{login_handler, register_handler},
    models::state_model::AppState,
};

pub fn guest_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
}
