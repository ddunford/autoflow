use crate::SprintsCommands;
use colored::*;

pub async fn run(cmd: SprintsCommands) -> anyhow::Result<()> {
    println!("{}", "📋 Sprints".bright_cyan().bold());
    println!("Subcommand: {:?}", cmd);
    println!("\n{}", "Command not yet implemented".yellow());
    Ok(())
}
