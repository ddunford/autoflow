use crate::WorktreeCommands;
use colored::*;

pub async fn run(cmd: WorktreeCommands) -> anyhow::Result<()> {
    println!("{}", "🌲 Worktree command".bright_cyan().bold());
    println!("Subcommand: {:?}", cmd);
    println!("\n{}", "Command not yet implemented".yellow());
    Ok(())
}
