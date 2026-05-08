// src/dto/user_dto.rs

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::dto::role_dto::RoleResponseDto;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(length(min = 5, max = 20, message = "nim must be between 5 and 20 characters"))]
    pub nim: String,

    #[validate(length(
        min = 3,
        max = 100,
        message = "name must be between 3 and 100 characters"
    ))]
    pub name: String,

    #[validate(email(message = "invalid email format"))]
    pub email: Option<String>,

    #[validate(length(min = 8, message = "password minimum length is 8"))]
    pub password: String,

    pub role_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserDto {
    #[validate(length(min = 5, max = 20, message = "nim must be between 5 and 20 characters"))]
    pub nim: Option<String>,

    #[validate(length(
        min = 3,
        max = 100,
        message = "name must be between 3 and 100 characters"
    ))]
    pub name: Option<String>,

    #[validate(email(message = "invalid email format"))]
    pub email: Option<String>,

    #[validate(length(min = 8, message = "password minimum length is 8"))]
    pub password: Option<String>,

    pub role_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub id: i32,
    pub nim: String,
    pub name: String,
    pub email: Option<String>,
    pub role: Option<RoleResponseDto>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginUserDto {
    #[validate(length(min = 5, max = 20, message = "nim must be between 5 and 20 characters"))]
    pub nim: String,

    #[validate(length(min = 8, message = "password minimum length is 8"))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponseDto {
    pub access_token: String,
    pub token_type: String,
    pub user: UserResponseDto,
}
