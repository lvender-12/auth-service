use thiserror::Error;

use crate::errors::{repository_error::RepoError, utils_error::UtilError};

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("User sudah terdaftar")]
    UserAlreadyExists,

    #[error("User tidak ditemukan")]
    UserNotFound,

    #[error("NIM atau password salah")]
    InvalidCredentials, // ← tambah ini

    #[error("Gagal memproses password")]
    PasswordError,

    #[error("Terjadi kesalahan internal")]
    Unexpected(String),
}

impl From<RepoError> for ServiceError {
    fn from(e: RepoError) -> Self {
        match e {
            RepoError::NotFound => ServiceError::UserNotFound,
            RepoError::AlreadyExists => ServiceError::UserAlreadyExists,
            e => ServiceError::Unexpected(e.to_string()),
        }
    }
}

impl From<UtilError> for ServiceError {
    fn from(e: UtilError) -> Self {
        match e {
            UtilError::PasswordHash(_) => ServiceError::PasswordError,
            UtilError::InvalidPassword => ServiceError::PasswordError,
            UtilError::TokenGenerate(_) => {
                ServiceError::Unexpected("Token generate error".to_string())
            }
            UtilError::TokenExpiration => {
                ServiceError::Unexpected("Token expiration error".to_string())
            }
        }
    }
}
