use axum::Json;
use http::StatusCode;
use validator::Validate;

use crate::{
    dto::user_dto::{CreateUserDto, UserResponseDto},
    errors::api_error::ApiError,
    services::user_service::register_user,
};

pub async fn register_handler(
    Json(dto): Json<CreateUserDto>,
) -> Result<(StatusCode, Json<UserResponseDto>), ApiError> {
    dto.validate()?;

    let user = register_user(dto).await?;

    Ok((StatusCode::CREATED, Json(user)))
}
