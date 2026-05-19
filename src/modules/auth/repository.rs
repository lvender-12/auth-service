use crate::{
    db::load_db,
    entity::user_entity::UserWithRole,
    errors::{AppError, AppResult},
};

pub async fn login_repository(email: String) -> AppResult<UserWithRole> {
    let pool = load_db().await;
    let user = sqlx::query_as::<_, UserWithRole>(
        "SELECT users.*, roles.name as role_name
             FROM users
             JOIN roles ON users.role_id = roles.id
             WHERE users.email = ?",
    )
    .bind(email)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(user)
}
