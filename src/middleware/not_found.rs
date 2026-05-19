use axum::extract::Request;
use http::StatusCode;

use crate::errors::AppResult;

pub async fn not_found_middleware(req: Request) -> AppResult<(StatusCode, String)> {
    Ok((
        StatusCode::NOT_FOUND,
        format!("Path '{}' not found", req.uri().path()),
    ))
}
