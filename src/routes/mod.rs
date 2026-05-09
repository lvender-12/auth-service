use axum::{Router, middleware::from_fn};

use crate::{
    middleware::auth::api_auth,
    routes::{
        auth_router::auth_router,
        fallback::{method_not_allowed, not_found},
        guest_router::guest_router,
    },
};

pub mod auth_router;
pub mod fallback;
pub mod guest_router;

pub fn user_routes() -> Router {
    Router::new()
        .merge(guest_router())
        .merge(auth_router())
        .layer(from_fn(api_auth))
        .fallback(not_found)
        .method_not_allowed_fallback(method_not_allowed)
}
