use thiserror::Error;

/// Error types for hexler operations.
///
/// This enum uses the `thiserror` crate to provide automatic `Display` and `Error` 
/// trait implementations with descriptive error messages.
#[derive(Error, Debug)]
pub enum HexlerError {
    /// Invalid bytes_per_line value.
    ///
    /// The bytes_per_line parameter must be a multiple of 8 (to align with byte groups)
    /// and at least 8 (minimum one group).
    #[error("bytes per line must be a multiple of 8 with a minimum of 8, got {0}")]
    InvalidBytesPerLine(usize),

    /// I/O operation failed.
    ///
    /// Automatically converts from `std::io::Error` via the `#[from]` attribute,
    /// allowing `?` operator to work seamlessly with I/O operations.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Failed to determine terminal dimensions.
    ///
    /// This error occurs when the terminal size cannot be determined, which is needed
    /// to calculate the optimal bytes_per_line value for the current terminal width.
    #[error("failed to determine terminal width")]
    TerminalSizeError,
}

/// Type alias for Results that use `HexlerError` as the error type.
///
/// This simplifies function signatures throughout the codebase:
/// ```
/// fn process() -> Result<()> { ... }
/// ```
/// instead of:
/// ```
/// fn process() -> std::result::Result<(), HexlerError> { ... }
/// ```
pub type Result<T> = std::result::Result<T, HexlerError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_bytes_per_line_error_message() {
        let error = HexlerError::InvalidBytesPerLine(5);
        let message = error.to_string();
        assert!(message.contains("bytes per line must be a multiple of 8"));
        assert!(message.contains("got 5"));
    }

    #[test]
    fn test_terminal_size_error_message() {
        let error = HexlerError::TerminalSizeError;
        let message = error.to_string();
        assert!(message.contains("failed to determine terminal width"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let hexler_error: HexlerError = io_error.into();
        
        let message = hexler_error.to_string();
        assert!(message.contains("IO error"));
        assert!(message.contains("file not found"));
    }

    #[test]
    fn test_error_debug_formatting() {
        let error = HexlerError::InvalidBytesPerLine(12);
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("InvalidBytesPerLine"));
        assert!(debug_str.contains("12"));
    }

    #[test]
    fn test_result_type_alias() {
        fn returns_result() -> Result<i32> {
            Ok(42)
        }
        
        let result = returns_result();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_result_with_error() {
        fn returns_error() -> Result<i32> {
            Err(HexlerError::TerminalSizeError)
        }
        
        let result = returns_error();
        assert!(result.is_err());
    }
}
