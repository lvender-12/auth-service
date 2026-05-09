use axum::Json;
use http::StatusCode;
use validator::Validate;

use crate::{
    dto::user_dto::{CreateUserDto, LoginResponseDto, LoginUserDto, UserResponseDto},
    errors::api_error::ApiError,
    services::user_service::{login_user, register_user},
};

pub async fn register_handler(
    Json(dto): Json<CreateUserDto>,
) -> Result<(StatusCode, Json<UserResponseDto>), ApiError> {
    dto.validate()?;

    let user = register_user(dto).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn login_handler(
    Json(dto): Json<LoginUserDto>,
) -> Result<(StatusCode, Json<LoginResponseDto>), ApiError> {
    dto.validate()?;

    let user = login_user(dto).await?;

    Ok((StatusCode::OK, Json(user)))
}
