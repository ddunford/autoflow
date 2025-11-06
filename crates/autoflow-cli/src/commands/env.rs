use crate::EnvCommands;
use colored::*;

pub async fn run(cmd: EnvCommands) -> anyhow::Result<()> {
    println!("{}", "🐳 Environment".bright_cyan().bold());
    println!("Subcommand: {:?}", cmd);
    println!("\n{}", "Command not yet implemented".yellow());
    Ok(())
}
