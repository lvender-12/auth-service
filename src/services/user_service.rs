use anyhow::Result;

use crate::{
    dto::{
        role_dto::RoleResponseDto,
        user_dto::{CreateUserDto, UserResponseDto},
    },
    repository::user_repository::create_user,
    utils::util::password_hash,
};

pub async fn register_user(dto: CreateUserDto) -> Result<UserResponseDto> {
    let password = password_hash(dto.password)?;

    let create_user_dto = CreateUserDto {
        nim: dto.nim,
        name: dto.name,
        email: dto.email,
        password,
        role_id: dto.role_id,
    };

    let created_user = create_user(create_user_dto).await?;

    Ok(UserResponseDto {
        id: created_user.id,
        nim: created_user.nim,
        name: created_user.name,
        email: created_user.email,
        role: created_user.role_id.map(|id| RoleResponseDto {
            id,
            name: String::from("user"),
        }),
        created_at: created_user.created_at,
    })
}
