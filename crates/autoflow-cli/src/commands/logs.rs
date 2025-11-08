use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::PathBuf;
use std::io::{BufRead, BufReader};

pub async fn run(follow: bool, live: bool) -> Result<()> {
    let log_dir = if live {
        PathBuf::from(".autoflow/.debug/live")
    } else {
        PathBuf::from(".autoflow/.debug")
    };

    if !log_dir.exists() {
        println!("{}", format!("No logs found in {:?}", log_dir).red());
        return Ok(());
    }

    // Find the most recent log file (search recursively for live logs)
    let mut log_files: Vec<_> = if live {
        // For live logs, search in sprint subdirectories
        walkdir::WalkDir::new(&log_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| {
                let path = entry.path();
                path.is_file() && path.extension().map_or(false, |ext| ext == "jsonl")
            })
            .filter_map(|entry| fs::metadata(entry.path()).ok().map(|m| (entry.into_path(), m)))
            .collect()
    } else {
        fs::read_dir(&log_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let path = entry.path();
                path.is_file() && path.extension().map_or(false, |ext| ext == "log")
            })
            .filter_map(|entry| fs::metadata(entry.path()).ok().map(|m| (entry.path(), m)))
            .collect()
    };

    log_files.sort_by_key(|(_, metadata)| {
        metadata.modified().ok().unwrap_or(std::time::SystemTime::UNIX_EPOCH)
    });

    let latest = log_files.last()
        .context("No log files found")?;

    let log_path = &latest.0;
    println!("{}", format!("📋 Viewing: {:?}", log_path).bright_cyan());

    if live {
        // Parse and display JSONL streaming events
        display_live_log(&log_path, follow)?;
    } else {
        // Display regular log file
        display_regular_log(&log_path, follow)?;
    }

    Ok(())
}

fn display_live_log(path: &PathBuf, follow: bool) -> Result<()> {
    use serde_json::Value;

    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    println!("\n{}", "Streaming Events:".bright_green().bold());
    println!("{}", "─".repeat(80).dimmed());

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        match serde_json::from_str::<Value>(&line) {
            Ok(event) => {
                display_event(&event);
            }
            Err(e) => {
                eprintln!("{}", format!("Failed to parse event: {}", e).red());
            }
        }
    }

    if follow {
        println!("\n{}", "Following mode not yet implemented...".yellow());
    }

    Ok(())
}

fn display_event(event: &serde_json::Value) {
    let event_type = event.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");

    match event_type {
        "message_start" => {
            if let Some(msg) = event.get("message") {
                let model = msg.get("model").and_then(|v| v.as_str()).unwrap_or("unknown");
                println!("\n{} {}", "🚀 Agent Started:".bright_green(), model.cyan());
            }
        }
        "content_block_start" => {
            let index = event.get("index").and_then(|v| v.as_u64()).unwrap_or(0);
            println!("{} {}", "📝 Content Block".bright_blue(), index);
        }
        "content_block_delta" => {
            if let Some(delta) = event.get("delta") {
                if let Some(text) = delta.get("text").and_then(|v| v.as_str()) {
                    print!("{}", text);
                } else if let Some(json) = delta.get("partial_json").and_then(|v| v.as_str()) {
                    print!("{}", json.dimmed());
                }
            }
        }
        "content_block_stop" => {
            println!();
        }
        "message_delta" => {
            if let Some(delta) = event.get("delta") {
                if let Some(reason) = delta.get("stop_reason").and_then(|v| v.as_str()) {
                    println!("\n{} {}", "✓ Stopped:".green(), reason.yellow());
                }
            }
            if let Some(usage) = event.get("usage") {
                if let Some(tokens) = usage.get("output_tokens").and_then(|v| v.as_u64()) {
                    println!("{} {} tokens", "   Output:".dimmed(), tokens.to_string().bright_white());
                }
            }
        }
        "message_stop" => {
            println!("\n{}", "─".repeat(80).dimmed());
        }
        "ping" => {
            // Silent ping
        }
        "error" => {
            if let Some(error) = event.get("error") {
                let error_msg = error.get("message").and_then(|v| v.as_str()).unwrap_or("Unknown error");
                println!("{} {}", "❌ Error:".red(), error_msg);
            }
        }
        _ => {
            println!("{} {}", "Unknown event:".yellow(), event_type);
        }
    }
}

fn display_regular_log(path: &PathBuf, _follow: bool) -> Result<()> {
    let content = fs::read_to_string(path)?;
    println!("\n{}", content);
    Ok(())
}
