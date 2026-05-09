use axum::Router;

use crate::routes::{
    fallback::{method_not_allowed, not_found},
    user::user_route,
};

pub mod fallback;
pub mod user;

pub fn user_routes() -> Router {
    Router::new()
        .merge(user_route())
        .fallback(not_found)
        .method_not_allowed_fallback(method_not_allowed)
}
