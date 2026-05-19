use axum::{Router, middleware::from_fn};

use crate::{
    middleware::{
        api_key::check_api_key, method_not_allowed::method_not_allowed_middleware,
        not_found::not_found_middleware,
    },
    modules::admin::route::routes_admin,
};

pub fn app_routes() -> Router {
    Router::new()
        .merge(routes_admin())
        .fallback(not_found_middleware)
        .method_not_allowed_fallback(method_not_allowed_middleware)
        .layer(from_fn(check_api_key))
}
