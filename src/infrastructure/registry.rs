use crate::domain::{StartupEntry, StartupRepository};
use crate::shared::error::{Result, StartupError};
use std::ffi::OsString;
use winreg::RegKey;
use winreg::enums::*;

/// Windows Registry implementation of the StartupRepository trait.
/// This follows the Dependency Inversion Principle by implementing the domain trait.
pub struct WindowsRegistryRepository {
    key: RegKey,
}

impl WindowsRegistryRepository {
    /// Creates a new Windows Registry repository.
    /// Opens the registry key for startup programs for the current user.
    pub fn new() -> Result<Self> {
        let path = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let key = hkcu
            .open_subkey_with_flags(path, KEY_ALL_ACCESS)
            .map_err(|e| {
                StartupError::RegistryError(format!(
                    "Failed to open startup registry key '{}': {}",
                    path, e
                ))
            })?;
        Ok(Self { key })
    }
}

impl StartupRepository for WindowsRegistryRepository {
    fn add(&self, entry: &StartupEntry) -> Result<()> {
        self.key
            .set_value(&entry.name, &OsString::from(&entry.command))
            .map_err(|e| {
                StartupError::RegistryError(format!(
                    "Failed to add entry '{}' to registry: {}",
                    entry.name, e
                ))
            })?;
        Ok(())
    }

    fn remove(&self, name: &str) -> Result<()> {
        self.key.delete_value(name).map_err(|e| {
            StartupError::RegistryError(format!(
                "Failed to remove entry '{}' from registry: {}",
                name, e
            ))
        })?;
        Ok(())
    }

    fn list(&self) -> Result<Vec<StartupEntry>> {
        let mut entries = Vec::new();

        for item in self.key.enum_values() {
            let (name, value) = item.map_err(|e| {
                StartupError::RegistryError(format!("Failed to enumerate registry values: {}", e))
            })?;

            let command = value.to_string();
            entries.push(StartupEntry::new(name, command));
        }

        Ok(entries)
    }

    fn exists(&self, name: &str) -> Result<bool> {
        match self.key.get_value::<String, _>(name) {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(StartupError::RegistryError(format!(
                "Failed to check if entry '{}' exists: {}",
                name, e
            ))),
        }
    }
}
