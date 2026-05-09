use axum::Router;

use crate::routes::user_routes;
pub async fn create_app() -> Router {
    Router::new().merge(user_routes())
}
