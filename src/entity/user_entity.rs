use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserEntity {
    pub id: i32,
    pub nim: String,
    pub name: String,
    pub email: Option<String>,
    pub password_hash: String,
    pub role_id: Option<i32>,
    pub created_at: NaiveDateTime,
}
