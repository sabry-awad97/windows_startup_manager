# Windows Startup Manager

A simple and efficient command-line tool to manage Windows startup programs via the registry.

## Features

- ✅ **Add** programs to Windows startup
- ✅ **Remove** programs from Windows startup
- ✅ **List** all current startup programs
- ✅ No administrator privileges required (uses `HKEY_CURRENT_USER`)
- ✅ Path validation before adding entries
- ✅ Clean error handling with contextual messages

## Installation

### Prerequisites

- Rust toolchain (1.70 or later recommended)
- Windows operating system

### Build from Source

```bash
git clone <repository-url>
cd windows_startup_manager
cargo build --release
cargo install --path .
```

The compiled binary will be available at `target/release/windows_startup_manager.exe`

## Usage

### Add a Program to Startup

```bash
windows_startup_manager add <name> <path>
```

**Example:**

```bash
windows_startup_manager add "MyApp" "C:\Program Files\MyApp\myapp.exe"
```

This will add an entry named "MyApp" that runs the specified executable on Windows startup.

### Remove a Program from Startup

```bash
windows_startup_manager remove <name>
```

**Example:**

```bash
windows_startup_manager remove "MyApp"
```

This will remove the "MyApp" entry from the startup registry.

### List All Startup Programs

```bash
windows_startup_manager list
```

This will display all programs currently configured to run on Windows startup.

**Example output:**

```
Current startup programs:
-------------------------
  Name: MyApp
  Path: C:\Program Files\MyApp\myapp.exe

  Name: AnotherApp
  Path: C:\Tools\another.exe
```

## How It Works

The tool manages startup programs by modifying the Windows Registry at:

```
HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Run
```

This registry key contains programs that run automatically when the current user logs in. Since it uses `HKEY_CURRENT_USER`, no administrator privileges are required.

## Dependencies

- **[clap](https://crates.io/crates/clap)** - Command-line argument parsing
- **[winreg](https://crates.io/crates/winreg)** - Windows Registry access
- **[anyhow](https://crates.io/crates/anyhow)** - Error handling with context

## Error Handling

The tool provides clear error messages for common scenarios:

- **Path doesn't exist**: When adding a program, the tool validates that the specified path exists
- **Entry not found**: When removing a non-existent entry
- **Registry access issues**: If the registry key cannot be opened or modified

## Safety

- ✅ Only modifies the current user's startup entries
- ✅ Validates file paths before adding entries
- ✅ Provides clear error messages
- ✅ No system-wide changes (no admin required)

## License

[Add your license here]

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Author

[Add your information here]
