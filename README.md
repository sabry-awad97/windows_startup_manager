# Windows Startup Manager

A professional command-line tool to manage Windows startup programs via the registry with a beautiful, colorful interface.

## Features

### **Core Functionality**
- ✅ **Add** programs to Windows startup
- ✅ **Add commands** with arguments (e.g., `bun run dev`, `python -m uvicorn`)
- ✅ **Working directory** support for commands
- ✅ **Remove** programs from Windows startup
- ✅ **List** all startup programs with **running status** (shows PIDs)
- ✅ **Kill** specific process by entry name
- ✅ **Kill all** processes from startup entries

### **User Experience**
- 🎨 **Colorful terminal output** for better readability
- 📊 **Status indicators** - see which processes are running
- 🔍 **Process monitoring** - displays PIDs of running processes
- ⚡ **Silent execution** - VBScript wrappers for truly invisible background processes
- 🛡️ **No admin required** - uses `HKEY_CURRENT_USER`
- ✅ **Path validation** before adding entries
- 🎯 **Clean error messages** with contextual information

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

This will display all programs currently configured to run on Windows startup, along with their running status.

**Example output (with colors):**

```
Current startup programs:
═════════════════════════════════════════════════

[1] BunDevServer
  Command: wscript.exe //B //Nologo "%APPDATA%\windows_startup_manager\launcher_abc123.vbs"
  Status: ✓ Running (1 process(es))
    PID: 12345

[2] MyApp
  Command: C:\Program Files\MyApp\myapp.exe
  Status: ○ Not running

──────────────────────────────────────────────────
Total: 2 entries
```

**Color scheme:**
- 🟢 **Green** - Success, running processes
- 🔵 **Cyan** - Entry names, counts
- 🟡 **Yellow** - PIDs, file paths
- ⚪ **White** - Commands, content
- ⚫ **Gray** - Inactive items, separators
- 🔴 **Red** - Errors

### Kill a Running Process

Kill a specific startup entry's running process:

```bash
windows_startup_manager kill "BunDevServer"
```

**Example output:**
```
✓ Killed 1 process(es) for 'BunDevServer'
```

### Kill All Running Processes

Kill all processes associated with startup entries:

```bash
windows_startup_manager kill-all
```

**Example output:**
```
✓ Killed processes:
  - wscript.exe: 2 process(es)
  - bun.exe: 1 process(es)

Total: 3 process(es) killed
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
- **[colored](https://crates.io/crates/colored)** - Beautiful terminal colors

## Error Handling

The tool provides clear error messages for common scenarios:

- **Path doesn't exist**: When adding a program, the tool validates that the specified path exists
- **Working directory doesn't exist**: When adding a command with `-d`, validates the directory exists
- **Entry not found**: When removing a non-existent entry
- **Registry access issues**: If the registry key cannot be opened or modified

## Important Notes

### Command Execution

#### **Silent Background Execution (Default)**
Commands use **VBScript wrappers** for the most reliable silent execution:
- ✅ **Zero window flash** - truly invisible
- ✅ Automatic VBScript file creation in `%APPDATA%\windows_startup_manager\`
- ✅ Works on all Windows versions
- ✅ Low resource overhead (~5-10 MB)

#### **How It Works**
1. Tool creates a VBScript file (e.g., `launcher_abc123.vbs`)
2. VBScript executes your command with hidden window
3. Registry points to: `wscript.exe //B //Nologo "path\to\launcher.vbs"`
4. No visible windows appear when Windows starts

#### **Alternative Methods**
See `docs/WINDOWS_EXECUTION_METHODS.md` for:
- PowerShell hidden window mode
- Task Scheduler integration
- Windows Services
- Native Win32 API approaches

### Best Practices for Development Servers
1. **Use absolute paths** for working directories
2. **Test commands manually** before adding to startup
3. **Monitor background processes**: Use Task Manager to verify servers are running
4. **Check logs**: Since output is hidden, redirect to log files for debugging:
   ```bash
   # PowerShell will handle redirection
   windows_startup_manager add-command "BunServer" bun run dev '>' server.log '2>&1' -d "C:\projects\app"
   ```
5. **Remove when not needed**: Development servers on startup can slow down boot time
6. **Kill processes**: Use Task Manager or `taskkill` to stop background servers

### Security Considerations
- Only add trusted commands to startup
- Validate all paths and commands before adding
- Commands run with your user privileges (not elevated)

## Visual Design

### **Colorful Terminal Output**

This tool uses the `colored` crate to provide a professional, visually appealing interface:

#### **Color Scheme**
- **Green (✓)** - Success messages, running processes
- **Red (✗)** - Error messages, failures
- **Cyan** - Entry names, primary identifiers
- **Yellow** - PIDs, file paths, counts
- **White** - Commands, general content
- **Gray (dimmed)** - Labels, metadata, separators
- **Bright Black** - Inactive items, disabled states

#### **Visual Elements**
- `═` Heavy separators for headers
- `─` Light separators for sections
- `•` Bullet points for lists
- `✓` Success indicators
- `✗` Error indicators
- `○` Inactive/not running indicators
- `[1]` Numbered items

#### **Example Output**

```
✓ Successfully added BunDevServer to startup.
  Working directory: C:\projects\my-app
  Command: bun run dev

Current startup programs:
═════════════════════════════════════════════════

[1] BunDevServer
  Command: wscript.exe //B //Nologo "..."
  Status: ✓ Running (1 process(es))
    PID: 12345

──────────────────────────────────────────────────
Total: 1 entries
```

For more details on the colored implementation, see [`docs/COLORED_GUIDE.md`](docs/COLORED_GUIDE.md).

---

## Architecture

This project follows **Domain-Driven Design (DDD)** and **SOLID principles**:

```
src/
├── domain/          # Business logic (models, repository trait, validators)
├── application/     # Use cases (add, remove, list, kill operations)
├── infrastructure/  # Windows Registry & Process management
├── interfaces/      # CLI and presentation layer
└── shared/          # Error types and utilities
```

For detailed architecture documentation, see [`ARCHITECTURE.md`](ARCHITECTURE.md).

---

## Documentation

- **[README.md](README.md)** - This file (getting started guide)
- **[QUICK_START.md](QUICK_START.md)** - Quick reference for common commands
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Technical architecture and design patterns
- **[docs/WINDOWS_EXECUTION_METHODS.md](docs/WINDOWS_EXECUTION_METHODS.md)** - Windows silent execution deep dive
- **[docs/COLORED_GUIDE.md](docs/COLORED_GUIDE.md)** - Colored crate implementation guide

---

## Safety & Security

### **Safety**
- ✅ Only modifies the current user's startup entries
- ✅ Validates file paths before adding entries
- ✅ Provides clear, colored error messages
- ✅ No system-wide changes (no admin required)
- ✅ VBScript files stored in user's `%APPDATA%` directory

### **Security Considerations**
- Only add trusted commands to startup
- Validate all paths and commands before adding
- Commands run with your user privileges (not elevated)
- VBScript files are user-specific and isolated
- Process management uses Windows built-in tools (`wmic`, `taskkill`)

---

## Troubleshooting

### **Colors not showing?**
On Windows, ANSI colors should work by default on Windows 10+. If colors aren't showing:
- Ensure you're using Windows Terminal or a modern terminal
- Update to the latest version of PowerShell
- Check if your terminal supports ANSI escape codes

### **Process not starting on boot?**
- Verify the command works manually first
- Check the working directory exists
- Use `list` to verify the entry was added correctly
- Check Windows Event Viewer for startup errors

### **Can't kill a process?**
- Verify the process is actually running with `list`
- Check Task Manager for the actual process name
- Use `taskkill /F /IM <process>.exe` as a fallback
- Some processes may require admin privileges to kill

---

## Contributing

Contributions are welcome! Please feel free to:
- Report bugs via GitHub Issues
- Suggest new features
- Submit Pull Requests
- Improve documentation

### **Development Setup**
```bash
git clone <repository-url>
cd windows_startup_manager
cargo build
cargo test
cargo clippy -- -D warnings
```

---

## License

MIT License - See LICENSE file for details

---

## Acknowledgments

Built with:
- **Rust** - Systems programming language
- **clap** - Command-line argument parsing
- **winreg** - Windows Registry access
- **colored** - Terminal colors
- **Domain-Driven Design** principles
- **SOLID** design patterns

---

**Made with ❤️ for Windows developers who want beautiful, functional CLI tools**
