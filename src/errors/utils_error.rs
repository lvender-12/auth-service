use thiserror::Error;

#[derive(Debug, Error)]
pub enum UtilError {
    #[error("Gagal hash password: {0}")]
    PasswordHash(#[from] argon2::password_hash::Error),

    #[error("Password tidak valid")]
    InvalidPassword,

    #[error("Gagal generate token: {0}")]
    TokenGenerate(#[from] jsonwebtoken::errors::Error),

    #[error("Gagal kalkulasi waktu expired token")]
    TokenExpiration,
}
