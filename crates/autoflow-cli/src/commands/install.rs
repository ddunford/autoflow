use colored::*;
use std::path::PathBuf;
use tracing::{info, warn};

pub async fn run(force: bool) -> anyhow::Result<()> {
    println!("{}", "🚀 Installing AutoFlow...".bright_cyan().bold());

    let home = std::env::var("HOME")?;
    let autoflow_dir = PathBuf::from(&home).join(".autoflow");

    // Check if already installed
    if autoflow_dir.exists() && !force {
        warn!("AutoFlow is already installed at {}", autoflow_dir.display());
        println!("\n{}", "Use --force to reinstall".yellow());
        return Ok(());
    }

    info!("Installation directory: {}", autoflow_dir.display());

    println!("\n{}", "Installation coming soon!".yellow());
    println!("For now, run from source:");
    println!("  {}", "cargo build --release".bright_blue());
    println!("  {}", "cargo install --path crates/autoflow-cli".bright_blue());

    Ok(())
}
