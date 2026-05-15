use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to open file {0}: {1}")]
    OpenFile(String, std::io::Error),
    #[error("Failed to read file {0}: {1}")]
    ReadFile(String, std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
