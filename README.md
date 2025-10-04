# Windows Startup Manager

A simple and efficient command-line tool to manage Windows startup programs via the registry.

## Features

- ✅ **Add** programs to Windows startup
- ✅ **Add commands** with arguments (e.g., `bun run dev`, `python -m uvicorn`)
- ✅ **Working directory** support for commands
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

### Add a Command with Arguments to Startup

For development servers or scripts that require arguments and working directories:

```bash
windows_startup_manager add-command <name> <command> [args...] [-d <workdir>]
```

**Examples:**

**Bun development server:**
```bash
windows_startup_manager add-command "BunDevServer" bun run dev -d "C:\projects\my-app"
```

**Python FastAPI/Uvicorn server:**
```bash
windows_startup_manager add-command "FastAPIServer" python -m uvicorn main:app --reload -d "C:\projects\api"
```

**Node.js server:**
```bash
windows_startup_manager add-command "NodeServer" npm start -d "C:\projects\node-app"
```

**Python script with arguments:**
```bash
windows_startup_manager add-command "DataProcessor" python process.py --config prod.json -d "C:\scripts"
```

**Without working directory:**
```bash
windows_startup_manager add-command "GlobalCommand" bun run start
```

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
- **Working directory doesn't exist**: When adding a command with `-d`, validates the directory exists
- **Entry not found**: When removing a non-existent entry
- **Registry access issues**: If the registry key cannot be opened or modified

## Important Notes

### Command Execution
- Commands with working directories are wrapped in `cmd.exe /c "cd /d <dir> && <command>"`
- This ensures the command runs in the correct directory context
- Commands run in the background when Windows starts

### Best Practices for Development Servers
1. **Use absolute paths** for working directories
2. **Test commands manually** before adding to startup
3. **Consider logging**: Redirect output to log files for debugging
   ```bash
   windows_startup_manager add-command "BunServer" cmd /c "bun run dev > server.log 2>&1" -d "C:\projects\app"
   ```
4. **Remove when not needed**: Development servers on startup can slow down boot time

### Security Considerations
- Only add trusted commands to startup
- Validate all paths and commands before adding
- Commands run with your user privileges (not elevated)

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
