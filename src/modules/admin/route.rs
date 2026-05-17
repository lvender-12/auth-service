use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::modules::admin::handler::{
    create_admin_handler, delete_user_handler, edit_user_handler, find_user_handler,
};

pub fn routes_admin() -> Router {
    Router::new()
        .route("/admin/create", post(create_admin_handler))
        .route("/admin/users", get(find_user_handler))
        .route("/admin/users/{id}", put(edit_user_handler))
        .route("/admin/users/{id}", delete(delete_user_handler))
}
