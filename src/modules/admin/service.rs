use validator::Validate;

use crate::{
    errors::AppResult,
    modules::admin::{
        dto::{CreateUserDto, PaginatedUserResponseDto, UpdateUserDto, UserQueryDto},
        repository::{create_admin_repository, edit_user_repository, find_user_repository},
    },
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

pub async fn find_user_service(params: UserQueryDto) -> AppResult<PaginatedUserResponseDto> {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let (users, total) = find_user_repository(&params).await?;
    let total_pages = (total + limit - 1) / limit;
    Ok(PaginatedUserResponseDto {
        data: users.into_iter().map(|u| u.into()).collect(),
        total,
        page,
        limit,
        total_pages,
    })
}

pub async fn edit_user_service(body: UpdateUserDto, id: u64) -> AppResult<()> {
    body.validate()?;
    let body = if let Some(password) = body.password {
        UpdateUserDto {
            password: Some(hash_password(&password)?),
            ..body
        }
    } else {
        body
    };
    edit_user_repository(body, id).await?;
    Ok(())
}
