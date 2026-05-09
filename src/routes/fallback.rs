use http::StatusCode;

pub async fn not_found() -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, "Not Found".to_string())
}

pub async fn method_not_allowed() -> (StatusCode, String) {
    (
        StatusCode::METHOD_NOT_ALLOWED,
        "Method Not Allowed".to_string(),
    )
}
