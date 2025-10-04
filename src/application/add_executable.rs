use crate::domain::{StartupEntry, StartupRepository, StartupValidator};
use crate::shared::error::Result;

/// Use case for adding an executable to startup.
/// This follows the Single Responsibility Principle.
pub struct AddExecutableUseCase<'a, R: StartupRepository> {
    repository: &'a R,
}

impl<'a, R: StartupRepository> AddExecutableUseCase<'a, R> {
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub fn execute(&self, name: &str, path: &str) -> Result<()> {
        // Validate inputs
        StartupValidator::validate_name(name)?;
        StartupValidator::validate_path(path)?;

        // Create and add entry
        let entry = StartupEntry::new(name, path);
        self.repository.add(&entry)?;

        Ok(())
    }
}
