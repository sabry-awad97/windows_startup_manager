use super::models::StartupEntry;
use crate::shared::error::Result;

/// Repository trait for managing startup entries.
/// This follows the Repository pattern and Dependency Inversion Principle.
pub trait StartupRepository {
    /// Adds a new startup entry to the registry.
    fn add(&self, entry: &StartupEntry) -> Result<()>;

    /// Removes a startup entry from the registry by name.
    fn remove(&self, name: &str) -> Result<()>;

    /// Lists all startup entries from the registry.
    fn list(&self) -> Result<Vec<StartupEntry>>;

    /// Checks if an entry with the given name exists.
    #[allow(dead_code)]
    fn exists(&self, name: &str) -> Result<bool>;
}
