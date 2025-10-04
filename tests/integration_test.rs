use std::cell::RefCell;
use std::collections::HashMap;

// Mock repository for testing
struct MockRepository {
    entries: RefCell<HashMap<String, String>>,
}

impl MockRepository {
    fn new() -> Self {
        Self {
            entries: RefCell::new(HashMap::new()),
        }
    }

    fn with_entries(entries: Vec<(&str, &str)>) -> Self {
        let mut map = HashMap::new();
        for (name, command) in entries {
            map.insert(name.to_string(), command.to_string());
        }
        Self {
            entries: RefCell::new(map),
        }
    }
}

// Import the domain types
use windows_startup_manager::domain::{StartupEntry, StartupRepository};
use windows_startup_manager::shared::error::{Result, StartupError};

impl StartupRepository for MockRepository {
    fn add(&self, entry: &StartupEntry) -> Result<()> {
        self.entries
            .borrow_mut()
            .insert(entry.name.clone(), entry.command.clone());
        Ok(())
    }

    fn remove(&self, name: &str) -> Result<()> {
        if self.entries.borrow_mut().remove(name).is_some() {
            Ok(())
        } else {
            Err(StartupError::RegistryError(format!(
                "Entry '{}' not found",
                name
            )))
        }
    }

    fn list(&self) -> Result<Vec<StartupEntry>> {
        Ok(self
            .entries
            .borrow()
            .iter()
            .map(|(name, command)| StartupEntry::new(name.clone(), command.clone()))
            .collect())
    }

    fn exists(&self, name: &str) -> Result<bool> {
        Ok(self.entries.borrow().contains_key(name))
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use windows_startup_manager::application::*;
    use windows_startup_manager::domain::ExecutionMode;

    #[test]
    fn test_add_executable_use_case() {
        let repo = MockRepository::new();
        let use_case = AddExecutableUseCase::new(&repo);

        // Create a temp file for testing
        let temp_file = std::env::temp_dir().join("test_app.exe");
        std::fs::write(&temp_file, "test").unwrap();

        let result = use_case.execute("TestApp", temp_file.to_str().unwrap());

        // Clean up
        std::fs::remove_file(&temp_file).ok();

        assert!(result.is_ok());

        let entries = repo.list().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "TestApp");
    }

    #[test]
    fn test_add_executable_nonexistent_path() {
        let repo = MockRepository::new();
        let use_case = AddExecutableUseCase::new(&repo);

        let result = use_case.execute("TestApp", "C:\\nonexistent\\app.exe");

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StartupError::PathNotFound(_)));
    }

    #[test]
    fn test_add_executable_empty_name() {
        let repo = MockRepository::new();
        let use_case = AddExecutableUseCase::new(&repo);

        let temp_file = std::env::temp_dir().join("test_app2.exe");
        std::fs::write(&temp_file, "test").unwrap();

        let result = use_case.execute("", temp_file.to_str().unwrap());

        std::fs::remove_file(&temp_file).ok();

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StartupError::InvalidName(_)));
    }

    #[test]
    fn test_add_command_use_case() {
        let repo = MockRepository::new();
        let use_case = AddCommandUseCase::new(&repo);

        let temp_dir = std::env::temp_dir();

        let result = use_case.execute(
            "BunServer",
            "bun",
            vec!["run".to_string(), "dev".to_string()],
            Some(temp_dir.to_str().unwrap()),
            ExecutionMode::VBScript,
        );

        assert!(result.is_ok());

        let entries = repo.list().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "BunServer");
        assert!(entries[0].command.contains("wscript.exe"));
    }

    #[test]
    fn test_add_command_invalid_workdir() {
        let repo = MockRepository::new();
        let use_case = AddCommandUseCase::new(&repo);

        let result = use_case.execute(
            "TestServer",
            "test",
            vec![],
            Some("C:\\nonexistent\\directory"),
            ExecutionMode::VBScript,
        );

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            StartupError::DirectoryNotFound(_)
        ));
    }

    #[test]
    fn test_remove_entry_use_case() {
        let repo = MockRepository::with_entries(vec![("TestApp", "C:\\test.exe")]);
        let use_case = RemoveEntryUseCase::new(&repo);

        let result = use_case.execute("TestApp");

        assert!(result.is_ok());

        let entries = repo.list().unwrap();
        assert_eq!(entries.len(), 0);
    }

    #[test]
    fn test_remove_nonexistent_entry() {
        let repo = MockRepository::new();
        let use_case = RemoveEntryUseCase::new(&repo);

        let result = use_case.execute("NonExistent");

        assert!(result.is_err());
    }

    #[test]
    fn test_list_entries_use_case() {
        let repo = MockRepository::with_entries(vec![
            ("App1", "C:\\app1.exe"),
            ("App2", "C:\\app2.exe"),
            ("App3", "C:\\app3.exe"),
        ]);
        let use_case = ListEntriesUseCase::new(&repo);

        let result = use_case.execute();

        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 3);
    }

    #[test]
    fn test_list_entries_empty() {
        let repo = MockRepository::new();
        let use_case = ListEntriesUseCase::new(&repo);

        let result = use_case.execute();

        assert!(result.is_ok());
        let entries = result.unwrap();
        assert_eq!(entries.len(), 0);
    }

    #[test]
    fn test_multiple_operations() {
        let repo = MockRepository::new();

        // Add multiple entries
        let add_use_case = AddCommandUseCase::new(&repo);
        let temp_dir = std::env::temp_dir();

        add_use_case
            .execute(
                "Server1",
                "bun",
                vec!["start".to_string()],
                Some(temp_dir.to_str().unwrap()),
                ExecutionMode::VBScript,
            )
            .unwrap();

        add_use_case
            .execute(
                "Server2",
                "node",
                vec!["server.js".to_string()],
                Some(temp_dir.to_str().unwrap()),
                ExecutionMode::VBScript,
            )
            .unwrap();

        // List entries
        let list_use_case = ListEntriesUseCase::new(&repo);
        let entries = list_use_case.execute().unwrap();
        assert_eq!(entries.len(), 2);

        // Remove one entry
        let remove_use_case = RemoveEntryUseCase::new(&repo);
        remove_use_case.execute("Server1").unwrap();

        // Verify removal
        let entries = list_use_case.execute().unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "Server2");
    }

    #[test]
    fn test_repository_exists() {
        let repo = MockRepository::with_entries(vec![("ExistingApp", "C:\\app.exe")]);

        assert!(repo.exists("ExistingApp").unwrap());
        assert!(!repo.exists("NonExistentApp").unwrap());
    }

    #[test]
    fn test_add_duplicate_name() {
        let repo = MockRepository::with_entries(vec![("App", "C:\\old.exe")]);
        let use_case = AddCommandUseCase::new(&repo);
        let temp_dir = std::env::temp_dir();

        // Adding with same name should overwrite
        use_case
            .execute(
                "App",
                "new",
                vec![],
                Some(temp_dir.to_str().unwrap()),
                ExecutionMode::VBScript,
            )
            .unwrap();

        let entries = repo.list().unwrap();
        assert_eq!(entries.len(), 1);
        // VBScript mode wraps the command, so check for wscript instead
        assert!(entries[0].command.contains("wscript.exe"));
        assert!(!entries[0].command.contains("old.exe"));
    }
}
