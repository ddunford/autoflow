/// Auto-update system for AutoFlow
/// Checks GitHub releases for new versions and installs them automatically
use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

const GITHUB_REPO: &str = "ddunford/autoflow";
const UPDATE_CHECK_FILE: &str = ".last_update_check";

/// Check if auto-update is enabled (default: true)
pub fn is_auto_update_enabled() -> bool {
    // Check environment variable
    if let Ok(val) = env::var("AUTOFLOW_AUTO_UPDATE") {
        return val != "0" && val.to_lowercase() != "false";
    }

    // Check config file
    let home = env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let config_path = PathBuf::from(home).join(".autoflow/config.toml");

    if let Ok(content) = fs::read_to_string(config_path) {
        // Simple parsing - look for auto_update = false
        if content.contains("auto_update = false") {
            return false;
        }
    }

    true // Default to enabled
}

/// Check if we should check for updates (respects check_interval_hours)
pub fn should_check_for_updates() -> bool {
    if !is_auto_update_enabled() {
        return false;
    }

    let home = env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let check_file = PathBuf::from(home).join(".autoflow").join(UPDATE_CHECK_FILE);

    // If file doesn't exist, we should check
    if !check_file.exists() {
        return true;
    }

    // Check if file is older than 24 hours (configurable later)
    if let Ok(metadata) = fs::metadata(&check_file) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(elapsed) = modified.elapsed() {
                // Check every 24 hours by default
                return elapsed.as_secs() > 24 * 60 * 60;
            }
        }
    }

    true
}

/// Update the last check timestamp
fn update_check_timestamp() -> Result<()> {
    let home = env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let autoflow_dir = PathBuf::from(home).join(".autoflow");
    fs::create_dir_all(&autoflow_dir)?;

    let check_file = autoflow_dir.join(UPDATE_CHECK_FILE);
    fs::write(check_file, chrono::Utc::now().to_rfc3339())?;

    Ok(())
}

/// Check for updates and install if available
pub async fn check_and_update(verbose: bool) -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");

    if verbose {
        eprintln!("🔍 Checking for AutoFlow updates...");
        eprintln!("   Current version: {}", current_version);
    }

    // Get latest release from GitHub API
    let url = format!("https://api.github.com/repos/{}/releases/latest", GITHUB_REPO);
    let client = reqwest::Client::builder()
        .user_agent("autoflow-cli")
        .build()?;

    let response = client
        .get(&url)
        .send()
        .await
        .context("Failed to check for updates")?;

    if !response.status().is_success() {
        if verbose {
            eprintln!("   ⚠ Could not check for updates ({})", response.status());
        }
        update_check_timestamp()?;
        return Ok(());
    }

    let release: serde_json::Value = response.json().await?;
    let latest_version = release["tag_name"]
        .as_str()
        .unwrap_or("")
        .trim_start_matches('v');

    if verbose {
        eprintln!("   Latest version: {}", latest_version);
    }

    // Compare versions
    if latest_version <= current_version {
        if verbose {
            eprintln!("   ✓ You're up to date!");
        }
        update_check_timestamp()?;
        return Ok(());
    }

    eprintln!("");
    eprintln!("🎉 New version available: {} → {}", current_version, latest_version);
    eprintln!("   Installing update...");
    eprintln!("");

    // Download and install the new version
    install_update(&release, verbose).await?;

    update_check_timestamp()?;

    eprintln!("");
    eprintln!("✅ AutoFlow updated to version {}", latest_version);
    eprintln!("   Restart your command to use the new version");
    eprintln!("");

    Ok(())
}

async fn install_update(release: &serde_json::Value, verbose: bool) -> Result<()> {
    // Determine platform
    let platform = if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        anyhow::bail!("Unsupported platform for auto-update");
    };

    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        anyhow::bail!("Unsupported architecture for auto-update");
    };

    // Look for asset matching platform
    let asset_name = format!("autoflow-{}-{}", platform, arch);

    let assets = release["assets"]
        .as_array()
        .context("No assets in release")?;

    let asset = assets
        .iter()
        .find(|a| {
            a["name"]
                .as_str()
                .map(|name| name.contains(&asset_name))
                .unwrap_or(false)
        })
        .context("No binary found for your platform")?;

    let download_url = asset["browser_download_url"]
        .as_str()
        .context("No download URL")?;

    if verbose {
        eprintln!("   Downloading: {}", download_url);
    }

    // Download binary
    let client = reqwest::Client::builder()
        .user_agent("autoflow-cli")
        .build()?;

    let response = client.get(download_url).send().await?;
    let bytes = response.bytes().await?;

    // Get current binary path
    let current_exe = env::current_exe()?;
    let temp_path = current_exe.with_extension("new");

    // Write new binary to temp location
    fs::write(&temp_path, bytes)?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&temp_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&temp_path, perms)?;
    }

    // Replace current binary
    // On Windows, this won't work if the binary is running
    // On Unix, we can replace it while running
    let backup_path = current_exe.with_extension("backup");

    // Backup current binary
    if current_exe.exists() {
        fs::copy(&current_exe, &backup_path)?;
    }

    // Replace with new binary
    fs::rename(&temp_path, &current_exe)?;

    if verbose {
        eprintln!("   ✓ Binary updated");
    }

    Ok(())
}

/// Manual update command
pub async fn update_now(verbose: bool) -> Result<()> {
    eprintln!("🔄 Checking for updates...");
    check_and_update(verbose).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_update_enabled_default() {
        // Should be enabled by default if no config
        assert!(is_auto_update_enabled());
    }
}
