use anyhow::Result;
use autoflow_utils::{
    check_binary_update, prompt_and_install_binary_update,
    check_for_updates, prompt_and_update, update_check_timestamp,
};
use colored::*;

pub async fn run(force: bool) -> Result<()> {
    println!("{}", "🔍 Checking for updates...".bright_cyan().bold());
    println!();

    let mut found_updates = false;

    // Check binary updates
    match check_binary_update().await {
        Ok(Some(binary_update)) => {
            found_updates = true;
            let updated = prompt_and_install_binary_update(&binary_update).await?;
            if updated {
                println!("{}", "⚠️  Binary updated! Please restart to use the new version.".bright_yellow().bold());
                println!();
                // Update timestamp so we don't check again immediately
                update_check_timestamp()?;
                return Ok(());
            }
        }
        Ok(None) => {
            println!("{}", "✓ Binary is up to date".green());
        }
        Err(e) => {
            println!("{}", format!("⚠ Failed to check for binary updates: {}", e).yellow());
            if force {
                eprintln!("Error details: {:?}", e);
            }
        }
    }

    // Check agent/skill updates
    match check_for_updates() {
        Ok(info) if info.has_updates() => {
            found_updates = true;
            println!();
            prompt_and_update(&info)?;
        }
        Ok(_) => {
            println!("{}", "✓ Agents and skills are up to date".green());
        }
        Err(e) => {
            println!("{}", format!("⚠ Failed to check for agent/skill updates: {}", e).yellow());
            if force {
                eprintln!("Error details: {:?}", e);
            }
        }
    }

    // Update check timestamp
    update_check_timestamp()?;

    if !found_updates {
        println!();
        println!("{}", "✅ Everything is up to date!".bright_green().bold());
        println!();
    }

    Ok(())
}
