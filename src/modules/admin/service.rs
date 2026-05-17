use validator::Validate;

use crate::{
    errors::AppResult,
    modules::admin::{dto::CreateUserDto, repository::create_admin_repository},
    utils::hash::hash_password,
};

pub async fn create_admin_service(body: CreateUserDto) -> AppResult<()> {
    body.validate()?;
    let password_hash = hash_password(&body.password)?;
    let body = CreateUserDto {
        password: password_hash,
        ..body
    };
    create_admin_repository(body).await?;
    Ok(())
}
