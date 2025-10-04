#[cfg(test)]
mod tests {
    use super::super::*;
    use std::fs;

    #[test]
    fn test_validate_name_valid() {
        let result = StartupValidator::validate_name("ValidName");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_name_with_spaces() {
        let result = StartupValidator::validate_name("Valid Name With Spaces");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_name_empty() {
        let result = StartupValidator::validate_name("");
        assert!(result.is_err());

        if let Err(e) = result {
            match e {
                crate::shared::error::StartupError::InvalidName(msg) => {
                    assert!(msg.contains("empty"));
                }
                _ => panic!("Expected InvalidName error"),
            }
        }
    }

    #[test]
    fn test_validate_name_only_whitespace() {
        let result = StartupValidator::validate_name("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_name_tabs_and_spaces() {
        let result = StartupValidator::validate_name("\t\n  ");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_path_nonexistent() {
        let result = StartupValidator::validate_path("C:\\nonexistent\\path\\file.exe");
        assert!(result.is_err());

        if let Err(e) = result {
            match e {
                crate::shared::error::StartupError::PathNotFound(path) => {
                    assert!(path.contains("nonexistent"));
                }
                _ => panic!("Expected PathNotFound error"),
            }
        }
    }

    #[test]
    fn test_validate_directory_nonexistent() {
        let result = StartupValidator::validate_directory("C:\\nonexistent\\directory");
        assert!(result.is_err());

        if let Err(e) = result {
            match e {
                crate::shared::error::StartupError::DirectoryNotFound(dir) => {
                    assert!(dir.contains("nonexistent"));
                }
                _ => panic!("Expected DirectoryNotFound error"),
            }
        }
    }

    #[test]
    fn test_validate_directory_is_file() {
        // Create a temporary file
        let temp_file = std::env::temp_dir().join("test_file.txt");
        fs::write(&temp_file, "test").unwrap();

        let result = StartupValidator::validate_directory(temp_file.to_str().unwrap());

        // Clean up
        fs::remove_file(&temp_file).ok();

        assert!(result.is_err());
        if let Err(e) = result {
            match e {
                crate::shared::error::StartupError::NotADirectory(_) => {
                    // Expected error
                }
                _ => panic!("Expected NotADirectory error"),
            }
        }
    }

    #[test]
    fn test_validate_directory_valid() {
        // Use temp directory which should always exist
        let temp_dir = std::env::temp_dir();
        let result = StartupValidator::validate_directory(temp_dir.to_str().unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_path_valid() {
        // Create a temporary file
        let temp_file = std::env::temp_dir().join("test_validate.txt");
        fs::write(&temp_file, "test").unwrap();

        let result = StartupValidator::validate_path(temp_file.to_str().unwrap());

        // Clean up
        fs::remove_file(&temp_file).ok();

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_name_special_characters() {
        // Names with special characters should be valid
        let result = StartupValidator::validate_name("App-Name_v1.0");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_name_unicode() {
        // Unicode characters should be valid
        let result = StartupValidator::validate_name("应用程序");
        assert!(result.is_ok());
    }
}
