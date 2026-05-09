use axum::{Json, Router, middleware::from_fn, routing::get};
use http::StatusCode;

use crate::{errors::api_error::ApiError, middleware::auth::jwt_auth};

pub fn auth_router() -> Router {
    Router::new()
        .route("/user", get(test_auth))
        .layer(from_fn(jwt_auth))
}
pub async fn test_auth() -> Result<(StatusCode, Json<String>), ApiError> {
    let user = "auth".to_string();

    Ok((StatusCode::OK, Json(user)))
}
