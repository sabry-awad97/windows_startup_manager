use clap::{Parser, Subcommand};

/// A simple command-line tool to manage Windows startup programs via the registry.
#[derive(Parser)]
#[command(name = "startup")]
#[command(about = "Manages programs that run on Windows startup.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adds a program to the startup list.
    Add {
        /// The name of the entry in the startup registry.
        name: String,
        /// The full path to the executable file to run.
        path: String,
    },
    /// Adds a command with arguments to the startup list (e.g., "bun run dev").
    AddCommand {
        /// The name of the entry in the startup registry.
        name: String,
        /// The command to execute (e.g., "bun").
        command: String,
        /// Arguments for the command (e.g., "run dev").
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
        /// Optional working directory where the command should run.
        /// If not specified, uses the current working directory.
        #[arg(short = 'd', long)]
        workdir: Option<String>,
    },
    /// Removes a program from the startup list.
    Remove {
        /// The name of the entry to remove from the startup registry.
        name: String,
    },
    /// Lists all programs currently in the startup list.
    List,
    /// Kills a running process associated with a startup entry.
    Kill {
        /// The name of the startup entry whose process to kill.
        name: String,
    },
    /// Kills all running processes associated with startup entries.
    KillAll,
}
