use axum::{
    Router,
    routing::{get, post},
};

use crate::modules::admin::handler::{create_admin_handler, find_user_handler};

pub fn routes_admin() -> Router {
    Router::new()
        .route("/admin/create", post(create_admin_handler))
        .route("/admin/users", get(find_user_handler))
}
