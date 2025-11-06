use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct McpServerConfig {
    command: String,
    args: Option<Vec<String>>,
    env: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClaudeConfig {
    #[serde(rename = "mcpServers")]
    mcp_servers: std::collections::HashMap<String, McpServerConfig>,
}

pub async fn run_install(servers: Vec<String>, project_level: bool) -> Result<()> {
    println!("{}", "📦 Installing MCP Servers...".bright_cyan().bold());
    println!();

    let config_path = if project_level {
        // Project-level installation (Claude Code project scope)
        let project_config = PathBuf::from(".mcp.json");
        if !Path::new(".").exists() {
            anyhow::bail!(
                "Not in a valid directory."
            );
        }
        println!("  {} Installing to project: {}", "ℹ".blue(), ".mcp.json".bright_blue());
        println!("  {} (Claude Code project scope - can be committed to git)", "ℹ".blue());
        project_config
    } else {
        // Global installation (Claude Code user scope)
        let home = std::env::var("HOME")?;
        let claude_dir = PathBuf::from(&home).join(".claude");
        fs::create_dir_all(&claude_dir)?;

        let claude_config_path = claude_dir.join("settings.local.json");
        println!("  {} Installing globally: {}", "ℹ".blue(), claude_config_path.display().to_string().bright_blue());
        println!("  {} (Claude Code user scope - available to all your projects)", "ℹ".blue());
        claude_config_path
    };

    let config_path_clone = config_path.clone();

    // Load existing config or create new
    let mut config: ClaudeConfig = if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;

        // Show what's already configured
        if let Ok(existing) = serde_json::from_str::<ClaudeConfig>(&content) {
            if !existing.mcp_servers.is_empty() {
                println!("  {} Found {} existing MCP servers",
                    "ℹ".blue(),
                    existing.mcp_servers.len().to_string().bright_blue()
                );
                for server_name in existing.mcp_servers.keys() {
                    println!("    • {}", server_name.bright_cyan());
                }
                println!();
            }
            existing
        } else {
            ClaudeConfig {
                mcp_servers: std::collections::HashMap::new(),
            }
        }
    } else {
        println!("  {} No existing configuration found", "ℹ".blue());
        println!();
        ClaudeConfig {
            mcp_servers: std::collections::HashMap::new(),
        }
    };

    // Recommended MCP servers for AutoFlow
    let recommended_servers = get_recommended_servers();

    let servers_to_install: Vec<&str> = if servers.is_empty() {
        // Install all recommended
        recommended_servers.keys().map(|s| s.as_str()).collect()
    } else {
        servers.iter().map(|s| s.as_str()).collect()
    };

    let mut installed_count = 0;
    let mut skipped_count = 0;

    for server_name in servers_to_install {
        if let Some(server_info) = recommended_servers.get(server_name) {
            // Check if already exists
            if config.mcp_servers.contains_key(server_name) {
                println!("  {} {} already configured (skipping)",
                    "→".yellow(),
                    server_name.bright_blue()
                );
                skipped_count += 1;
                continue;
            }

            println!("{} {}...", "Installing".bright_cyan(), server_name.bright_blue());

            match install_server(server_name, server_info, &mut config).await {
                Ok(_) => {
                    println!("  {} {} installed", "✓".green(), server_name.bright_blue());
                    installed_count += 1;
                }
                Err(e) => {
                    println!("  {} Failed to install {}: {}", "✗".red(), server_name, e);
                }
            }
            println!();
        } else {
            println!("  {} Unknown server: {}", "⚠".yellow(), server_name);
        }
    }

    // Save config
    let config_json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path_clone, config_json)?;

    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!("{}", "  ✅ MCP Server Installation Complete!".bright_green().bold());
    println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
    println!();

    println!("{}", "📊 Summary:".bright_cyan());
    println!("   Installed: {}", installed_count.to_string().bright_green());
    if skipped_count > 0 {
        println!("   Skipped (already configured): {}", skipped_count.to_string().bright_yellow());
    }
    println!("   Total configured: {}", config.mcp_servers.len().to_string().bright_blue());
    println!();

    println!("{}", "📝 Configuration saved to:".bright_cyan());
    println!("   {}", config_path_clone.display().to_string().bright_blue());
    println!();

    println!("{}", "🔄 Next steps:".bright_cyan());
    if project_level {
        println!("   1. Project-level servers configured for this project only");
        println!("   2. Start using: {}", "autoflow start".bright_blue());
    } else {
        println!("   1. Restart Claude Desktop (if running)");
        println!("   2. MCP servers will be available in all Claude Code sessions");
        println!("   3. Verify with: {}", "autoflow mcp list".bright_blue());
    }
    println!();

    Ok(())
}

pub async fn run_list() -> Result<()> {
    println!("{}", "📋 Installed MCP Servers".bright_cyan().bold());
    println!();

    let home = std::env::var("HOME")?;
    let user_config_path = PathBuf::from(&home).join(".claude/settings.local.json");
    let project_config_path = PathBuf::from(".mcp.json");

    let mut all_servers = std::collections::HashMap::new();

    // Load user-level servers
    if user_config_path.exists() {
        match fs::read_to_string(&user_config_path) {
            Ok(content) => {
                if let Ok(config) = serde_json::from_str::<ClaudeConfig>(&content) {
                    println!("{}", "User-level servers (all projects):".bright_cyan());
                    for (name, _) in &config.mcp_servers {
                        println!("  • {}", name.bright_blue());
                        all_servers.insert(name.clone(), "user");
                    }
                    println!();
                }
            }
            Err(_) => {}
        }
    }

    // Load project-level servers
    if project_config_path.exists() {
        match fs::read_to_string(&project_config_path) {
            Ok(content) => {
                if let Ok(config) = serde_json::from_str::<ClaudeConfig>(&content) {
                    println!("{}", "Project-level servers (this project only):".bright_cyan());
                    for (name, _) in &config.mcp_servers {
                        println!("  • {}", name.bright_blue());
                        all_servers.insert(name.clone(), "project");
                    }
                    println!();
                }
            }
            Err(_) => {}
        }
    }

    if all_servers.is_empty() {
        println!("{}", "No MCP servers installed.".yellow());
        println!();
        println!("Install recommended servers with:");
        println!("  {} - User-level (all projects)", "autoflow mcp install".bright_blue());
        println!("  {} - Project-level (this project only)", "autoflow mcp install --project".bright_blue());
        return Ok(());
    }

    println!("{}", "Detailed configuration:".bright_cyan());
    println!();

    // Show user-level details
    let user_config_path_for_details = user_config_path;
    if user_config_path_for_details.exists() {
        if let Ok(content) = fs::read_to_string(&user_config_path_for_details) {
            if let Ok(config) = serde_json::from_str::<ClaudeConfig>(&content) {
                for (name, server) in &config.mcp_servers {
                    println!("  {} {} (user)", "•".bright_blue(), name.bright_cyan());
                    println!("    Command: {}", server.command);
                    if let Some(args) = &server.args {
                        println!("    Args: {}", args.join(" "));
                    }
                    println!();
                }
            }
        }
    }

    Ok(())
}

pub async fn run_info(server_name: Option<String>) -> Result<()> {
    let recommended = get_recommended_servers();

    if let Some(name) = server_name {
        if let Some(info) = recommended.get(&name) {
            println!("{}", format!("📦 {} MCP Server", name).bright_cyan().bold());
            println!();
            println!("{}: {}", "Description".bright_cyan(), info.description);
            println!("{}: {}", "Use Case".bright_cyan(), info.use_case);
            println!();
            println!("{}", "Capabilities:".bright_cyan());
            for capability in &info.capabilities {
                println!("  • {}", capability);
            }
            println!();
            println!("{}", "Installation:".bright_cyan());
            println!("  {}", format!("autoflow mcp install {}", name).bright_blue());
        } else {
            println!("{} Unknown server: {}", "⚠".yellow(), name);
        }
    } else {
        println!("{}", "📦 Recommended MCP Servers for AutoFlow".bright_cyan().bold());
        println!();

        for (name, info) in recommended {
            println!("{} {} - {}", "•".bright_blue(), name.bright_cyan(), info.description);
        }

        println!();
        println!("For details: {}", "autoflow mcp info <server-name>".bright_blue());
        println!("Install all: {}", "autoflow mcp install".bright_blue());
    }

    Ok(())
}

#[derive(Debug)]
struct ServerInfo {
    description: String,
    use_case: String,
    capabilities: Vec<String>,
    install_command: String,
    npm_package: Option<String>,
    github_repo: Option<String>,
}

fn get_recommended_servers() -> std::collections::HashMap<String, ServerInfo> {
    let mut servers = std::collections::HashMap::new();

    servers.insert(
        "memory".to_string(),
        ServerInfo {
            description: "Knowledge graph memory for persistent context".to_string(),
            use_case: "Store learnings across sessions, remember project patterns".to_string(),
            capabilities: vec![
                "Create entities (functions, patterns, bugs)".to_string(),
                "Add observations and relations".to_string(),
                "Search stored knowledge".to_string(),
                "Build up project knowledge over time".to_string(),
            ],
            install_command: "npx -y @modelcontextprotocol/server-memory".to_string(),
            npm_package: Some("@modelcontextprotocol/server-memory".to_string()),
            github_repo: Some("modelcontextprotocol/servers".to_string()),
        },
    );

    servers.insert(
        "filesystem".to_string(),
        ServerInfo {
            description: "Advanced file operations with search".to_string(),
            use_case: "Read/write files with better context and search capabilities".to_string(),
            capabilities: vec![
                "Read and write files".to_string(),
                "Create directories".to_string(),
                "Move and delete files".to_string(),
                "Search file contents".to_string(),
                "List directory contents".to_string(),
            ],
            install_command: "npx -y @modelcontextprotocol/server-filesystem".to_string(),
            npm_package: Some("@modelcontextprotocol/server-filesystem".to_string()),
            github_repo: Some("modelcontextprotocol/servers".to_string()),
        },
    );

    servers.insert(
        "github".to_string(),
        ServerInfo {
            description: "GitHub API integration".to_string(),
            use_case: "Create issues, PRs, manage repositories".to_string(),
            capabilities: vec![
                "Create and manage issues".to_string(),
                "Create and review pull requests".to_string(),
                "Search repositories".to_string(),
                "Manage branches and commits".to_string(),
                "Access GitHub Actions".to_string(),
            ],
            install_command: "npx -y @modelcontextprotocol/server-github".to_string(),
            npm_package: Some("@modelcontextprotocol/server-github".to_string()),
            github_repo: Some("modelcontextprotocol/servers".to_string()),
        },
    );

    servers.insert(
        "postgres".to_string(),
        ServerInfo {
            description: "PostgreSQL database access".to_string(),
            use_case: "Query databases, inspect schemas, run migrations".to_string(),
            capabilities: vec![
                "Execute SQL queries".to_string(),
                "Inspect database schema".to_string(),
                "List tables and columns".to_string(),
                "Analyze query performance".to_string(),
            ],
            install_command: "npx -y @modelcontextprotocol/server-postgres".to_string(),
            npm_package: Some("@modelcontextprotocol/server-postgres".to_string()),
            github_repo: Some("modelcontextprotocol/servers".to_string()),
        },
    );

    servers.insert(
        "playwright".to_string(),
        ServerInfo {
            description: "Browser automation for testing".to_string(),
            use_case: "E2E testing, bug reproduction, UI interaction".to_string(),
            capabilities: vec![
                "Launch browsers (Chrome, Firefox, Safari)".to_string(),
                "Navigate and interact with pages".to_string(),
                "Take screenshots".to_string(),
                "Fill forms and click elements".to_string(),
                "Get computed styles".to_string(),
                "Execute JavaScript".to_string(),
            ],
            install_command: "npx -y @executeautomation/playwright-mcp-server".to_string(),
            npm_package: Some("@executeautomation/playwright-mcp-server".to_string()),
            github_repo: Some("executeautomation/playwright-mcp-server".to_string()),
        },
    );

    servers.insert(
        "fetch".to_string(),
        ServerInfo {
            description: "HTTP requests and web scraping".to_string(),
            use_case: "Fetch documentation, test APIs, scrape web content".to_string(),
            capabilities: vec![
                "Make HTTP requests (GET, POST, etc)".to_string(),
                "Handle authentication".to_string(),
                "Parse HTML responses".to_string(),
                "Download files".to_string(),
            ],
            install_command: "npx -y @modelcontextprotocol/server-fetch".to_string(),
            npm_package: Some("@modelcontextprotocol/server-fetch".to_string()),
            github_repo: Some("modelcontextprotocol/servers".to_string()),
        },
    );

    servers.insert(
        "git".to_string(),
        ServerInfo {
            description: "Git repository operations".to_string(),
            use_case: "Read commits, diffs, history, branch info".to_string(),
            capabilities: vec![
                "Read git log and history".to_string(),
                "View diffs and changes".to_string(),
                "List branches and tags".to_string(),
                "Show file history".to_string(),
                "Inspect commits".to_string(),
            ],
            install_command: "npx -y @modelcontextprotocol/server-git".to_string(),
            npm_package: Some("@modelcontextprotocol/server-git".to_string()),
            github_repo: Some("modelcontextprotocol/servers".to_string()),
        },
    );

    servers.insert(
        "sqlite".to_string(),
        ServerInfo {
            description: "SQLite database access".to_string(),
            use_case: "Query local databases, manage app data".to_string(),
            capabilities: vec![
                "Execute SQL queries".to_string(),
                "Inspect schema".to_string(),
                "List tables".to_string(),
                "Manage local data".to_string(),
            ],
            install_command: "npx -y @modelcontextprotocol/server-sqlite".to_string(),
            npm_package: Some("@modelcontextprotocol/server-sqlite".to_string()),
            github_repo: Some("modelcontextprotocol/servers".to_string()),
        },
    );

    servers
}

async fn install_server(
    name: &str,
    info: &ServerInfo,
    config: &mut ClaudeConfig,
) -> Result<()> {
    // Check if npm is available
    let npm_check = Command::new("npm").arg("--version").output();
    if npm_check.is_err() {
        anyhow::bail!("npm not found. Please install Node.js and npm first.");
    }

    // Parse command and args
    let parts: Vec<&str> = info.install_command.split_whitespace().collect();
    let command = parts[0].to_string();
    let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

    // Create server config
    let mut server_config = McpServerConfig {
        command,
        args: Some(args),
        env: None,
    };

    // Add special configurations for certain servers
    match name {
        "github" => {
            println!("  {} GitHub requires GITHUB_TOKEN environment variable", "ℹ".blue());
            println!("  Create token at: https://github.com/settings/tokens");
            server_config.env = Some({
                let mut env = std::collections::HashMap::new();
                env.insert("GITHUB_TOKEN".to_string(), "your-token-here".to_string());
                env
            });
        }
        "postgres" => {
            println!("  {} PostgreSQL requires connection string", "ℹ".blue());
            println!("  Set POSTGRES_CONNECTION_STRING in environment");
            server_config.env = Some({
                let mut env = std::collections::HashMap::new();
                env.insert(
                    "POSTGRES_CONNECTION_STRING".to_string(),
                    "postgresql://user:pass@localhost:5432/dbname".to_string(),
                );
                env
            });
        }
        "filesystem" => {
            // Allow access to home directory by default
            let home = std::env::var("HOME")?;
            server_config.args = Some(vec![
                "-y".to_string(),
                "@modelcontextprotocol/server-filesystem".to_string(),
                home,
            ]);
        }
        _ => {}
    }

    // Add to config
    config.mcp_servers.insert(name.to_string(), server_config);

    Ok(())
}
