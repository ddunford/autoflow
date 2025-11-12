use crate::EnvCommands;
use anyhow::{bail, Context};
use colored::*;
use std::path::Path;
use std::process::Command;

pub async fn run(cmd: EnvCommands) -> anyhow::Result<()> {
    println!("{}", "🐳 Environment".bright_cyan().bold());

    // Check if docker-compose.yml exists
    if !Path::new("docker-compose.yml").exists() {
        println!("\n{}", "⚠️  No docker-compose.yml found".yellow());
        println!("This project doesn't appear to have Docker configuration.");
        return Ok(());
    }

    match cmd {
        EnvCommands::Start => start_env().await,
        EnvCommands::Stop => stop_env().await,
        EnvCommands::Restart => restart_env().await,
        EnvCommands::Logs { follow } => logs_env(follow).await,
        EnvCommands::Health => health_check().await,
    }
}

async fn start_env() -> anyhow::Result<()> {
    println!("\n{}", "Starting Docker containers...".bright_cyan());

    let status = Command::new("docker-compose")
        .args(&["up", "-d"])
        .status()
        .context("Failed to execute docker-compose")?;

    if status.success() {
        println!("{} {}", "✅".green(), "Containers started successfully!".bright_green());
    } else {
        bail!("Failed to start containers");
    }

    Ok(())
}

async fn stop_env() -> anyhow::Result<()> {
    println!("\n{}", "Stopping Docker containers...".bright_cyan());

    let status = Command::new("docker-compose")
        .args(&["down"])
        .status()
        .context("Failed to execute docker-compose")?;

    if status.success() {
        println!("{} {}", "✅".green(), "Containers stopped successfully!".bright_green());
    } else {
        bail!("Failed to stop containers");
    }

    Ok(())
}

async fn restart_env() -> anyhow::Result<()> {
    println!("\n{}", "Restarting Docker containers...".bright_cyan());

    let status = Command::new("docker-compose")
        .args(&["restart"])
        .status()
        .context("Failed to execute docker-compose")?;

    if status.success() {
        println!("{} {}", "✅".green(), "Containers restarted successfully!".bright_green());
    } else {
        bail!("Failed to restart containers");
    }

    Ok(())
}

async fn logs_env(follow: bool) -> anyhow::Result<()> {
    println!("\n{}", "Viewing container logs...".bright_cyan());

    let mut args = vec!["logs"];
    if follow {
        args.push("-f");
    }

    let status = Command::new("docker-compose")
        .args(&args)
        .status()
        .context("Failed to execute docker-compose")?;

    if !status.success() {
        bail!("Failed to view logs");
    }

    Ok(())
}

async fn health_check() -> anyhow::Result<()> {
    println!("\n{}", "Checking container health...".bright_cyan());

    let output = Command::new("docker-compose")
        .args(&["ps"])
        .output()
        .context("Failed to execute docker-compose")?;

    if output.status.success() {
        println!("\n{}", String::from_utf8_lossy(&output.stdout));
        println!("{} {}", "✅".green(), "Health check complete".bright_green());
    } else {
        bail!("Failed to check health");
    }

    Ok(())
}
