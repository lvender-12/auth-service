use axum::Json;
use http::StatusCode;
use validator::Validate;

use crate::{
    dto::user_dto::{CreateUserDto, UserResponseDto},
    services::user_service::register_user,
};

pub async fn register_handler(
    Json(dto): Json<CreateUserDto>,
) -> Result<(StatusCode, Json<UserResponseDto>), (StatusCode, String)> {
    dto.validate()
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let user = register_user(dto)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(user)))
}
