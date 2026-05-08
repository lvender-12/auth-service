// src/repository/user_repository.rs

use anyhow::Result;

use crate::{
    config::database::db_connection, dto::user_dto::CreateUserDto, entity::user_entity::UserEntity,
};

pub async fn create_user(user: CreateUserDto) -> Result<UserEntity> {
    let pool = db_connection().await?;

    let created_user = sqlx::query_as::<_, UserEntity>(
        "
            INSERT INTO users
            (
                nim,
                name,
                email,
                password,
                role_id
            )

            VALUES ($1, $2, $3, $4, $5)

            RETURNING *
            ",
    )
    .bind(user.nim)
    .bind(user.name)
    .bind(user.email)
    .bind(user.password)
    .bind(user.role_id)
    .fetch_one(&pool)
    .await?;

    Ok(created_user)
}
