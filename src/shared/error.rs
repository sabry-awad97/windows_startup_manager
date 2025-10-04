use std::fmt;

/// Custom error types for the startup manager.
/// This provides better error handling than generic anyhow errors.
#[derive(Debug)]
pub enum StartupError {
    /// Path to executable does not exist.
    PathNotFound(String),
    /// Working directory does not exist.
    DirectoryNotFound(String),
    /// Path exists but is not a directory.
    NotADirectory(String),
    /// Entry name is invalid.
    InvalidName(String),
    /// Entry not found in registry.
    #[allow(dead_code)]
    EntryNotFound(String),
    /// Registry access error.
    RegistryError(String),
    /// Generic I/O error.
    IoError(std::io::Error),
}

impl fmt::Display for StartupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StartupError::PathNotFound(path) => {
                write!(f, "The specified path does not exist: {}", path)
            }
            StartupError::DirectoryNotFound(dir) => {
                write!(f, "The specified directory does not exist: {}", dir)
            }
            StartupError::NotADirectory(path) => {
                write!(f, "The specified path is not a directory: {}", path)
            }
            StartupError::InvalidName(msg) => write!(f, "Invalid entry name: {}", msg),
            StartupError::EntryNotFound(name) => {
                write!(f, "Entry '{}' not found in startup registry", name)
            }
            StartupError::RegistryError(msg) => write!(f, "Registry error: {}", msg),
            StartupError::IoError(err) => write!(f, "I/O error: {}", err),
        }
    }
}

impl std::error::Error for StartupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StartupError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for StartupError {
    fn from(err: std::io::Error) -> Self {
        StartupError::IoError(err)
    }
}

/// Result type alias for startup operations.
pub type Result<T> = std::result::Result<T, StartupError>;
