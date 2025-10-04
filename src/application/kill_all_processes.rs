use crate::domain::StartupRepository;
use crate::infrastructure::ProcessManager;
use crate::shared::error::Result;
use std::collections::HashSet;

/// Use case for killing all processes associated with startup entries.
pub struct KillAllProcessesUseCase<'a, R: StartupRepository> {
    repository: &'a R,
}

impl<'a, R: StartupRepository> KillAllProcessesUseCase<'a, R> {
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub fn execute(&self) -> Result<Vec<(String, u32)>> {
        // Get all entries from repository
        let entries = self.repository.list()?;

        // Extract unique executable names
        let mut exe_names = HashSet::new();
        for entry in &entries {
            if let Some(exe_name) = ProcessManager::extract_executable_name(&entry.command) {
                exe_names.insert(exe_name);
            }
        }

        // Kill processes for each executable
        let mut results = Vec::new();
        for exe_name in exe_names {
            match ProcessManager::kill_processes_by_name(&exe_name) {
                Ok(count) => {
                    if count > 0 {
                        results.push((exe_name, count));
                    }
                }
                Err(_) => {
                    // Continue even if one fails
                    continue;
                }
            }
        }

        Ok(results)
    }
}
