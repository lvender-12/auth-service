// src/dto/user_dto.rs

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::dto::role_dto::RoleResponseDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub nim: String,
    pub name: String,
    pub email: Option<String>,
    pub password: String,
    pub role_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub nim: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
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
