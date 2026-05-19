use axum::{
    Json,
    extract::{Path, Query},
};
use http::StatusCode;

use crate::{
    errors::AppResult,
    modules::admin::{
        dto::{CreateUserDto, PaginatedUserResponseDto, UpdateUserDto, UserQueryDto},
        service::{
            create_admin_service, delete_user_service, edit_user_service, find_user_service,
        },
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

pub async fn find_user_handler(
    Query(params): Query<UserQueryDto>,
) -> AppResult<(StatusCode, Json<PaginatedUserResponseDto>)> {
    let result = find_user_service(params).await?;
    Ok((StatusCode::OK, Json(result)))
}

pub async fn edit_user_handler(
    Path(id): Path<u64>,
    Json(body): Json<UpdateUserDto>,
) -> AppResult<(StatusCode, String)> {
    edit_user_service(body, id).await?;
    Ok((StatusCode::OK, "Berhasil edit data".to_string()))
}

pub async fn delete_user_handler(Path(id): Path<u64>) -> AppResult<(StatusCode, String)> {
    delete_user_service(id).await?;
    Ok((StatusCode::OK, "Berhasil hapus data".to_string()))
}
