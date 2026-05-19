use axum::{extract::Request, middleware::Next, response::Response};
use http::HeaderMap;

use crate::{
    config::load_config,
    errors::{AppError, AppResult},
};

pub async fn check_api_key(headers: HeaderMap, req: Request, next: Next) -> AppResult<Response> {
    let conf = load_config();
    let secret = headers
        .get("x-api-secret")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if secret != conf.api.secret {
        return Err(AppError::Unauthorized);
    }
    Ok(next.run(req).await)
}
