use axum::{extract::Request, middleware::Next, response::Response};
use axum_extra::extract::CookieJar;

use crate::{
    errors::{AppError, AppResult},
    utils::{jwt::decode_jwt, role_check::get_role_from_cookie},
};

pub async fn admin_only_middleware(jar: CookieJar, req: Request, nex: Next) -> AppResult<Response> {
    let role = get_role_from_cookie(&jar)?;
    if role != "admin" {
        return Err(AppError::Forbidden);
    }
    Ok(nex.run(req).await)
}

pub async fn _dosen_only_middleware(
    jar: CookieJar,
    req: Request,
    nex: Next,
) -> AppResult<Response> {
    let role = get_role_from_cookie(&jar)?;
    if role != "dosen" {
        return Err(AppError::Forbidden);
    }
    Ok(nex.run(req).await)
}

pub async fn _mahasiswa_only_middleware(
    jar: CookieJar,
    req: Request,
    nex: Next,
) -> AppResult<Response> {
    let role = get_role_from_cookie(&jar)?;
    if role != "mahasiswa" {
        return Err(AppError::Forbidden);
    }
    Ok(nex.run(req).await)
}

pub async fn login_only_middleware(jar: CookieJar, req: Request, nex: Next) -> AppResult<Response> {
    let token = jar.get("token").ok_or(AppError::Forbidden)?;
    let jwt = token.value();
    decode_jwt(jwt)?;
    Ok(nex.run(req).await)
}
