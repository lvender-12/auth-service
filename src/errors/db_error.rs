use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Gagal konek ke database: {0}")]
    ConnectionFailed(#[from] sqlx::Error),

    #[error("Database URL tidak ditemukan di environment")]
    UrlNotFound,
}
