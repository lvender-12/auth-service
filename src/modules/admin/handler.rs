use axum::Json;
use http::StatusCode;
use validator::Validate;

use crate::{
    errors::AppResult,
    modules::admin::{dto::CreateUserDto, service::create_admin_service},
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
