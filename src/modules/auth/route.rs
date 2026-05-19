use axum::{Router, routing::post};

use crate::modules::auth::handler::login_handler;

pub fn routes_auth() -> Router {
    Router::new().route("/auth/login", post(login_handler))
}
