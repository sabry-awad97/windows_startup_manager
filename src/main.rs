mod application;
mod domain;
mod infrastructure;
mod interfaces;
mod shared;

use application::{
    AddCommandUseCase, AddExecutableUseCase, KillAllProcessesUseCase, KillProcessUseCase,
    ListEntriesUseCase, RemoveEntryUseCase,
};
use clap::Parser;
use infrastructure::WindowsRegistryRepository;
use interfaces::{Cli, Commands, ConsolePresenter};

fn main() {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize the repository (infrastructure layer)
    let repository = match WindowsRegistryRepository::new() {
        Ok(repo) => repo,
        Err(e) => {
            ConsolePresenter::show_error(&e);
            std::process::exit(1);
        }
    };

    // Execute the appropriate use case based on the command
    let result = match cli.command {
        Commands::Add { name, path } => {
            let use_case = AddExecutableUseCase::new(&repository);
            use_case.execute(&name, &path).map(|_| {
                ConsolePresenter::show_success_add(&name);
            })
        }
        Commands::AddCommand {
            name,
            command,
            args,
            workdir,
        } => {
            let use_case = AddCommandUseCase::new(&repository);
            let command_display = if args.is_empty() {
                command.clone()
            } else {
                format!("{} {}", command, args.join(" "))
            };

            // Use current working directory if not specified
            let working_dir = match workdir {
                Some(dir) => Some(dir),
                None => std::env::current_dir()
                    .ok()
                    .and_then(|p| p.to_str().map(|s| s.to_string())),
            };

            // Use VBScript mode by default for most reliable silent execution
            use domain::ExecutionMode;
            use_case
                .execute(
                    &name,
                    &command,
                    args,
                    working_dir.as_deref(),
                    ExecutionMode::VBScript,
                )
                .map(|_| {
                    ConsolePresenter::show_success_add_command(
                        &name,
                        &command_display,
                        working_dir.as_deref(),
                    );
                })
        }
        Commands::Remove { name } => {
            let use_case = RemoveEntryUseCase::new(&repository);
            use_case.execute(&name).map(|_| {
                ConsolePresenter::show_success_remove(&name);
            })
        }
        Commands::List => {
            let use_case = ListEntriesUseCase::new(&repository);
            use_case.execute().map(|entries| {
                ConsolePresenter::show_entries(&entries);
            })
        }
        Commands::Kill { name } => {
            let use_case = KillProcessUseCase::new(&repository);
            use_case.execute(&name).map(|count| {
                ConsolePresenter::show_kill_success(&name, count);
            })
        }
        Commands::KillAll => {
            let use_case = KillAllProcessesUseCase::new(&repository);
            use_case.execute().map(|results| {
                ConsolePresenter::show_kill_all_success(&results);
            })
        }
    };

    // Handle any errors
    if let Err(e) = result {
        ConsolePresenter::show_error(&e);
        std::process::exit(1);
    }
}
