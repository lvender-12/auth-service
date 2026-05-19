use axum_extra::extract::{CookieJar, cookie::Cookie};
use validator::Validate;

use crate::{
    errors::{AppError, AppResult},
    modules::auth::{dto::LoginDto, repository::login_repository},
    utils::{hash::verify_password, jwt::generate_token},
};

pub async fn login_service(body: LoginDto) -> AppResult<CookieJar> {
    body.validate()?;
    let user = login_repository(body.email).await?;

    if !verify_password(&body.password, &user.password)? {
        return Err(AppError::Unauthorized);
    }

    let token = generate_token(user.id, &user.email, &user.role_name)?;

    let cookie = Cookie::build(("token", token))
        .http_only(true)
        .path("/")
        .build();

    Ok(CookieJar::new().add(cookie))
}
