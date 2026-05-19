use crate::{
    db::load_db,
    entity::user_entity::{User, UserWithRole},
    errors::{AppError, AppResult},
    modules::admin::dto::{CreateUserDto, UpdateUserDto, UserQueryDto},
};

pub async fn create_admin_repository(body: CreateUserDto) -> AppResult<()> {
    let pool = load_db().await;
    sqlx::query("INSERT INTO users (name, email, password, role_id) VALUES (?, ?, ?, ?)")
        .bind(body.name)
        .bind(body.email)
        .bind(body.password)
        .bind(body.role_id)
        .execute(&pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23000") => {
                AppError::Conflict("Email already exists".to_string())
            }
            _ => AppError::Db(e),
        })?;
    Ok(())
}

pub async fn find_user_repository(params: &UserQueryDto) -> AppResult<(Vec<UserWithRole>, u64)> {
    let pool = load_db().await;
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    let offset = (page - 1) * limit;
    let search = format!("%{}%", params.search.clone().unwrap_or_default());

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users
         JOIN roles ON users.role_id = roles.id
         WHERE users.name LIKE ? OR users.email LIKE ?",
    )
    .bind(&search)
    .bind(&search)
    .fetch_one(&pool)
    .await
    .map_err(AppError::Db)?;

    let users = sqlx::query_as::<_, UserWithRole>(
        "SELECT users.*, roles.name as role_name
         FROM users
         JOIN roles ON users.role_id = roles.id
         WHERE users.name LIKE ? OR users.email LIKE ?
         LIMIT ? OFFSET ?",
    )
    .bind(&search)
    .bind(&search)
    .bind(limit)
    .bind(offset)
    .fetch_all(&pool)
    .await
    .map_err(AppError::Db)?;

    Ok((users, total as u64))
}

pub async fn edit_user_repository(body: UpdateUserDto, id: u64) -> AppResult<()> {
    let pool = load_db().await;
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    sqlx::query("UPDATE users SET name = ?, email = ?, password = ?, role_id = ? WHERE id = ?")
        .bind(body.name.unwrap_or(user.name))
        .bind(body.email.unwrap_or(user.email))
        .bind(body.password.unwrap_or(user.password))
        .bind(body.role_id.unwrap_or(user.role_id))
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn delete_user_repository(id: u64) -> AppResult<()> {
    let pool = load_db().await;

    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or(AppError::NotFound)?;

    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(())
}
