use thiserror::Error;

use crate::errors::{repository_error::RepoError, utils_error::UtilError};

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("User sudah terdaftar")]
    UserAlreadyExists,

    #[error("User tidak ditemukan")]
    UserNotFound,

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
        }
    }
}
