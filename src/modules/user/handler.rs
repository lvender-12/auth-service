use axum::Json;
use axum_extra::extract::CookieJar;
use http::StatusCode;

use crate::{
    errors::AppResult,
    modules::user::{dto::UserProfile, service::profile_service},
};

pub async fn profile_handler(jar: CookieJar) -> AppResult<(StatusCode, Json<UserProfile>)> {
    let user = profile_service(jar).await?;
    Ok((StatusCode::OK, Json(user)))
}
