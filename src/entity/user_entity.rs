use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role_id: u64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserWithRole {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role_id: u64,
    pub role_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
