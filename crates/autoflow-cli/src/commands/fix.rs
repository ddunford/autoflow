use colored::*;

pub async fn run(description: String, _auto_fix: bool, _playwright_headed: bool) -> anyhow::Result<()> {
    println!("{}", "🐛 Investigating bug...".bright_cyan().bold());
    println!("Bug: {}", description.bright_blue());
    println!("\n{}", "Command not yet implemented".yellow());
    Ok(())
}
