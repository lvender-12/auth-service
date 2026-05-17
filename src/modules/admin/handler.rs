use std::collections::HashMap;

use axum::{Json, extract::Query};
use http::StatusCode;

use crate::{
    errors::AppResult,
    modules::admin::{
        dto::{CreateUserDto, PaginatedUserResponseDto, UserQueryDto, UserResponseDto},
        service::{create_admin_service, find_user_service},
    },
};

pub async fn create_admin_handler(
    Json(body): Json<CreateUserDto>,
) -> AppResult<(StatusCode, String)> {
    create_admin_service(body).await?;
    Ok((
        StatusCode::CREATED,
        "Admin created successfully".to_string(),
    ))
}

#[axum::debug_handler]
pub async fn find_user_handler(
    Query(params): Query<UserQueryDto>,
) -> AppResult<(StatusCode, Json<PaginatedUserResponseDto>)> {
    let result = find_user_service(params).await?;
    Ok((StatusCode::OK, Json(result)))
}
