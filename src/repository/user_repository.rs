use sqlx::Row;

use crate::{
    config::database::db_connection, dto::user_dto::CreateUserRepoDto,
    entity::user_entity::UserEntity, errors::repository_error::RepoError,
    models::state_model::AppState,
};

pub async fn create_user(user: CreateUserRepoDto) -> Result<UserEntity, RepoError> {
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

pub async fn check_user_exists(
    state: &AppState,
    nim: &str,
    email: Option<&str>,
) -> Result<bool, RepoError> {
    let exists = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE nim = $1 OR email = $2)")
        .bind(nim)
        .bind(email)
        .fetch_one(&state.db)
        .await?
        .get(0);

    Ok(exists)
}

pub async fn login_user_repo(state: &AppState, nim: &str) -> Result<UserEntity, RepoError> {
    let user = sqlx::query_as::<_, UserEntity>("SELECT * FROM users WHERE nim = $1")
        .bind(nim)
        .fetch_one(&state.db)
        .await?;

    Ok(user)
}
