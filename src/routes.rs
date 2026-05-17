use axum::Router;

use crate::modules::admin::route::routes_admin;

pub fn app_routes() -> Router {
    Router::new().merge(routes_admin())
}
