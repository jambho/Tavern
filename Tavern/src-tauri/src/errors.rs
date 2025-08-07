use std::fmt;
use thiserror::Error;
use tauri::ipc::InvokeError;
use sqlx::Error as SqlxError;
use sqlx::migrate::MigrateError as MigrateError;
use uuid::Error as UuidError;
use std::io::Error as IoError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Item not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("UUID error: {0}")]
    UuidError(#[from] UuidError),

    #[error("IO error: {0}")]
    IoError(#[from] IoError),

    #[error("SQLx error: {0}")]
    SqlxError(#[from] SqlxError),
    
    #[error("SQLx Migrate error: {0}")]
    MigrateError(#[from] MigrateError),

    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Other error: {0}")]
    Other(String),
}

// Optional: Convert to Tauri InvokeError so your commands can return them directly
impl From<AppError> for InvokeError {
    fn from(err: AppError) -> Self {
        InvokeError::from_anyhow(anyhow::anyhow!(err))
    }
}