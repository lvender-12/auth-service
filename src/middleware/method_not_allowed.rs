use axum::extract::Request;
use http::StatusCode;

use crate::errors::AppResult;

pub async fn method_not_allowed_middleware(req: Request) -> AppResult<(StatusCode, String)> {
    Ok((
        StatusCode::METHOD_NOT_ALLOWED,
        format!(
            "Method '{}' not allowed on path '{}'",
            req.method(),
            req.uri().path()
        ),
    ))
}
