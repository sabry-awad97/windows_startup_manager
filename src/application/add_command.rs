use crate::domain::{
    ExecutionMode, StartupCommand, StartupEntry, StartupRepository, StartupValidator,
};
use crate::shared::error::{Result, StartupError};
use std::fs;
use std::path::PathBuf;

/// Use case for adding a command with arguments to startup.
/// This follows the Single Responsibility Principle.
pub struct AddCommandUseCase<'a, R: StartupRepository> {
    repository: &'a R,
}

impl<'a, R: StartupRepository> AddCommandUseCase<'a, R> {
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub fn execute(
        &self,
        name: &str,
        command: &str,
        args: Vec<String>,
        workdir: Option<&str>,
        mode: ExecutionMode,
    ) -> Result<()> {
        // Validate inputs
        StartupValidator::validate_name(name)?;

        // Validate working directory if provided
        if let Some(dir) = workdir {
            StartupValidator::validate_directory(dir)?;
        }

        // Create command
        let startup_command = StartupCommand::CommandWithArgs {
            command: command.to_string(),
            args,
            workdir: workdir.map(|s| s.to_string()),
            mode,
        };

        // If using VBScript mode, create the VBScript file
        if mode == ExecutionMode::VBScript
            && let Some((filename, content)) = startup_command.get_vbscript_content()
        {
            self.create_vbscript_file(&filename, &content)?;
        }

        // Convert to registry value and create entry
        let registry_value = startup_command.to_registry_value();
        let entry = StartupEntry::new(name, registry_value);

        // Add to repository
        self.repository.add(&entry)?;

        Ok(())
    }

    /// Creates the VBScript file in %APPDATA%\windows_startup_manager\
    fn create_vbscript_file(&self, filename: &str, content: &str) -> Result<()> {
        // Get APPDATA directory
        let appdata = std::env::var("APPDATA").map_err(|_| {
            StartupError::RegistryError("Failed to get APPDATA environment variable".to_string())
        })?;

        // Create directory path
        let mut dir_path = PathBuf::from(appdata);
        dir_path.push("windows_startup_manager");

        // Create directory if it doesn't exist
        fs::create_dir_all(&dir_path)?;

        // Create file path
        let mut file_path = dir_path;
        file_path.push(filename);

        // Write VBScript content
        fs::write(&file_path, content)?;

        Ok(())
    }
}
