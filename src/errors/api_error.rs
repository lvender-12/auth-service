use crate::errors::services_error::ServiceError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String), // ← tambah

    #[error("Method not allowed")]
    MethodNotAllowed, // ← tambah

    #[error("Validation error")]
    Validation(#[from] ValidationErrors),

    #[error("Internal server error")]
    Internal,
}

impl From<ServiceError> for ApiError {
    fn from(e: ServiceError) -> Self {
        match e {
            ServiceError::UserNotFound => ApiError::NotFound(e.to_string()),
            ServiceError::UserAlreadyExists => ApiError::Conflict(e.to_string()),
            ServiceError::InvalidCredentials => ApiError::Unauthorized(e.to_string()), // ← tambah
            ServiceError::PasswordError => ApiError::Internal,
            ServiceError::Unexpected(_) => ApiError::Internal,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()), // ← tambah
            ApiError::MethodNotAllowed => (
                StatusCode::METHOD_NOT_ALLOWED,
                "Method not allowed".to_string(),
            ), // ← tambah
            ApiError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
            ApiError::Validation(e) => {
                let errors = e
                    .field_errors()
                    .iter()
                    .map(|(field, errs)| {
                        let messages: Vec<String> = errs
                            .iter()
                            .filter_map(|e| e.message.as_ref().map(|m| m.to_string()))
                            .collect();
                        (field.to_string(), messages)
                    })
                    .collect::<std::collections::HashMap<_, _>>();
                return (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(json!({
                        "success": false,
                        "status": 422,
                        "error": "Validation error",
                        "fields": errors
                    })),
                )
                    .into_response();
            }
        };
        (
            status,
            Json(json!({
                "success": false,
                "status": status.as_u16(),
                "error": message
            })),
        )
            .into_response()
    }
}
