use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

const GITHUB_REPO: &str = "anthropics/autoflow"; // Update with actual repo
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub assets: Vec<GitHubAsset>,
    pub prerelease: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct BinaryUpdate {
    pub version: String,
    pub download_url: String,
    pub changelog: String,
    pub asset_name: String,
}

/// Check if a binary update is available
pub async fn check_binary_update() -> Result<Option<BinaryUpdate>> {
    tracing::debug!("Checking for binary updates...");
    tracing::debug!("Current version: {}", CURRENT_VERSION);

    // Fetch latest release from GitHub API
    let url = format!("https://api.github.com/repos/{}/releases/latest", GITHUB_REPO);

    let client = reqwest::Client::builder()
        .user_agent("autoflow-cli")
        .build()?;

    let response = client.get(&url)
        .send()
        .await
        .context("Failed to fetch release info from GitHub")?;

    if !response.status().is_success() {
        tracing::warn!("GitHub API returned status: {}", response.status());
        return Ok(None);
    }

    let release: GitHubRelease = response
        .json()
        .await
        .context("Failed to parse GitHub release JSON")?;

    tracing::debug!("Latest release: {}", release.tag_name);

    // Skip prerelease versions
    if release.prerelease {
        tracing::debug!("Skipping prerelease version: {}", release.tag_name);
        return Ok(None);
    }

    // Compare versions
    let latest_version = release.tag_name.trim_start_matches('v');
    if !is_version_newer(latest_version, CURRENT_VERSION)? {
        tracing::debug!("Current version is up to date");
        return Ok(None);
    }

    // Find the appropriate asset for this platform
    let platform = get_platform_string();
    let asset = release.assets.iter()
        .find(|a| a.name.contains(&platform) && a.name.ends_with(".tar.gz"))
        .context(format!("No binary found for platform: {}", platform))?;

    tracing::info!("Update available: {} -> {}", CURRENT_VERSION, latest_version);

    Ok(Some(BinaryUpdate {
        version: release.tag_name.clone(),
        download_url: asset.browser_download_url.clone(),
        changelog: release.body.clone(),
        asset_name: asset.name.clone(),
    }))
}

/// Install a binary update
pub async fn install_binary_update(update: &BinaryUpdate) -> Result<()> {
    println!("{}", format!("📥 Downloading {} ...", update.asset_name).bright_cyan());

    let home = dirs::home_dir().context("Could not find home directory")?;
    let bin_dir = home.join(".autoflow/bin");
    let bin_path = bin_dir.join("autoflow");
    let tmp_archive = bin_dir.join(format!("autoflow-{}.tar.gz", update.version));
    let tmp_dir = bin_dir.join("tmp");
    let tmp_binary = tmp_dir.join("autoflow");

    // Create temp directory
    fs::create_dir_all(&tmp_dir)?;

    // Download the archive
    let client = reqwest::Client::builder()
        .user_agent("autoflow-cli")
        .build()?;

    let response = client.get(&update.download_url)
        .send()
        .await?;

    let bytes = response.bytes().await?;
    fs::write(&tmp_archive, &bytes)?;

    println!("{}", "📦 Extracting archive...".bright_cyan());

    // Extract tar.gz
    let tar_gz = fs::File::open(&tmp_archive)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(&tmp_dir)?;

    // Verify the binary exists in extracted files
    if !tmp_binary.exists() {
        anyhow::bail!("Binary not found in archive");
    }

    println!("{}", "🔄 Installing new version...".bright_cyan());

    // Make executable (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&tmp_binary)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&tmp_binary, perms)?;
    }

    // Atomic replace: rename old binary, move new one, delete old
    let backup_path = bin_dir.join("autoflow.backup");

    // Backup current binary
    if bin_path.exists() {
        fs::rename(&bin_path, &backup_path)?;
    }

    // Move new binary into place
    fs::rename(&tmp_binary, &bin_path)?;

    // Also update agents/skills/schemas if they exist in the archive
    let tmp_agents = tmp_dir.join("agents");
    if tmp_agents.exists() {
        let agents_dir = home.join(".autoflow/agents");
        fs::create_dir_all(&agents_dir)?;
        copy_dir_all(&tmp_agents, &agents_dir)?;
        println!("{}", "  ✓ Updated agents".green());
    }

    let tmp_skills = tmp_dir.join("skills");
    if tmp_skills.exists() {
        let skills_dir = home.join(".autoflow/skills");
        fs::create_dir_all(&skills_dir)?;
        copy_dir_all(&tmp_skills, &skills_dir)?;
        println!("{}", "  ✓ Updated skills".green());
    }

    let tmp_schemas = tmp_dir.join("schemas");
    if tmp_schemas.exists() {
        let schemas_dir = home.join(".autoflow/schemas");
        fs::create_dir_all(&schemas_dir)?;
        copy_dir_all(&tmp_schemas, &schemas_dir)?;
        println!("{}", "  ✓ Updated schemas".green());
    }

    // Clean up
    let _ = fs::remove_file(&tmp_archive);
    let _ = fs::remove_file(&backup_path);
    let _ = fs::remove_dir_all(&tmp_dir);

    println!();
    println!("{}", format!("✅ Successfully updated to version {}", update.version).bright_green().bold());
    println!();

    Ok(())
}

/// Prompt user to install binary update
pub async fn prompt_and_install_binary_update(update: &BinaryUpdate) -> Result<bool> {
    println!();
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", format!("  AutoFlow Update Available: {}", update.version).bright_yellow().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();
    println!("{}", format!("Current version: {}", CURRENT_VERSION).dimmed());
    println!("{}", format!("New version:     {}", update.version.trim_start_matches('v')).bright_green());
    println!();

    // Show changelog if available (first 5 lines)
    if !update.changelog.is_empty() {
        println!("{}", "What's new:".bright_cyan());
        for line in update.changelog.lines().take(5) {
            println!("  {}", line);
        }
        if update.changelog.lines().count() > 5 {
            println!("  {}", "...".dimmed());
        }
        println!();
    }

    print!("Update now? [Y/n/skip] ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "" | "y" | "yes" => {
            println!();
            install_binary_update(update).await?;
            Ok(true)
        }
        "skip" => {
            println!();
            println!("{}", "⏭ Skipped update check for 24 hours".bright_yellow());
            println!();
            Ok(false)
        }
        _ => {
            println!();
            println!("{}", "❌ Update cancelled".bright_yellow());
            println!();
            Ok(false)
        }
    }
}

/// Compare two semantic versions
/// Returns true if `new_version` is newer than `current_version`
fn is_version_newer(new_version: &str, current_version: &str) -> Result<bool> {
    let new = semver::Version::parse(new_version)
        .context(format!("Invalid version string: {}", new_version))?;
    let current = semver::Version::parse(current_version)
        .context(format!("Invalid version string: {}", current_version))?;

    Ok(new > current)
}

/// Get platform-specific string for asset matching
fn get_platform_string() -> String {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    return "x86_64-unknown-linux-gnu".to_string();

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    return "x86_64-apple-darwin".to_string();

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    return "aarch64-apple-darwin".to_string();

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    return "x86_64-pc-windows-msvc".to_string();

    #[cfg(not(any(
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "macos", target_arch = "x86_64"),
        all(target_os = "macos", target_arch = "aarch64"),
        all(target_os = "windows", target_arch = "x86_64")
    )))]
    panic!("Unsupported platform");
}

/// Recursively copy a directory
fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
