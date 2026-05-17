use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validator::Validate;

use crate::entity::user_entity::UserWithRole;

// CREATE
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 3, message = "Name must be at least 3 characters"))]
    pub name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub role_id: u64,
}

// EDIT
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserDto {
    #[validate(length(min = 3, message = "Name must be at least 3 characters"))]
    pub name: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: Option<String>,
    pub role_id: Option<u64>,
}

// READ — response
#[derive(Debug, Serialize, Clone, FromRow)]
pub struct UserResponseDto {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub role_id: u64,
    pub role_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UserQueryDto {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedUserResponseDto {
    pub data: Vec<UserResponseDto>,
    pub total: u64,
    pub page: u64,
    pub limit: u64,
    pub total_pages: u64,
}

impl From<UserWithRole> for UserResponseDto {
    fn from(user: UserWithRole) -> Self {
        UserResponseDto {
            id: user.id,
            name: user.name,
            email: user.email,
            role_id: user.role_id,
            role_name: user.role_name,
        }
    }
}
