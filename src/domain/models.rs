/// Represents a startup entry in the Windows registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartupEntry {
    pub name: String,
    pub command: String,
}

impl StartupEntry {
    /// Creates a new startup entry.
    pub fn new(name: impl Into<String>, command: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            command: command.into(),
        }
    }
}

/// Execution mode for background commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExecutionMode {
    /// Silent execution using VBScript (most reliable, no window flash).
    #[default]
    VBScript,
    /// Silent execution using PowerShell with hidden window.
    #[allow(dead_code)]
    PowerShellHidden,
    /// Visible window (for debugging).
    #[allow(dead_code)]
    Visible,
}

/// Represents different types of startup commands.
#[derive(Debug, Clone)]
pub enum StartupCommand {
    /// A simple executable path.
    #[allow(dead_code)]
    Executable { path: String },
    /// A command with arguments and optional working directory.
    CommandWithArgs {
        command: String,
        args: Vec<String>,
        workdir: Option<String>,
        mode: ExecutionMode,
    },
}

impl StartupCommand {
    /// Converts the command to a registry-compatible string based on execution mode.
    pub fn to_registry_value(&self) -> String {
        match self {
            StartupCommand::Executable { path } => path.clone(),
            StartupCommand::CommandWithArgs {
                command,
                args,
                workdir,
                mode,
            } => {
                let command_string = if args.is_empty() {
                    command.clone()
                } else {
                    format!("{} {}", command, args.join(" "))
                };

                match mode {
                    ExecutionMode::VBScript => {
                        // VBScript provides the most reliable silent execution
                        self.generate_vbscript_wrapper(&command_string, workdir.as_deref())
                    }
                    ExecutionMode::PowerShellHidden => {
                        // PowerShell with hidden window
                        if let Some(dir) = workdir {
                            format!(
                                "powershell.exe -WindowStyle Hidden -NoProfile -Command \"Set-Location '{}'; {}\"",
                                dir, command_string
                            )
                        } else {
                            format!(
                                "powershell.exe -WindowStyle Hidden -NoProfile -Command \"{}\"",
                                command_string
                            )
                        }
                    }
                    ExecutionMode::Visible => {
                        // Visible window for debugging
                        if let Some(dir) = workdir {
                            format!("cmd.exe /c \"cd /d \"{}\" && {}\"", dir, command_string)
                        } else {
                            command_string
                        }
                    }
                }
            }
        }
    }

    /// Generates a VBScript wrapper for truly silent execution.
    /// This is the most reliable method to avoid any window flash.
    fn generate_vbscript_wrapper(&self, command: &str, workdir: Option<&str>) -> String {
        let _vbs_content = if let Some(dir) = workdir {
            format!(
                "Set WshShell = CreateObject(\"WScript.Shell\")\n\
                 WshShell.CurrentDirectory = \"{}\"\n\
                 WshShell.Run \"{}\", 0, False",
                dir.replace("\"", "\"\""),
                command.replace("\"", "\"\"")
            )
        } else {
            format!(
                "Set WshShell = CreateObject(\"WScript.Shell\")\n\
                 WshShell.Run \"{}\", 0, False",
                command.replace("\"", "\"\"")
            )
        };

        // Create a unique VBScript filename based on command hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        command.hash(&mut hasher);
        let hash = hasher.finish();

        let vbs_path = format!(
            "%APPDATA%\\windows_startup_manager\\launcher_{:x}.vbs",
            hash
        );

        // Note: The VBScript file needs to be created separately
        // This returns the command to execute the VBScript
        format!("wscript.exe //B //Nologo \"{}\"", vbs_path)
    }

    /// Returns the VBScript content that needs to be written to disk.
    /// Only applicable for VBScript execution mode.
    pub fn get_vbscript_content(&self) -> Option<(String, String)> {
        match self {
            StartupCommand::CommandWithArgs {
                command,
                args,
                workdir,
                mode: ExecutionMode::VBScript,
            } => {
                let command_string = if args.is_empty() {
                    command.clone()
                } else {
                    format!("{} {}", command, args.join(" "))
                };

                let vbs_content = if let Some(dir) = workdir {
                    format!(
                        "Set WshShell = CreateObject(\"WScript.Shell\")\n\
                         WshShell.CurrentDirectory = \"{}\"\n\
                         WshShell.Run \"{}\", 0, False",
                        dir.replace("\"", "\"\""),
                        command_string.replace("\"", "\"\"")
                    )
                } else {
                    format!(
                        "Set WshShell = CreateObject(\"WScript.Shell\")\n\
                         WshShell.Run \"{}\", 0, False",
                        command_string.replace("\"", "\"\"")
                    )
                };

                // Generate filename
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};

                let mut hasher = DefaultHasher::new();
                command_string.hash(&mut hasher);
                let hash = hasher.finish();

                let filename = format!("launcher_{:x}.vbs", hash);

                Some((filename, vbs_content))
            }
            _ => None,
        }
    }
}
