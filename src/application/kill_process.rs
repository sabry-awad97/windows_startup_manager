use crate::domain::{StartupRepository, StartupValidator};
use crate::infrastructure::ProcessManager;
use crate::shared::error::{Result, StartupError};

/// Use case for killing a process associated with a startup entry.
pub struct KillProcessUseCase<'a, R: StartupRepository> {
    repository: &'a R,
}

impl<'a, R: StartupRepository> KillProcessUseCase<'a, R> {
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub fn execute(&self, name: &str) -> Result<u32> {
        // Validate input
        StartupValidator::validate_name(name)?;

        // Get the entry from repository
        let entries = self.repository.list()?;
        let entry = entries
            .iter()
            .find(|e| e.name == name)
            .ok_or_else(|| {
                StartupError::RegistryError(format!("Entry '{}' not found in startup registry", name))
            })?;

        // Extract executable name from command
        let exe_name = ProcessManager::extract_executable_name(&entry.command)
            .ok_or_else(|| {
                StartupError::RegistryError(format!(
                    "Could not determine executable name from command: {}",
                    entry.command
                ))
            })?;

        // Kill all processes with that name
        let count = ProcessManager::kill_processes_by_name(&exe_name)?;

        Ok(count)
    }
}
