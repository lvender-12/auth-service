use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use jsonwebtoken::{EncodingKey, Header};

use crate::{
    config::config::load_config, entity::user_entity::UserEntity, errors::utils_error::UtilError,
    models::jwt_model::Claims,
};

pub fn password_hash(password: String) -> Result<String, UtilError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();

    let hash = argon.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(hash)
}

pub fn password_verify(password: &str, hash: &str) -> Result<(), UtilError> {
    let parsed_hash = PasswordHash::new(hash).map_err(|_| UtilError::InvalidPassword)?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| UtilError::InvalidPassword)?;
    Ok(())
}

pub async fn generate_token(user: &UserEntity) -> Result<String, UtilError> {
    let config = load_config().await;
    let exp = chrono::Local::now()
        .checked_add_signed(chrono::Duration::days(30))
        .ok_or(UtilError::TokenExpiration)?
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id,
        nim: user.nim.clone(),
        role_id: user.role_id.unwrap_or_default(),
        exp,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt.secret.as_bytes()),
    )?;
    Ok(token)
}
