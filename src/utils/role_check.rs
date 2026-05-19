use axum_extra::extract::CookieJar;

use crate::{
    errors::{AppError, AppResult},
    utils::jwt::decode_jwt,
};

pub fn get_role_from_cookie(jar: &CookieJar) -> AppResult<String> {
    let token = jar.get("token").ok_or(AppError::Unauthorized)?;

    let jwt = token.value();

    let claims = decode_jwt(jwt)?;

    Ok(claims.role)
}
