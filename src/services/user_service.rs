use crate::{
    dto::{
        role_dto::RoleResponseDto,
        user_dto::{CreateUserDto, CreateUserRepoDto, UserResponseDto},
    },
    errors::services_error::ServiceError,
    repository::user_repository::{check_user_exists, create_user},
    utils::util::password_hash,
};

pub async fn register_user(dto: CreateUserDto) -> Result<UserResponseDto, ServiceError> {
    let password = password_hash(dto.password)?;

    let user_exists = check_user_exists(&dto.nim, dto.email.as_deref()).await?;
    if user_exists {
        return Err(ServiceError::UserAlreadyExists);
    }

    let create_user_dto = CreateUserRepoDto {
        nim: dto.nim,
        name: dto.name,
        email: dto.email,
        password,
        role_id: Some(2),
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
