use crate::domain::StartupEntry;
use crate::infrastructure::ProcessManager;
use colored::*;

/// Handles output presentation to the user.
pub struct ConsolePresenter;

impl ConsolePresenter {
    pub fn show_success_add(name: &str) {
        println!(
            "{} Successfully added {} to startup.",
            "✓".green().bold(),
            name.cyan().bold()
        );
    }

    pub fn show_success_add_command(name: &str, command: &str, workdir: Option<&str>) {
        println!(
            "{} Successfully added command {} to startup.",
            "✓".green().bold(),
            name.cyan().bold()
        );
        if let Some(dir) = workdir {
            println!("  {} {}", "Working directory:".dimmed(), dir.yellow());
        }
        println!("  {} {}", "Command:".dimmed(), command.white());
    }

    pub fn show_success_remove(name: &str) {
        println!(
            "{} Successfully removed {} from startup.",
            "✓".green().bold(),
            name.cyan().bold()
        );
    }

    pub fn show_entries(entries: &[StartupEntry]) {
        println!("\n{}", "Current startup programs:".bright_blue().bold());
        println!("{}", "═".repeat(50).bright_black());

        if entries.is_empty() {
            println!("  {}", "No startup programs found.".yellow());
        } else {
            // Get running processes
            let running_processes = ProcessManager::list_processes().unwrap_or_default();

            for (idx, entry) in entries.iter().enumerate() {
                println!(
                    "\n{} {}",
                    format!("[{}]", idx + 1).bright_black(),
                    entry.name.cyan().bold()
                );
                println!("  {} {}", "Command:".dimmed(), entry.command.white());

                // Check if process is running
                if let Some(exe_name) = ProcessManager::extract_executable_name(&entry.command) {
                    let matching_procs: Vec<_> = running_processes
                        .iter()
                        .filter(|p| p.name.eq_ignore_ascii_case(&exe_name))
                        .collect();

                    if !matching_procs.is_empty() {
                        println!(
                            "  {} {} {}",
                            "Status:".dimmed(),
                            "✓".green().bold(),
                            format!("Running ({} process(es))", matching_procs.len()).green()
                        );
                        for proc in matching_procs {
                            println!("    {} {}", "PID:".dimmed(), proc.pid.to_string().yellow());
                        }
                    } else {
                        println!(
                            "  {} {} {}",
                            "Status:".dimmed(),
                            "○".bright_black(),
                            "Not running".bright_black()
                        );
                    }
                }
            }
            println!("\n{}", "─".repeat(50).bright_black());
            println!(
                "{} {} entries",
                "Total:".dimmed(),
                entries.len().to_string().cyan().bold()
            );
        }
    }

    pub fn show_kill_success(name: &str, count: u32) {
        if count > 0 {
            println!(
                "{} Killed {} for {}",
                "✓".green().bold(),
                format!("{} process(es)", count).yellow(),
                name.cyan().bold()
            );
        } else {
            println!(
                "{} No running processes found for {}",
                "○".bright_black(),
                name.cyan()
            );
        }
    }

    pub fn show_kill_all_success(results: &[(String, u32)]) {
        if results.is_empty() {
            println!(
                "{} No running processes found for any startup entries",
                "○".bright_black()
            );
        } else {
            println!("{} Killed processes:", "✓".green().bold());
            let total: u32 = results.iter().map(|(_, count)| count).sum();
            for (exe_name, count) in results {
                println!(
                    "  {} {}: {}",
                    "•".bright_blue(),
                    exe_name.white(),
                    format!("{} process(es)", count).yellow()
                );
            }
            println!(
                "\n{} {} process(es) killed",
                "Total:".dimmed(),
                total.to_string().green().bold()
            );
        }
    }

    pub fn show_error(error: &dyn std::error::Error) {
        eprintln!("{} {}", "✗".red().bold(), error.to_string().red());
    }
}
