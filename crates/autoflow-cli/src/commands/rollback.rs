use colored::*;

pub async fn run(sprint: Option<u32>) -> anyhow::Result<()> {
    println!("{}", "⏪ Rolling back...".bright_cyan().bold());
    if let Some(id) = sprint {
        println!("Sprint: {}", id);
    }
    println!("\n{}", "Command not yet implemented".yellow());
    Ok(())
}
