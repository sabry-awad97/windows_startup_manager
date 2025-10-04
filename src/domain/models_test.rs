#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_startup_entry_creation() {
        let entry = StartupEntry::new("TestApp", "C:\\test.exe");

        assert_eq!(entry.name, "TestApp");
        assert_eq!(entry.command, "C:\\test.exe");
    }

    #[test]
    fn test_startup_entry_with_string_types() {
        let name = String::from("MyApp");
        let command = String::from("cmd.exe");
        let entry = StartupEntry::new(name.clone(), command.clone());

        assert_eq!(entry.name, "MyApp");
        assert_eq!(entry.command, "cmd.exe");
    }

    #[test]
    fn test_execution_mode_default() {
        let mode = ExecutionMode::default();
        assert_eq!(mode, ExecutionMode::VBScript);
    }

    #[test]
    fn test_startup_command_vbscript_with_workdir() {
        let command = StartupCommand::CommandWithArgs {
            command: "bun".to_string(),
            args: vec!["run".to_string(), "dev".to_string()],
            workdir: Some("C:\\projects\\app".to_string()),
            mode: ExecutionMode::VBScript,
        };

        let registry_value = command.to_registry_value();

        assert!(registry_value.contains("wscript.exe"));
        assert!(registry_value.contains("//B"));
        assert!(registry_value.contains("//Nologo"));
        assert!(registry_value.contains("%APPDATA%"));
    }

    #[test]
    fn test_startup_command_vbscript_without_workdir() {
        let command = StartupCommand::CommandWithArgs {
            command: "notepad".to_string(),
            args: vec![],
            workdir: None,
            mode: ExecutionMode::VBScript,
        };

        let registry_value = command.to_registry_value();

        assert!(registry_value.contains("wscript.exe"));
        assert!(registry_value.contains("//B"));
        assert!(registry_value.contains("%APPDATA%"));
    }

    #[test]
    fn test_startup_command_powershell_hidden() {
        let command = StartupCommand::CommandWithArgs {
            command: "python".to_string(),
            args: vec!["-m".to_string(), "http.server".to_string()],
            workdir: Some("C:\\www".to_string()),
            mode: ExecutionMode::PowerShellHidden,
        };

        let registry_value = command.to_registry_value();

        assert!(registry_value.contains("powershell.exe"));
        assert!(registry_value.contains("-WindowStyle Hidden"));
        assert!(registry_value.contains("-NoProfile"));
        assert!(registry_value.contains("Set-Location"));
        assert!(registry_value.contains("C:\\www"));
        assert!(registry_value.contains("python -m http.server"));
    }

    #[test]
    fn test_startup_command_visible_mode() {
        let command = StartupCommand::CommandWithArgs {
            command: "cmd".to_string(),
            args: vec!["/c".to_string(), "echo test".to_string()],
            workdir: Some("C:\\temp".to_string()),
            mode: ExecutionMode::Visible,
        };

        let registry_value = command.to_registry_value();

        assert!(registry_value.contains("cmd.exe /c"));
        assert!(registry_value.contains("cd /d"));
        assert!(registry_value.contains("C:\\temp"));
    }

    #[test]
    fn test_startup_command_args_joining() {
        let command = StartupCommand::CommandWithArgs {
            command: "bun".to_string(),
            args: vec![
                "run".to_string(),
                "dev".to_string(),
                "--port".to_string(),
                "3000".to_string(),
            ],
            workdir: None,
            mode: ExecutionMode::VBScript,
        };

        let registry_value = command.to_registry_value();

        // VBScript wraps the command, so check it's in the generated VBScript content
        assert!(registry_value.contains("wscript.exe"));

        // Check the VBScript content contains the full command
        let vbs_content = command.get_vbscript_content();
        assert!(vbs_content.is_some());
        let (_, content) = vbs_content.unwrap();
        assert!(content.contains("bun run dev --port 3000"));
    }

    #[test]
    fn test_startup_command_empty_args() {
        let command = StartupCommand::CommandWithArgs {
            command: "notepad.exe".to_string(),
            args: vec![],
            workdir: None,
            mode: ExecutionMode::VBScript,
        };

        let registry_value = command.to_registry_value();

        // VBScript wraps the command
        assert!(registry_value.contains("wscript.exe"));

        // Check VBScript content has the command
        let vbs_content = command.get_vbscript_content();
        assert!(vbs_content.is_some());
        let (_, content) = vbs_content.unwrap();
        assert!(content.contains("notepad.exe"));
    }

    #[test]
    fn test_vbscript_content_generation() {
        let command = StartupCommand::CommandWithArgs {
            command: "test".to_string(),
            args: vec!["arg1".to_string()],
            workdir: Some("C:\\test".to_string()),
            mode: ExecutionMode::VBScript,
        };

        let result = command.get_vbscript_content();

        assert!(result.is_some());
        let (filename, content) = result.unwrap();

        assert!(filename.starts_with("launcher_"));
        assert!(filename.ends_with(".vbs"));
        assert!(content.contains("WScript.Shell"));
        assert!(content.contains("CurrentDirectory"));
        assert!(content.contains("C:\\test"));
        assert!(content.contains("test arg1"));
    }

    #[test]
    fn test_vbscript_content_not_generated_for_other_modes() {
        let command = StartupCommand::CommandWithArgs {
            command: "test".to_string(),
            args: vec![],
            workdir: None,
            mode: ExecutionMode::PowerShellHidden,
        };

        let result = command.get_vbscript_content();
        assert!(result.is_none());
    }

    #[test]
    fn test_executable_variant() {
        let command = StartupCommand::Executable {
            path: "C:\\Program Files\\App\\app.exe".to_string(),
        };

        let registry_value = command.to_registry_value();
        assert_eq!(registry_value, "C:\\Program Files\\App\\app.exe");
    }

    #[test]
    fn test_special_characters_in_command() {
        let command = StartupCommand::CommandWithArgs {
            command: "echo".to_string(),
            args: vec!["Hello \"World\"".to_string()],
            workdir: None,
            mode: ExecutionMode::VBScript,
        };

        let result = command.get_vbscript_content();
        assert!(result.is_some());

        let (_, content) = result.unwrap();
        // VBScript should escape quotes
        assert!(content.contains("\"\""));
    }
}
