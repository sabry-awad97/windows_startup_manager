use crate::shared::error::{Result, StartupError};
use std::process::Command;

/// Process information for a running startup entry.
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    #[allow(dead_code)]
    pub command_line: String,
}

/// Manages Windows processes.
pub struct ProcessManager;

impl ProcessManager {
    /// Lists all running processes.
    pub fn list_processes() -> Result<Vec<ProcessInfo>> {
        let output = Command::new("wmic")
            .args([
                "process",
                "get",
                "ProcessId,Name,CommandLine",
                "/format:csv",
            ])
            .output()
            .map_err(|e| StartupError::RegistryError(format!("Failed to list processes: {}", e)))?;

        if !output.status.success() {
            return Err(StartupError::RegistryError(
                "Failed to execute wmic command".to_string(),
            ));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut processes = Vec::new();

        for line in output_str.lines().skip(2) {
            // Skip header lines
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 3
                && let Ok(pid) = parts[2].trim().parse::<u32>()
            {
                processes.push(ProcessInfo {
                    pid,
                    name: parts[1].trim().to_string(),
                    command_line: parts[0].trim().to_string(),
                });
            }
        }

        Ok(processes)
    }

    /// Kills a process by PID.
    #[allow(dead_code)]
    pub fn kill_process(pid: u32) -> Result<()> {
        let output = Command::new("taskkill")
            .args(["/F", "/PID", &pid.to_string()])
            .output()
            .map_err(|e| {
                StartupError::RegistryError(format!("Failed to kill process {}: {}", pid, e))
            })?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(StartupError::RegistryError(format!(
                "Failed to kill process {}: {}",
                pid, error_msg
            )));
        }

        Ok(())
    }

    /// Kills all processes matching a name.
    pub fn kill_processes_by_name(name: &str) -> Result<u32> {
        let output = Command::new("taskkill")
            .args(["/F", "/IM", name])
            .output()
            .map_err(|e| {
                StartupError::RegistryError(format!("Failed to kill processes {}: {}", name, e))
            })?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            // Check if it's just "not found" error
            if error_msg.contains("not found") {
                return Ok(0);
            }
            return Err(StartupError::RegistryError(format!(
                "Failed to kill processes {}: {}",
                name, error_msg
            )));
        }

        // Parse output to count killed processes
        let output_str = String::from_utf8_lossy(&output.stdout);
        let count = output_str.lines().filter(|l| l.contains("SUCCESS")).count() as u32;

        Ok(count)
    }

    /// Finds processes that match a command pattern.
    #[allow(dead_code)]
    pub fn find_processes_by_command(pattern: &str) -> Result<Vec<ProcessInfo>> {
        let all_processes = Self::list_processes()?;

        let matching = all_processes
            .into_iter()
            .filter(|p| {
                p.command_line
                    .to_lowercase()
                    .contains(&pattern.to_lowercase())
                    || p.name.to_lowercase().contains(&pattern.to_lowercase())
            })
            .collect();

        Ok(matching)
    }

    /// Extracts the executable name from a command string.
    pub fn extract_executable_name(command: &str) -> Option<String> {
        // Handle VBScript wrapper
        if command.contains("wscript.exe") {
            return Some("wscript.exe".to_string());
        }

        // Handle PowerShell
        if command.contains("powershell.exe") {
            // Try to extract the actual command being run
            if let Some(start) = command.find("Command \"") {
                let after_command = &command[start + 9..];
                if let Some(end) = after_command.find('"') {
                    let inner_command = &after_command[..end];
                    // Get first word (executable name)
                    if let Some(exe) = inner_command.split_whitespace().next() {
                        return Some(format!("{}.exe", exe));
                    }
                }
            }
            return Some("powershell.exe".to_string());
        }

        // Handle CMD
        if command.contains("cmd.exe") {
            return Some("cmd.exe".to_string());
        }

        // Direct executable
        if let Some(exe) = command.split_whitespace().next()
            && exe.ends_with(".exe")
        {
            return Some(exe.to_string());
        }

        None
    }
}
