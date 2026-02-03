use thiserror::Error;

/// Errors specific to `tomldir`.
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Configuration key not found: {0}")]
    NotFound(String),

    #[error("Type mismatch: expected {expected}, found {found}")]
    TypeMismatch { expected: String, found: String },
}

pub type Result<T> = std::result::Result<T, Error>;
