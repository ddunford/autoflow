use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "autoflow")]
#[command(version = "0.1.0")]
#[command(about = "Best-in-class autonomous coding agent", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Install AutoFlow globally to ~/.autoflow/
    Install {
        /// Force reinstall even if already installed
        #[arg(long)]
        force: bool,
    },

    /// Create new project from IDEA.md (full autonomous setup)
    Create {
        /// Project name (optional, uses current directory if IDEA.md exists)
        #[arg(value_name = "NAME", required = false)]
        name: Option<String>,

        /// Path to IDEA.md file (optional, uses ./IDEA.md or creates template if not provided)
        #[arg(short, long)]
        idea: Option<String>,
    },

    /// Initialize new project with AutoFlow
    Init {
        /// Use specific template (e.g., "react-node", "laravel-react")
        #[arg(short, long)]
        template: Option<String>,
    },

    /// Start autonomous development (execute sprints)
    Start {
        /// Execute sprints in parallel
        #[arg(short, long)]
        parallel: bool,

        /// Run specific sprint by ID
        #[arg(short, long)]
        sprint: Option<u32>,
    },

    /// Show sprint progress and status
    Status {
        /// Output as JSON
        #[arg(short, long)]
        json: bool,
    },

    /// Analyze existing codebase
    Analyze,

    /// Add new feature to existing codebase
    Add {
        /// Feature description
        description: String,

        /// Additional requirements
        #[arg(short, long)]
        requirements: Option<String>,
    },

    /// Fix a bug
    Fix {
        /// Bug description
        description: String,

        /// Auto-implement fix after investigation
        #[arg(long)]
        auto_fix: bool,

        /// Launch Playwright in headed mode
        #[arg(long)]
        playwright_headed: bool,
    },

    /// Update documentation and regenerate sprints based on feedback
    Pivot {
        /// Feedback/instruction for updating documentation
        instruction: String,
    },

    /// Rollback sprint
    Rollback {
        /// Sprint ID to rollback (default: last sprint)
        #[arg(short, long)]
        sprint: Option<u32>,
    },

    /// Manage git worktrees
    #[command(subcommand)]
    Worktree(WorktreeCommands),

    /// Validate project (quality gates, infrastructure, etc.)
    Validate {
        /// Validate infrastructure
        #[arg(long)]
        infrastructure: bool,

        /// Validate integration
        #[arg(long)]
        integration: bool,

        /// Auto-fix issues where possible
        #[arg(long)]
        fix: bool,
    },

    /// Manage sprints
    #[command(subcommand)]
    Sprints(SprintsCommands),

    /// List available agents
    Agents {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },

    /// List available skills
    Skills,

    /// Manage development environment
    #[command(subcommand)]
    Env(EnvCommands),

    /// Manage MCP servers
    #[command(subcommand)]
    Mcp(McpCommands),
}

#[derive(Subcommand, Debug)]
enum WorktreeCommands {
    /// List all worktrees
    List {
        /// Filter by type (sprint, bugfix, other)
        #[arg(long)]
        r#type: Option<String>,
    },

    /// Create new worktree
    Create {
        /// Branch name
        branch: String,
    },

    /// Merge worktree to main
    Merge {
        /// Branch name
        branch: String,
    },

    /// Delete worktree
    Delete {
        /// Branch name
        branch: String,

        /// Force delete even if not merged
        #[arg(short, long)]
        force: bool,
    },

    /// Clean up merged worktrees
    Prune,
}

#[derive(Subcommand, Debug)]
enum SprintsCommands {
    /// List all sprints
    List,

    /// Show sprint details
    Show {
        /// Sprint ID
        id: u32,

        /// Show integration points
        #[arg(long)]
        integration: bool,
    },

    /// Create new sprint manually
    Create,
}

#[derive(Subcommand, Debug)]
enum EnvCommands {
    /// Start development environment
    Start,

    /// Stop development environment
    Stop,

    /// Restart development environment
    Restart,

    /// View environment logs
    Logs {
        /// Follow logs
        #[arg(short, long)]
        follow: bool,
    },

    /// Check environment health
    Health,
}

#[derive(Subcommand, Debug)]
enum McpCommands {
    /// Install MCP servers (installs all recommended if none specified)
    Install {
        /// Specific servers to install (memory, playwright, github, postgres, etc.)
        servers: Vec<String>,

        /// Install to project level (.autoflow/settings.json) instead of global
        #[arg(long)]
        project: bool,
    },

    /// List installed MCP servers
    List,

    /// Show information about available servers
    Info {
        /// Server name (optional, shows all if not specified)
        server: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Set up logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(log_level)
        .with_target(false)
        .init();

    // Execute command
    match cli.command {
        Commands::Install { force } => {
            commands::install::run(force).await?;
        }
        Commands::Create { name, idea } => {
            commands::create::run(name, idea).await?;
        }
        Commands::Init { template } => {
            commands::init::run(template).await?;
        }
        Commands::Start { parallel, sprint } => {
            commands::start::run(parallel, sprint).await?;
        }
        Commands::Status { json } => {
            commands::status::run(json).await?;
        }
        Commands::Analyze => {
            commands::analyze::run().await?;
        }
        Commands::Add {
            description,
            requirements,
        } => {
            commands::add::run(description, requirements).await?;
        }
        Commands::Fix {
            description,
            auto_fix,
            playwright_headed,
        } => {
            commands::fix::run(description, auto_fix, playwright_headed).await?;
        }
        Commands::Pivot { instruction } => {
            commands::pivot::run(instruction).await?;
        }
        Commands::Rollback { sprint } => {
            commands::rollback::run(sprint).await?;
        }
        Commands::Worktree(cmd) => {
            commands::worktree::run(cmd).await?;
        }
        Commands::Validate {
            infrastructure,
            integration,
            fix,
        } => {
            commands::validate::run(infrastructure, integration, fix).await?;
        }
        Commands::Sprints(cmd) => {
            commands::sprints::run(cmd).await?;
        }
        Commands::Agents { detailed } => {
            commands::agents::run(detailed).await?;
        }
        Commands::Skills => {
            commands::skills::run().await?;
        }
        Commands::Env(cmd) => {
            commands::env::run(cmd).await?;
        }
        Commands::Mcp(cmd) => match cmd {
            McpCommands::Install { servers, project } => {
                commands::mcp::run_install(servers, project).await?;
            }
            McpCommands::List => {
                commands::mcp::run_list().await?;
            }
            McpCommands::Info { server } => {
                commands::mcp::run_info(server).await?;
            }
        },
    }

    Ok(())
}
