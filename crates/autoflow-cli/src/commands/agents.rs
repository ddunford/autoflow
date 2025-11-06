use colored::*;

pub async fn run(detailed: bool) -> anyhow::Result<()> {
    println!("{}", "🤖 Available Agents".bright_cyan().bold());

    if detailed {
        println!("Showing detailed information...");
    }

    println!("\n{}", "Command not yet implemented".yellow());
    Ok(())
}
