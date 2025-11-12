use autoflow_data::{AutoFlowError, Result};
use git2::{Repository, BranchType};
use std::path::{Path, PathBuf};
use std::fs;

/// Git worktree manager
pub struct WorktreeManager {
    repo: Repository,
}

impl WorktreeManager {
    /// Create a new worktree manager for the current repository
    pub fn new<P: AsRef<Path>>(repo_path: P) -> Result<Self> {
        let repo = Repository::open(repo_path)?;
        Ok(Self { repo })
    }

    /// Create a new worktree for a sprint
    pub fn create_worktree(&self, sprint_id: u32, branch_name: &str) -> Result<WorktreeInfo> {
        // Get repository path
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| AutoFlowError::ValidationError("Invalid repository path".to_string()))?;

        // Create worktree path
        let worktree_name = format!("sprint-{}", sprint_id);
        let worktree_path = repo_path.join("..").join(&worktree_name);

        // Check if worktree already exists
        if worktree_path.exists() {
            return Err(AutoFlowError::ValidationError(
                format!("Worktree already exists: {}", worktree_path.display())
            ));
        }

        // Get current branch to use as base
        let head = self.repo.head()?;
        let base_branch = head.shorthand()
            .ok_or_else(|| AutoFlowError::ValidationError("Could not determine current branch".to_string()))?;

        tracing::info!("Creating worktree {} from branch {}", worktree_name, base_branch);

        // Create worktree with new branch using git command (git2 doesn't support worktree creation)
        // The -b flag creates a new branch from HEAD
        let status = std::process::Command::new("git")
            .args(&["worktree", "add", "-b", branch_name, worktree_path.to_str().unwrap(), "HEAD"])
            .current_dir(repo_path)
            .status()?;

        if !status.success() {
            return Err(AutoFlowError::ValidationError(
                "Failed to create git worktree".to_string()
            ));
        }

        // Calculate unique port for this worktree
        let base_port = 3000;
        let port = base_port + (sprint_id * 10);

        Ok(WorktreeInfo {
            name: worktree_name,
            path: worktree_path,
            branch: branch_name.to_string(),
            port,
            created_at: chrono::Utc::now(),
        })
    }

    /// List all worktrees
    pub fn list_worktrees(&self) -> Result<Vec<WorktreeInfo>> {
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| AutoFlowError::ValidationError("Invalid repository path".to_string()))?;

        // Use git command to list worktrees
        let output = std::process::Command::new("git")
            .args(&["worktree", "list", "--porcelain"])
            .current_dir(repo_path)
            .output()?;

        if !output.status.success() {
            return Err(AutoFlowError::ValidationError(
                "Failed to list git worktrees".to_string()
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut worktrees = Vec::new();
        let mut current_worktree: Option<(PathBuf, String)> = None;

        for line in stdout.lines() {
            if line.starts_with("worktree ") {
                let path = PathBuf::from(line.trim_start_matches("worktree "));
                current_worktree = Some((path, String::new()));
            } else if line.starts_with("branch ") {
                if let Some((path, _)) = &mut current_worktree {
                    let branch = line.trim_start_matches("branch refs/heads/").to_string();

                    // Extract sprint ID from path if it's a sprint worktree
                    let name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    let sprint_id = if name.starts_with("sprint-") {
                        name.trim_start_matches("sprint-")
                            .parse::<u32>()
                            .ok()
                    } else {
                        None
                    };

                    let port = sprint_id.map(|id| 3000 + (id * 10)).unwrap_or(3000);

                    worktrees.push(WorktreeInfo {
                        name,
                        path: path.clone(),
                        branch,
                        port,
                        created_at: chrono::Utc::now(), // We don't have actual creation time
                    });

                    current_worktree = None;
                }
            }
        }

        Ok(worktrees)
    }

    /// Delete a worktree
    pub fn delete_worktree(&self, worktree_name: &str) -> Result<()> {
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| AutoFlowError::ValidationError("Invalid repository path".to_string()))?;

        tracing::info!("Deleting worktree {}", worktree_name);

        // Get worktree path
        let worktree_path = repo_path.join("..").join(worktree_name);

        if !worktree_path.exists() {
            return Err(AutoFlowError::ValidationError(
                format!("Worktree does not exist: {}", worktree_name)
            ));
        }

        // Remove worktree using git command
        let status = std::process::Command::new("git")
            .args(&["worktree", "remove", worktree_path.to_str().unwrap(), "--force"])
            .current_dir(repo_path)
            .status()?;

        if !status.success() {
            return Err(AutoFlowError::ValidationError(
                "Failed to remove git worktree".to_string()
            ));
        }

        Ok(())
    }

    /// Merge a worktree branch back to main
    pub fn merge_worktree(&self, branch_name: &str) -> Result<()> {
        tracing::info!("Merging branch {} to main", branch_name);

        // Checkout main branch
        let main_branch = self.repo.find_branch("main", BranchType::Local)
            .or_else(|_| self.repo.find_branch("master", BranchType::Local))?;

        let main_ref = main_branch.get().name()
            .ok_or_else(|| AutoFlowError::ValidationError("Invalid main branch".to_string()))?;

        self.repo.set_head(main_ref)?;

        // Checkout working directory
        self.repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;

        // Merge the branch
        let branch = self.repo.find_branch(branch_name, BranchType::Local)?;
        let branch_commit = branch.get().peel_to_commit()?;

        let mut index = self.repo.merge_commits(
            &self.repo.head()?.peel_to_commit()?,
            &branch_commit,
            None
        )?;

        if index.has_conflicts() {
            return Err(AutoFlowError::MergeConflict {
                branch: branch_name.to_string(),
            });
        }

        // Create merge commit
        let tree_oid = index.write_tree_to(&self.repo)?;
        let tree = self.repo.find_tree(tree_oid)?;
        let signature = self.repo.signature()?;
        let main_commit = self.repo.head()?.peel_to_commit()?;

        self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &format!("Merge sprint branch {}", branch_name),
            &tree,
            &[&main_commit, &branch_commit],
        )?;

        tracing::info!("Successfully merged {} to main", branch_name);

        Ok(())
    }

    /// Prune merged worktrees
    pub fn prune_worktrees(&self) -> Result<Vec<String>> {
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| AutoFlowError::ValidationError("Invalid repository path".to_string()))?;

        // Prune using git command
        let status = std::process::Command::new("git")
            .args(&["worktree", "prune"])
            .current_dir(repo_path)
            .status()?;

        if !status.success() {
            return Err(AutoFlowError::ValidationError(
                "Failed to prune git worktrees".to_string()
            ));
        }

        Ok(Vec::new()) // Return empty for now
    }

    /// Setup environment for a worktree
    pub fn setup_worktree_env(&self, worktree_info: &WorktreeInfo) -> Result<()> {
        // Copy docker-compose.yml if it exists
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| AutoFlowError::ValidationError("Invalid repository path".to_string()))?;

        let docker_compose = repo_path.join("docker-compose.yml");

        if docker_compose.exists() {
            let content = fs::read_to_string(&docker_compose)?;

            // Replace ports
            let modified = content.replace("3000:", &format!("{}:", worktree_info.port));

            let worktree_docker = worktree_info.path.join("docker-compose.yml");
            fs::write(worktree_docker, modified)?;
        }

        // Copy .env if it exists
        let env_file = repo_path.join(".env");
        if env_file.exists() {
            let content = fs::read_to_string(&env_file)?;

            // Modify port in .env
            let modified = content.replace("PORT=3000", &format!("PORT={}", worktree_info.port));

            let worktree_env = worktree_info.path.join(".env");
            fs::write(worktree_env, modified)?;
        }

        Ok(())
    }
}

/// Worktree information
#[derive(Debug, Clone)]
pub struct WorktreeInfo {
    pub name: String,
    pub path: PathBuf,
    pub branch: String,
    pub port: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl WorktreeInfo {
    pub fn display_path(&self) -> String {
        self.path.display().to_string()
    }
}
