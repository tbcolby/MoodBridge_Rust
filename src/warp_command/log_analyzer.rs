use super::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use chrono::{DateTime, Utc, TimeZone, Local, NaiveDateTime};
use serde_json::Value;

/// Raw log entry from Warp
#[derive(Debug, Clone)]
pub struct WarpLogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub component: String,
    pub message: String,
    pub data: Option<Value>,
}

/// Parsed command execution
#[derive(Debug, Clone)]
pub struct CommandExecution {
    pub timestamp: DateTime<Utc>,
    pub command: String,
    pub args: Vec<String>,
    pub working_directory: String,
    pub exit_code: Option<i32>,
    pub duration_ms: Option<u64>,
    pub output_size: Option<usize>,
}

/// File operation detected in logs
#[derive(Debug, Clone)]
pub struct FileOperation {
    pub timestamp: DateTime<Utc>,
    pub operation_type: String, // "create", "modify", "delete", "read"
    pub file_path: String,
    pub file_size: Option<u64>,
    pub tool_used: String, // "vim", "code", "nano", etc.
}

/// Development session boundaries
#[derive(Debug, Clone)]
pub struct DevelopmentSession {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub project_path: String,
    pub activity_type: String,
    pub intensity_score: f64,
}

pub struct WarpLogAnalyzer {
    log_path: String,
    command_regex: Regex,
    file_regex: Regex,
    timestamp_regex: Regex,
    git_regex: Regex,
    build_regex: Regex,
    error_regex: Regex,
}

impl WarpLogAnalyzer {
    pub fn new(log_path: &str) -> Result<Self, WarpCommandError> {
        Ok(Self {
            log_path: log_path.to_string(),
            command_regex: Regex::new(r#"command_execution.*"command":\s*"([^"]+)""#)?,
            file_regex: Regex::new(r#"file_operation.*"path":\s*"([^"]+)""#)?,
            timestamp_regex: Regex::new(r"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2})")?,
            git_regex: Regex::new(r"git\s+(add|commit|push|pull|checkout|branch|merge|rebase)")?,
            build_regex: Regex::new(r"(cargo|npm|yarn|make|cmake|rustc|gcc|clang)")?,
            error_regex: Regex::new(r"(error|Error|ERROR|failed|Failed|FAILED)")?,
        })
    }

    /// Analyze today's development activity from logs
    pub async fn analyze_daily_activity(&self) -> Result<Vec<CommandExecution>, WarpCommandError> {
        let mut file = File::open(&self.log_path)?;
        let reader = BufReader::new(file);
        
        let mut commands = Vec::new();
        let today = Utc::now().date_naive();
        
        for line in reader.lines() {
            let line = line?;
            if let Some(entry) = self.parse_log_line(&line)? {
                // Only include today's entries
                if entry.timestamp.date_naive() == today {
                    if let Some(command) = self.extract_command_execution(&entry) {
                        commands.push(command);
                    }
                }
            }
        }
        
        tracing::info!("Analyzed {} commands for today", commands.len());
        Ok(commands)
    }

    /// Parse a single log line into structured data
    fn parse_log_line(&self, line: &str) -> Result<Option<WarpLogEntry>, WarpCommandError> {
        // Extract timestamp
        let timestamp = if let Some(caps) = self.timestamp_regex.captures(line) {
            let timestamp_str = &caps[1];
            DateTime::parse_from_rfc3339(&format!("{}Z", timestamp_str))
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now())
        } else {
            return Ok(None);
        };

        // Extract log level and component
        let parts: Vec<&str> = line.split_whitespace().collect();
        let level = parts.get(1).unwrap_or(&"INFO").to_string();
        let component = parts.get(2).unwrap_or(&"unknown").to_string();
        
        // Extract message (everything after timestamp, level, component)
        let message = line.chars()
            .skip_while(|&c| c != ']')
            .skip(1)
            .collect::<String>()
            .trim()
            .to_string();

        // Try to parse JSON data if present
        let data = if message.contains('{') && message.contains('}') {
            serde_json::from_str::<Value>(&message).ok()
        } else {
            None
        };

        Ok(Some(WarpLogEntry {
            timestamp,
            level,
            component,
            message,
            data,
        }))
    }

    /// Extract command execution from log entry
    fn extract_command_execution(&self, entry: &WarpLogEntry) -> Option<CommandExecution> {
        // Check if this is a command execution log
        if !entry.message.contains("command") && !entry.message.contains("exec") {
            return None;
        }

        // Try to extract command from structured data first
        if let Some(data) = &entry.data {
            if let Some(command_obj) = data.get("command") {
                return self.parse_command_from_json(entry.timestamp, command_obj);
            }
        }

        // Fallback to regex parsing
        if let Some(caps) = self.command_regex.captures(&entry.message) {
            let command_line = &caps[1];
            let parts: Vec<String> = command_line.split_whitespace()
                .map(|s| s.to_string())
                .collect();
            
            if !parts.is_empty() {
                return Some(CommandExecution {
                    timestamp: entry.timestamp,
                    command: parts[0].clone(),
                    args: parts[1..].to_vec(),
                    working_directory: self.extract_working_directory(&entry.message),
                    exit_code: self.extract_exit_code(&entry.message),
                    duration_ms: self.extract_duration(&entry.message),
                    output_size: None,
                });
            }
        }

        None
    }

    fn parse_command_from_json(&self, timestamp: DateTime<Utc>, command_obj: &Value) -> Option<CommandExecution> {
        let command = command_obj.get("name")?.as_str()?.to_string();
        let args: Vec<String> = command_obj.get("args")?
            .as_array()?
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();

        Some(CommandExecution {
            timestamp,
            command,
            args,
            working_directory: command_obj.get("cwd")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string(),
            exit_code: command_obj.get("exit_code").and_then(|v| v.as_i64()).map(|v| v as i32),
            duration_ms: command_obj.get("duration_ms").and_then(|v| v.as_u64()),
            output_size: command_obj.get("output_size").and_then(|v| v.as_u64()).map(|v| v as usize),
        })
    }

    fn extract_working_directory(&self, message: &str) -> String {
        // Try to extract working directory from message
        if let Some(start) = message.find("cwd\":\"") {
            let start = start + 6;
            if let Some(end) = message[start..].find('"') {
                return message[start..start + end].to_string();
            }
        }
        "unknown".to_string()
    }

    fn extract_exit_code(&self, message: &str) -> Option<i32> {
        if let Some(start) = message.find("exit_code\":") {
            let start = start + 11;
            if let Some(end) = message[start..].find(&[',', '}'][..]) {
                return message[start..start + end].trim().parse().ok();
            }
        }
        None
    }

    fn extract_duration(&self, message: &str) -> Option<u64> {
        if let Some(start) = message.find("duration_ms\":") {
            let start = start + 13;
            if let Some(end) = message[start..].find(&[',', '}'][..]) {
                return message[start..start + end].trim().parse().ok();
            }
        }
        None
    }

    /// Analyze development patterns from commands
    pub fn analyze_development_patterns(&self, commands: &[CommandExecution]) -> HashMap<String, serde_json::Value> {
        let mut patterns = HashMap::new();

        // Command frequency analysis
        let mut command_freq: HashMap<String, u32> = HashMap::new();
        for cmd in commands {
            *command_freq.entry(cmd.command.clone()).or_insert(0) += 1;
        }
        patterns.insert("command_frequency".to_string(), serde_json::json!(command_freq));

        // Git activity analysis
        let git_commands: Vec<_> = commands.iter()
            .filter(|cmd| cmd.command == "git")
            .collect();
        patterns.insert("git_activity".to_string(), serde_json::json!({
            "total_git_commands": git_commands.len(),
            "git_operations": self.analyze_git_operations(&git_commands)
        }));

        // Build system analysis
        let build_commands: Vec<_> = commands.iter()
            .filter(|cmd| self.build_regex.is_match(&cmd.command))
            .collect();
        patterns.insert("build_activity".to_string(), serde_json::json!({
            "total_builds": build_commands.len(),
            "build_tools": self.analyze_build_tools(&build_commands)
        }));

        // Error analysis
        let error_commands: Vec<_> = commands.iter()
            .filter(|cmd| cmd.exit_code.unwrap_or(0) != 0)
            .collect();
        patterns.insert("error_analysis".to_string(), serde_json::json!({
            "total_errors": error_commands.len(),
            "error_rate": if !commands.is_empty() { 
                error_commands.len() as f64 / commands.len() as f64 
            } else { 0.0 },
            "common_error_commands": self.analyze_error_commands(&error_commands)
        }));

        // Working directory analysis
        let mut dir_activity: HashMap<String, u32> = HashMap::new();
        for cmd in commands {
            *dir_activity.entry(cmd.working_directory.clone()).or_insert(0) += 1;
        }
        patterns.insert("directory_activity".to_string(), serde_json::json!(dir_activity));

        patterns
    }

    fn analyze_git_operations(&self, git_commands: &[&CommandExecution]) -> HashMap<String, u32> {
        let mut operations: HashMap<String, u32> = HashMap::new();
        
        for cmd in git_commands {
            if !cmd.args.is_empty() {
                let operation = &cmd.args[0];
                *operations.entry(operation.clone()).or_insert(0) += 1;
            }
        }
        
        operations
    }

    fn analyze_build_tools(&self, build_commands: &[&CommandExecution]) -> HashMap<String, u32> {
        let mut tools: HashMap<String, u32> = HashMap::new();
        
        for cmd in build_commands {
            *tools.entry(cmd.command.clone()).or_insert(0) += 1;
        }
        
        tools
    }

    fn analyze_error_commands(&self, error_commands: &[&CommandExecution]) -> HashMap<String, u32> {
        let mut error_freq: HashMap<String, u32> = HashMap::new();
        
        for cmd in error_commands {
            *error_freq.entry(cmd.command.clone()).or_insert(0) += 1;
        }
        
        error_freq
    }

    /// Extract learning indicators from command patterns
    pub fn extract_learning_indicators(&self, commands: &[CommandExecution]) -> Vec<String> {
        let mut indicators = Vec::new();

        // Look for documentation/help commands
        let help_commands: Vec<_> = commands.iter()
            .filter(|cmd| {
                cmd.args.contains(&"help".to_string()) ||
                cmd.args.contains(&"--help".to_string()) ||
                cmd.command == "man" ||
                cmd.command == "info"
            })
            .collect();

        if !help_commands.is_empty() {
            indicators.push(format!("Sought help {} times - active learning detected", help_commands.len()));
        }

        // Look for exploration patterns
        let exploration_commands = ["ls", "find", "grep", "cat", "less", "head", "tail"];
        let exploration_count: usize = commands.iter()
            .filter(|cmd| exploration_commands.contains(&cmd.command.as_str()))
            .count();

        if exploration_count > 50 {
            indicators.push("High exploration activity - discovering codebase structure".to_string());
        }

        // Look for experimentation patterns (multiple attempts at same command)
        let mut command_attempts: HashMap<String, Vec<DateTime<Utc>>> = HashMap::new();
        for cmd in commands {
            command_attempts.entry(cmd.command.clone()).or_default().push(cmd.timestamp);
        }

        for (command, timestamps) in command_attempts {
            if timestamps.len() > 10 {
                indicators.push(format!("Intensive {} usage - deep diving into functionality", command));
            }
        }

        indicators
    }
}

impl From<regex::Error> for WarpCommandError {
    fn from(err: regex::Error) -> Self {
        WarpCommandError::LogParsingError(format!("Regex error: {}", err))
    }
}
