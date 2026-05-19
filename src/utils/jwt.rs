use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::{config::load_config, errors::AppResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: u64,
    pub email: String,
    pub role: String,
    pub exp: usize,
}
pub fn generate_token(id: u64, email: &str, role: &str) -> AppResult<String> {
    let conf = load_config().jwt;
    let exp = Utc::now() + Duration::seconds(conf.expiry as i64);
    let claims = Claims {
        sub: id,
        email: email.to_string(),
        role: role.to_string(),
        exp: exp.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(conf.secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn decode_jwt(token: &str) -> AppResult<Claims> {
    let conf = load_config().jwt;
    let decode = decode::<Claims>(
        token,
        &DecodingKey::from_secret(conf.secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(decode.claims)
}
