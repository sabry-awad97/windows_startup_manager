use crate::shared::error::{Result, StartupError};
use std::path::Path;

/// Validates startup-related inputs.
/// This follows the Single Responsibility Principle.
pub struct StartupValidator;

impl StartupValidator {
    /// Validates that a file path exists.
    pub fn validate_path(path: &str) -> Result<()> {
        if !Path::new(path).exists() {
            return Err(StartupError::PathNotFound(path.to_string()));
        }
        Ok(())
    }

    /// Validates that a directory exists.
    pub fn validate_directory(dir: &str) -> Result<()> {
        let path = Path::new(dir);
        if !path.exists() {
            return Err(StartupError::DirectoryNotFound(dir.to_string()));
        }
        if !path.is_dir() {
            return Err(StartupError::NotADirectory(dir.to_string()));
        }
        Ok(())
    }

    /// Validates that an entry name is not empty.
    pub fn validate_name(name: &str) -> Result<()> {
        if name.trim().is_empty() {
            return Err(StartupError::InvalidName(
                "Entry name cannot be empty".to_string(),
            ));
        }
        Ok(())
    }
}
