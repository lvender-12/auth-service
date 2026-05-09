// src/repository/error.rs
use crate::errors::db_error::DbError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("Data tidak ditemukan")]
    NotFound,

    #[error("NIM atau email sudah dipakai")]
    AlreadyExists,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Koneksi database gagal: {0}")]
    Connection(#[from] DbError), // ← tambah ini
}
