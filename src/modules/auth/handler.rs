use axum::Json;
use axum_extra::extract::CookieJar;
use http::StatusCode;

use crate::{
    errors::AppResult,
    modules::auth::{dto::LoginDto, service::login_service},
};

#[axum::debug_handler]
pub async fn login_handler(
    Json(body): Json<LoginDto>,
) -> AppResult<(StatusCode, CookieJar, String)> {
    let jar = login_service(body).await?;
    Ok((StatusCode::OK, jar, "Login Success".to_string()))
}
