use anyhow::Result;
use chrono::Utc;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Debug logger for comprehensive execution tracing
pub struct DebugLogger {
    debug_dir: PathBuf,
    session_id: String,
}

impl DebugLogger {
    /// Create a new debug logger
    pub fn new() -> Result<Self> {
        let debug_dir = PathBuf::from(".autoflow/.debug");
        fs::create_dir_all(&debug_dir)?;
        
        // Add to gitignore
        Self::ensure_gitignored()?;
        
        let session_id = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        
        Ok(Self {
            debug_dir,
            session_id,
        })
    }
    
    /// Ensure .debug directory is in .gitignore
    fn ensure_gitignored() -> Result<()> {
        let gitignore_path = ".autoflow/.gitignore";
        
        // Create .autoflow/.gitignore if it doesn't exist
        if !Path::new(gitignore_path).exists() {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(gitignore_path)?;
            writeln!(file, ".debug/")?;
        } else {
            // Check if .debug is already in .gitignore
            let content = fs::read_to_string(gitignore_path)?;
            if !content.contains(".debug") {
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(gitignore_path)?;
                writeln!(file, ".debug/")?;
            }
        }
        
        Ok(())
    }
    
    /// Log agent execution start
    pub fn log_agent_start(&self, agent_name: &str, context: &str) -> Result<()> {
        let log_file = self.debug_dir.join(format!("{}_{}.log", self.session_id, agent_name));
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)?;
        
        writeln!(file, "=")?;
        writeln!(file, "=")?;
        writeln!(file, "=")?;
        writeln!(file, "AGENT EXECUTION START")?;
        writeln!(file, "=")?;
        writeln!(file, "Agent: {}", agent_name)?;
        writeln!(file, "Timestamp: {}", Utc::now().to_rfc3339())?;
        writeln!(file, "Session: {}", self.session_id)?;
        writeln!(file, "=")?;
        writeln!(file)?;
        writeln!(file, "CONTEXT:")?;
        writeln!(file, "---")?;
        writeln!(file, "{}", context)?;
        writeln!(file, "---")?;
        writeln!(file)?;
        
        Ok(())
    }
    
    /// Log agent execution step
    pub fn log_agent_step(&self, agent_name: &str, step: &str, details: &str) -> Result<()> {
        let log_file = self.debug_dir.join(format!("{}_{}.log", self.session_id, agent_name));
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)?;

        writeln!(file, "[{}] {}", Utc::now().format("%H:%M:%S"), step)?;
        if !details.is_empty() {
            writeln!(file, "{}", details)?;
        }
        writeln!(file)?;

        Ok(())
    }

    /// Log agent execution end
    pub fn log_agent_end(&self, agent_name: &str, success: bool, error: Option<&str>) -> Result<()> {
        let log_file = self.debug_dir.join(format!("{}_{}.log", self.session_id, agent_name));
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)?;
        
        writeln!(file)?;
        writeln!(file, "=")?;
        writeln!(file, "AGENT EXECUTION END")?;
        writeln!(file, "=")?;
        writeln!(file, "Status: {}", if success { "SUCCESS" } else { "FAILED" })?;
        if let Some(err) = error {
            writeln!(file, "Error: {}", err)?;
        }
        writeln!(file, "Timestamp: {}", Utc::now().to_rfc3339())?;
        writeln!(file, "=")?;
        writeln!(file, "=")?;
        writeln!(file, "=")?;
        writeln!(file)?;
        
        Ok(())
    }
    
    /// Log sprint execution
    pub fn log_sprint(&self, sprint_id: u32, status: &str, details: &str) -> Result<()> {
        let log_file = self.debug_dir.join(format!("{}_sprint_execution.log", self.session_id));
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)?;
        
        writeln!(file, "[{}] Sprint {} - {}", Utc::now().format("%H:%M:%S"), sprint_id, status)?;
        if !details.is_empty() {
            writeln!(file, "{}", details)?;
        }
        writeln!(file)?;
        
        Ok(())
    }
    
    /// Log command execution
    pub fn log_command(&self, command: &str, output: &str, error: Option<&str>) -> Result<()> {
        let log_file = self.debug_dir.join(format!("{}_commands.log", self.session_id));
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)?;
        
        writeln!(file, "[{}] COMMAND: {}", Utc::now().format("%H:%M:%S"), command)?;
        if !output.is_empty() {
            writeln!(file, "Output:")?;
            writeln!(file, "{}", output)?;
        }
        if let Some(err) = error {
            writeln!(file, "Error:")?;
            writeln!(file, "{}", err)?;
        }
        writeln!(file, "---")?;
        writeln!(file)?;
        
        Ok(())
    }
    
    /// Get the current session log directory
    pub fn session_dir(&self) -> PathBuf {
        self.debug_dir.clone()
    }
}

/// Global debug logger instance
pub fn get_debug_logger() -> Option<DebugLogger> {
    // Only create if in a project directory
    if Path::new(".autoflow").exists() {
        DebugLogger::new().ok()
    } else {
        None
    }
}
