use crate::domain::{StartupRepository, StartupValidator};
use crate::shared::error::Result;

/// Use case for removing a startup entry.
/// This follows the Single Responsibility Principle.
pub struct RemoveEntryUseCase<'a, R: StartupRepository> {
    repository: &'a R,
}

impl<'a, R: StartupRepository> RemoveEntryUseCase<'a, R> {
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub fn execute(&self, name: &str) -> Result<()> {
        // Validate input
        StartupValidator::validate_name(name)?;

        // Remove from repository
        self.repository.remove(name)?;

        Ok(())
    }
}
