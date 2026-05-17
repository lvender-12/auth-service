use axum::{Router, routing::post};

use crate::modules::admin::handler::create_admin_handler;

pub fn routes_admin() -> Router {
    Router::new().route("/admin/create", post(create_admin_handler))
}
