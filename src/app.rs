use axum::{Router, middleware::from_fn_with_state};

use crate::{
    middleware::auth::api_auth,
    models::state_model::AppState,
    routes::{
        auth_router::auth_router,
        fallback::{method_not_allowed, not_found},
        guest_router::guest_router,
    },
};

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .merge(guest_router())
        .merge(auth_router(state.clone()))
        .fallback(not_found)
        .layer(from_fn_with_state(state.clone(), api_auth))
        .with_state(state)
        .method_not_allowed_fallback(method_not_allowed)
}
