# Auto-Update Implementation Plan

## Current State
- ✅ GitHub Actions workflow exists for releases (`.github/workflows/release.yml`)
- ✅ Releases triggered on git tags (`v*`)
- ✅ Builds Linux x86_64 binary and creates tar.gz with agents/skills/schemas
- ⚠️ Auto-update only checks agents/skills, not binary
- ❌ No binary version checking against GitHub releases

## Required Changes

### 1. Add Version to Binary (HIGH PRIORITY)
**File**: `crates/autoflow-cli/Cargo.toml`
- Set semantic version (e.g., `0.1.0`)
- This gets embedded in binary via Cargo

**File**: `crates/autoflow-cli/src/main.rs`
- Add `--version` flag using `env!("CARGO_PKG_VERSION")`

### 2. Implement Binary Update Checker
**File**: `crates/autoflow-utils/src/update.rs`

Add new functions:
```rust
/// Check GitHub releases for binary updates
pub async fn check_binary_updates() -> Result<Option<BinaryUpdate>> {
    let current_version = env!("CARGO_PKG_VERSION");
    let repo = "anthropics/autoflow"; // or your repo

    // Fetch latest release from GitHub API
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    let response = reqwest::get(&url).await?;
    let release: GitHubRelease = response.json().await?;

    // Compare versions
    if version_is_newer(&release.tag_name, current_version) {
        return Ok(Some(BinaryUpdate {
            version: release.tag_name,
            download_url: get_download_url_for_platform(&release.assets)?,
            changelog: release.body,
        }));
    }

    Ok(None)
}

/// Download and install new binary
pub async fn install_binary_update(update: &BinaryUpdate) -> Result<()> {
    let home = dirs::home_dir()?;
    let bin_path = home.join(".autoflow/bin/autoflow");
    let tmp_path = home.join(".autoflow/bin/autoflow.new");

    // Download new binary
    let response = reqwest::get(&update.download_url).await?;
    let bytes = response.bytes().await?;

    // Extract if tar.gz, write binary
    // ...

    // Atomic replace
    fs::rename(&tmp_path, &bin_path)?;

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&bin_path, fs::Permissions::from_mode(0o755))?;
    }

    Ok(())
}
```

### 3. Update Start Command
**File**: `crates/autoflow-cli/src/commands/start.rs`

Modify the update check section:
```rust
// Check for updates (if enabled and interval has passed)
if should_check_for_updates().unwrap_or(false) {
    // Check binary updates first
    match check_binary_updates().await {
        Ok(Some(binary_update)) => {
            prompt_and_update_binary(&binary_update).await?;
        }
        _ => {}
    }

    // Then check agents/skills
    match check_for_updates() {
        Ok(info) if info.has_updates() => {
            prompt_and_update(&info)?;
        }
        _ => {}
    }

    update_check_timestamp()?;
}
```

### 4. Add Dependencies
**File**: `crates/autoflow-utils/Cargo.toml`
```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
semver = "1.0"
tokio = { version = "1", features = ["full"] }
```

### 5. Release Process
1. Update version in `crates/autoflow-cli/Cargo.toml`
2. Update CHANGELOG.md
3. Commit: `git commit -m "Release v0.2.0"`
4. Tag: `git tag v0.2.0`
5. Push: `git push && git push --tags`
6. GitHub Actions automatically:
   - Builds binaries for all platforms
   - Creates GitHub release
   - Uploads artifacts
7. Users automatically get update prompt on next run

## Benefits
✅ Users always have latest version
✅ Bug fixes deployed automatically
✅ Agents/skills/schemas stay in sync with binary
✅ No manual installation steps
✅ Works across all platforms

## Timeline
- Binary version checking: 2-3 hours
- Download/install logic: 3-4 hours
- Testing: 2 hours
- Documentation: 1 hour
**Total**: ~1 day of work

## Alternative: Quick Fix for Now
For immediate use, users can:
```bash
cd /opt/workspaces/autoflow
git pull
cargo build --release
cp target/release/autoflow ~/.autoflow/bin/autoflow
```

Or add to `.bashrc`:
```bash
alias autoflow-update='cd /opt/workspaces/autoflow && git pull && cargo build --release && cp target/release/autoflow ~/.autoflow/bin/autoflow'
```
