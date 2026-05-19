use crate::{
    db::load_db,
    errors::{AppError, AppResult},
    modules::user::dto::UserProfile,
};

pub async fn profile_repository(email: String) -> AppResult<UserProfile> {
    let pool = load_db().await;

    let user = sqlx::query_as::<_, UserProfile>(
        "
            SELECT
                users.name,
                users.email,
                roles.name as role_name,
                users.created_at,
                users.updated_at
            FROM users
            JOIN roles ON users.role_id = roles.id
            WHERE users.email = ?
            ",
    )
    .bind(email)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(user)
}
