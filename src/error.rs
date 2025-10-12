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
