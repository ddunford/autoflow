use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Claude API streaming event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreamEvent {
    MessageStart {
        message: MessageInfo,
    },
    ContentBlockStart {
        index: usize,
        content_block: ContentBlock,
    },
    ContentBlockDelta {
        index: usize,
        delta: Delta,
    },
    ContentBlockStop {
        index: usize,
    },
    MessageDelta {
        delta: MessageDeltaInfo,
        usage: Option<UsageInfo>,
    },
    MessageStop,
    Ping,
    Error {
        error: ErrorInfo,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub role: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    Text { text: String },
    ToolUse { id: String, name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Delta {
    TextDelta { text: String },
    InputJsonDelta { partial_json: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDeltaInfo {
    pub stop_reason: Option<String>,
    pub stop_sequence: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageInfo {
    pub output_tokens: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

/// Live logger that writes streaming events to JSONL file
pub struct LiveLogger {
    file: Arc<Mutex<File>>,
    log_path: PathBuf,
}

impl LiveLogger {
    /// Create a new live logger for an agent execution
    pub fn new(agent_name: &str, sprint_id: Option<u32>) -> Result<Self> {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");

        // Organize by sprint folder, then agent name with timestamp
        let log_path = if let Some(id) = sprint_id {
            let sprint_dir = PathBuf::from(".autoflow/.debug/live").join(format!("sprint-{}", id));
            sprint_dir.join(format!("{}_{}.jsonl", timestamp, agent_name))
        } else {
            PathBuf::from(".autoflow/.debug/live").join(format!("{}_{}.jsonl", timestamp, agent_name))
        };

        // Ensure directory exists
        if let Some(parent) = log_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&log_path)?;

        Ok(Self {
            file: Arc::new(Mutex::new(file)),
            log_path,
        })
    }

    /// Log a streaming event
    pub fn log_event(&self, event: &StreamEvent) -> Result<()> {
        let mut file = self.file.lock().unwrap();
        let json = serde_json::to_string(event)?;
        writeln!(file, "{}", json)?;
        file.flush()?;
        Ok(())
    }

    /// Log a custom text event (for agent metadata)
    pub fn log_text(&self, text: &str) -> Result<()> {
        let event = StreamEvent::ContentBlockDelta {
            index: 0,
            delta: Delta::TextDelta {
                text: text.to_string(),
            },
        };
        self.log_event(&event)
    }

    /// Log agent start
    pub fn log_agent_start(&self, agent_name: &str, model: &str) -> Result<()> {
        let event = StreamEvent::MessageStart {
            message: MessageInfo {
                id: format!("msg_{}", chrono::Utc::now().timestamp()),
                msg_type: "message".to_string(),
                role: "assistant".to_string(),
                model: model.to_string(),
            },
        };
        self.log_event(&event)
    }

    /// Log agent completion
    pub fn log_agent_complete(&self, stop_reason: &str, output_tokens: usize) -> Result<()> {
        let event = StreamEvent::MessageDelta {
            delta: MessageDeltaInfo {
                stop_reason: Some(stop_reason.to_string()),
                stop_sequence: None,
            },
            usage: Some(UsageInfo { output_tokens }),
        };
        self.log_event(&event)?;

        let event = StreamEvent::MessageStop;
        self.log_event(&event)
    }

    /// Get the log file path
    pub fn path(&self) -> &PathBuf {
        &self.log_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_logger_creation() {
        let logger = LiveLogger::new("test-agent", Some(1));
        assert!(logger.is_ok());
    }

    #[test]
    fn test_log_event() {
        let logger = LiveLogger::new("test-agent", Some(1)).unwrap();
        let result = logger.log_agent_start("test-agent", "claude-sonnet-4");
        assert!(result.is_ok());
    }
}
