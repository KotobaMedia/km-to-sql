use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Failed to parse metadata: {0}")]
    ParseMetadata(#[from] serde_json::Error),

    #[cfg(feature = "postgres")]
    #[error("Postgres error: {0}")]
    Postgres(#[from] tokio_postgres::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
