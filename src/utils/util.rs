use anyhow::Result;
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};

pub fn password_hash(password: String) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon = Argon2::default();

    let hash = argon.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(hash)
}
