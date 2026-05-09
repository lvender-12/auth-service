use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use jsonwebtoken::{DecodingKey, Validation};

use crate::{config::config::load_config, errors::api_error::ApiError, models::jwt_model::Claims};

pub async fn api_auth(req: Request<Body>, next: Next) -> Result<Response, ApiError> {
    let config = load_config().await;

    let api_key = req.headers().get("X-api-key").and_then(|v| v.to_str().ok());

    match api_key {
        Some(key) if key == config.api.secret => Ok(next.run(req).await),
        Some(_) => Err(ApiError::Unauthorized("API key tidak valid".to_string())),
        None => Err(ApiError::Unauthorized(
            "API key tidak ditemukan".to_string(),
        )),
    }
}

pub async fn jwt_auth(mut req: Request<Body>, next: Next) -> Result<Response, ApiError> {
    let config = load_config().await;

    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(ApiError::Unauthorized("Token tidak ditemukan".to_string()))?;

    let claims = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(config.jwt.secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| ApiError::Unauthorized("Token tidak valid".to_string()))?
    .claims;

    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
