use thiserror::Error;

use crate::event_systems::TapeEvent;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to open file {0}: {1}")]
    OpenFile(String, std::io::Error),
    #[error("Failed to read file {0}: {1}")]
    ReadFile(String, std::io::Error),
    #[error("color_erye error")]
    ColorErye(#[from] color_eyre::eyre::Error),
    #[error("Ratatui IO error")]
    RatatuiIO(#[from] std::io::Error),
    #[error("No events present")]
    NoEvents,
    #[error("Invalid buffer size")]
    InvalidBufferSize,
    #[error("Error in the tape channel")]
    TapeChannel(#[from] tokio::sync::mpsc::error::SendError<TapeEvent>),
}

pub type Result<T> = std::result::Result<T, Error>;
