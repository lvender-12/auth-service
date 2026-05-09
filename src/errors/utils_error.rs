use thiserror::Error;

#[derive(Debug, Error)]
pub enum UtilError {
    #[error("Gagal hash password: {0}")]
    PasswordHash(#[from] argon2::password_hash::Error),
}
