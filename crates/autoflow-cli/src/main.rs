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
    }

    Ok(())
}
