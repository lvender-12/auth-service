use axum_extra::extract::CookieJar;

use crate::{
    errors::{AppError, AppResult},
    modules::user::{dto::UserProfile, repository::profile_repository},
    utils::jwt::decode_jwt,
};

pub async fn profile_service(jar: CookieJar) -> AppResult<UserProfile> {
    let token = jar.get("token").ok_or(AppError::Unauthorized)?;

    let jwt = token.value();

    let claims = decode_jwt(jwt)?;

    let user = profile_repository(claims.email).await?;
    Ok(user)
}
