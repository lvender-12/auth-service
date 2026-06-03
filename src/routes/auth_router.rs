use axum::{Json, Router, middleware::from_fn_with_state, routing::get};
use http::StatusCode;

use crate::{
    errors::api_error::ApiError, middleware::auth::jwt_auth, models::state_model::AppState,
};

pub fn auth_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/user", get(test_auth))
        .layer(from_fn_with_state(state.clone(), jwt_auth))
}
pub async fn test_auth() -> Result<(StatusCode, Json<String>), ApiError> {
    let user = "auth".to_string();

    Ok((StatusCode::OK, Json(user)))
}
