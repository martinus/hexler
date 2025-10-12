use thiserror::Error;

/// Custom error type for hexler operations
#[derive(Error, Debug)]
pub enum HexlerError {
    /// Error when bytes per line is invalid
    #[error("bytes per line must be a multiple of 8 with a minimum of 8, got {0}")]
    InvalidBytesPerLine(usize),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Terminal size error
    #[error("failed to determine terminal width")]
    TerminalSizeError,
}

/// Type alias for Results using HexlerError
pub type Result<T> = std::result::Result<T, HexlerError>;
