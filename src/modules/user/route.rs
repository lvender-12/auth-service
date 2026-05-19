use axum::{Router, middleware::from_fn, routing::get};

use crate::{middleware::auth::login_only_middleware, modules::user::handler::profile_handler};

pub fn routes_user() -> Router {
    Router::new()
        .route("/profile", get(profile_handler))
        .layer(from_fn(login_only_middleware))
}
