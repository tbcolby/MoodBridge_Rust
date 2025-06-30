#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! regex = "1.0"
//! chrono = { version = "0.4", features = ["serde"] }
//! serde_json = "1.0"
//! ```

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use chrono::{DateTime, Utc, NaiveDateTime};

#[derive(Debug)]
struct CommandExecution {
    timestamp: DateTime<Utc>,
    command: String,
    args: Vec<String>,
    working_directory: String,
    exit_code: Option<i32>,
}

struct WarpLogAnalyzer {
    log_path: String,
    timestamp_regex: Regex,
}

impl WarpLogAnalyzer {
    fn new(log_path: &str) -> Self {
        Self {
            log_path: log_path.to_string(),
            timestamp_regex: Regex::new(r"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2})").unwrap(),
        }
    }

    fn analyze_todays_activity(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(&self.log_path)?;
        let reader = BufReader::new(file);
        
        let mut commands = Vec::new();
        let today = Utc::now().date_naive();
        
        println!("🔍 Analyzing Warp logs for today ({})...", today);
        println!("📁 Log file: {}", self.log_path);
        println!();

        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            
            // Look for command executions in the logs
            if line.contains("command") || line.contains("exec") {
                if let Some(timestamp) = self.extract_timestamp(&line) {
                    if timestamp.date_naive() == today {
                        if let Some(command) = self.extract_command_info(&line) {
                            commands.push(command);
                        }
                    }
                }
            }
            
            // Show progress every 10,000 lines
            if line_num % 10000 == 0 && line_num > 0 {
                println!("📊 Processed {} lines...", line_num);
            }
        }

        println!("✅ Analysis complete!");
        println!("📈 Found {} command executions today", commands.len());
        println!();

        // Analyze patterns
        self.analyze_patterns(&commands);
        
        Ok(())
    }

    fn extract_timestamp(&self, line: &str) -> Option<DateTime<Utc>> {
        if let Some(caps) = self.timestamp_regex.captures(line) {
            let timestamp_str = &caps[1];
            // Try parsing with Z suffix first
            DateTime::parse_from_rfc3339(&format!("{}Z", timestamp_str))
                .map(|dt| dt.with_timezone(&Utc))
                .ok()
        } else {
            None
        }
    }

    fn extract_command_info(&self, line: &str) -> Option<CommandExecution> {
        // Simple pattern matching for common command patterns
        let timestamp = self.extract_timestamp(line)?;
        
        // Look for various command patterns in Warp logs
        if line.contains("cargo") {
            return Some(CommandExecution {
                timestamp,
                command: "cargo".to_string(),
                args: vec!["build".to_string()], // Simplified
                working_directory: "unknown".to_string(),
                exit_code: None,
            });
        }
        
        if line.contains("git") {
            return Some(CommandExecution {
                timestamp,
                command: "git".to_string(),
                args: vec!["status".to_string()], // Simplified
                working_directory: "unknown".to_string(),
                exit_code: None,
            });
        }

        if line.contains("ls") || line.contains("find") {
            return Some(CommandExecution {
                timestamp,
                command: "ls".to_string(),
                args: vec![],
                working_directory: "unknown".to_string(),
                exit_code: None,
            });
        }

        None
    }

    fn analyze_patterns(&self, commands: &[CommandExecution]) {
        if commands.is_empty() {
            println!("❌ No commands found in today's logs");
            return;
        }

        println!("🎯 WARP COMMAND Analysis Results");
        println!("================================");
        
        // Command frequency
        let mut command_freq: HashMap<String, usize> = HashMap::new();
        for cmd in commands {
            *command_freq.entry(cmd.command.clone()).or_insert(0) += 1;
        }

        println!("📊 Command Frequency:");
        for (cmd, count) in command_freq.iter() {
            println!("   {} x{}", cmd, count);
        }
        println!();

        // Time analysis
        if let (Some(first), Some(last)) = (commands.first(), commands.last()) {
            let duration = last.timestamp.signed_duration_since(first.timestamp);
            println!("⏰ Active Time Analysis:");
            println!("   First command: {}", first.timestamp.format("%H:%M:%S"));
            println!("   Last command: {}", last.timestamp.format("%H:%M:%S"));
            println!("   Total span: {} hours {} minutes", 
                duration.num_hours(), 
                duration.num_minutes() % 60
            );
            println!();
        }

        // Development insights
        println!("💡 Development Insights:");
        
        let git_count = command_freq.get("git").unwrap_or(&0);
        let cargo_count = command_freq.get("cargo").unwrap_or(&0);
        let exploration_count = command_freq.get("ls").unwrap_or(&0);

        if *git_count > 5 {
            println!("   🔧 High git activity - active version control usage");
        }
        if *cargo_count > 3 {
            println!("   🦀 Multiple cargo builds - Rust development session");
        }
        if *exploration_count > 10 {
            println!("   🔍 High exploration activity - discovering project structure");
        }

        println!("   🎯 Total commands executed: {}", commands.len());
        
        if commands.len() > 100 {
            println!("   🔥 High activity day - significant development work!");
        } else if commands.len() > 50 {
            println!("   💪 Good productive session");
        } else {
            println!("   🌱 Light development activity");
        }

        println!();
        println!("🚀 WARP COMMAND Report Complete!");
        println!("💌 Next step: Set up email reporting for daily summaries");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 WARP COMMAND - Development Intelligence System");
    println!("==================================================");
    println!();

    // Use the Warp log path
    let log_path = format!("{}/Library/Logs/warp.log", 
        std::env::var("HOME").unwrap_or_else(|_| "/Users/tyler".to_string()));

    let analyzer = WarpLogAnalyzer::new(&log_path);
    analyzer.analyze_todays_activity()?;

    Ok(())
}
