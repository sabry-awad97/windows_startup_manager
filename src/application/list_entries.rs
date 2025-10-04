use crate::domain::{StartupEntry, StartupRepository};
use crate::shared::error::Result;

/// Use case for listing all startup entries.
/// This follows the Single Responsibility Principle.
pub struct ListEntriesUseCase<'a, R: StartupRepository> {
    repository: &'a R,
}

impl<'a, R: StartupRepository> ListEntriesUseCase<'a, R> {
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub fn execute(&self) -> Result<Vec<StartupEntry>> {
        self.repository.list()
    }
}
