use crate::{
    db::load_db,
    errors::{AppError, AppResult},
    modules::admin::dto::CreateUserDto,
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
