use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::ffi::OsString;
use std::path::Path;
use winreg::RegKey;
use winreg::enums::*;

/// A simple command-line tool to manage Windows startup programs via the registry.
#[derive(Parser)]
#[command(name = "startup")]
#[command(about = "Manages programs that run on Windows startup.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds a program to the startup list.
    Add {
        /// The name of the entry in the startup registry.
        name: String,
        /// The full path to the executable file to run.
        path: String,
    },
    /// Removes a program from the startup list.
    Remove {
        /// The name of the entry to remove from the startup registry.
        name: String,
    },
    /// Lists all programs currently in the startup list.
    List,
}

/// Manages startup entries in the Windows Registry.
struct StartupManager {
    key: RegKey,
}

impl StartupManager {
    /// Creates a new StartupManager.
    /// Opens the registry key for startup programs for the current user.
    /// This does not require administrator privileges.
    fn new() -> Result<Self> {
        let path = r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run";
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let key = hkcu
            .open_subkey_with_flags(path, KEY_ALL_ACCESS)
            .with_context(|| format!("Failed to open startup registry key: {}", path))?;
        Ok(StartupManager { key })
    }

    /// Adds a new entry to the startup registry.
    fn add(&self, name: &str, path: &str) -> Result<()> {
        // Optional: Validate that the path exists before adding it.
        if !Path::new(path).exists() {
            return Err(anyhow::anyhow!(
                "The specified path does not exist: {}",
                path
            ));
        }

        self.key
            .set_value(name, &OsString::from(path))
            .with_context(|| {
                format!(
                    "Failed to set value '{}' with path '{}' in the registry",
                    name, path
                )
            })?;
        println!("Successfully added '{}' to startup.", name);
        Ok(())
    }

    /// Removes an entry from the startup registry.
    fn remove(&self, name: &str) -> Result<()> {
        self.key
            .delete_value(name)
            .with_context(|| format!("Failed to remove entry '{}' from the registry.", name))?;
        println!("Successfully removed '{}' from startup.", name);
        Ok(())
    }

    /// Lists all entries in the startup registry.
    fn list(&self) -> Result<()> {
        println!("Current startup programs:");
        println!("-------------------------");

        let mut has_entries = false;
        for item in self.key.enum_values() {
            let (name, value) = item.with_context(|| "Failed to enumerate registry values")?;
            has_entries = true;
            // The value type is REG_SZ, which is a string.
            let path = value.to_string();
            println!("  Name: {}\n  Path: {}\n", name, path);
        }

        if !has_entries {
            println!("  No startup programs found.");
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize the manager. If this fails, we can't do anything.
    let manager = StartupManager::new()?;

    match cli.command {
        Commands::Add { name, path } => {
            manager.add(&name, &path)?;
        }
        Commands::Remove { name } => {
            manager.remove(&name)?;
        }
        Commands::List => {
            manager.list()?;
        }
    }

    Ok(())
}
