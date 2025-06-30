use super::*;
use super::log_analyzer::CommandExecution;
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

pub struct ActivityTracker {
    session_gap_minutes: i64,
    deep_work_threshold_minutes: i64,
    context_switch_threshold_seconds: i64,
}

impl ActivityTracker {
    pub fn new() -> Self {
        Self {
            session_gap_minutes: 30,      // 30 minute gap = new session
            deep_work_threshold_minutes: 45, // 45+ minutes = deep work
            context_switch_threshold_seconds: 30, // <30 seconds between commands = context switch
        }
    }

    /// Process raw command executions into structured development activity
    pub async fn process_activities(&self, commands: &[CommandExecution]) -> Result<DevelopmentActivity, WarpCommandError> {
        let date = Utc::now();
        
        // Analyze command patterns
        let command_activities = self.analyze_commands(commands);
        
        // Detect file modifications
        let file_activities = self.detect_file_activities(commands);
        
        // Analyze project focus
        let project_focus = self.analyze_project_focus(commands);
        
        // Calculate productivity metrics
        let productivity_metrics = self.calculate_productivity_metrics(commands);
        
        // Generate initial AI insights
        let ai_insights = self.generate_activity_insights(commands, &productivity_metrics);

        Ok(DevelopmentActivity {
            date,
            commands_executed: command_activities,
            files_modified: file_activities,
            project_focus,
            productivity_metrics,
            ai_insights,
        })
    }

    fn analyze_commands(&self, commands: &[CommandExecution]) -> Vec<CommandActivity> {
        let mut command_map: HashMap<String, Vec<&CommandExecution>> = HashMap::new();
        
        // Group commands by name
        for cmd in commands {
            command_map.entry(cmd.command.clone()).or_default().push(cmd);
        }

        // Analyze each command
        command_map.into_iter().map(|(command, executions)| {
            let frequency = executions.len() as u32;
            let first_used = executions.iter().map(|e| e.timestamp).min().unwrap_or(Utc::now());
            let last_used = executions.iter().map(|e| e.timestamp).max().unwrap_or(Utc::now());
            
            // Calculate success rate
            let successful = executions.iter()
                .filter(|e| e.exit_code.unwrap_or(0) == 0)
                .count();
            let success_rate = if frequency > 0 {
                successful as f64 / frequency as f64
            } else {
                1.0
            };

            // Extract contexts
            let context = self.extract_command_context(&executions);

            CommandActivity {
                command,
                frequency,
                first_used,
                last_used,
                success_rate,
                context,
            }
        }).collect()
    }

    fn extract_command_context(&self, executions: &[&CommandExecution]) -> Vec<String> {
        let mut contexts = Vec::new();
        
        // Analyze working directories
        let dirs: std::collections::HashSet<_> = executions.iter()
            .map(|e| &e.working_directory)
            .collect();
        
        if dirs.len() == 1 {
            contexts.push(format!("Focused on single directory: {}", dirs.iter().next().unwrap()));
        } else if dirs.len() > 5 {
            contexts.push("Multi-project work session".to_string());
        }

        // Analyze command arguments for patterns
        let all_args: Vec<String> = executions.iter()
            .flat_map(|e| &e.args)
            .cloned()
            .collect();

        if all_args.iter().any(|arg| arg.contains(".rs")) {
            contexts.push("Rust development".to_string());
        }
        if all_args.iter().any(|arg| arg.contains(".py")) {
            contexts.push("Python development".to_string());
        }
        if all_args.iter().any(|arg| arg.contains(".js") || arg.contains(".ts")) {
            contexts.push("JavaScript/TypeScript development".to_string());
        }

        contexts
    }

    fn detect_file_activities(&self, commands: &[CommandExecution]) -> Vec<FileActivity> {
        let mut file_activities = Vec::new();
        
        // Look for file editing commands
        let editing_commands = ["vim", "nvim", "nano", "emacs", "code", "subl"];
        
        for cmd in commands {
            if editing_commands.contains(&cmd.command.as_str()) {
                // Extract file paths from arguments
                for arg in &cmd.args {
                    if self.is_likely_file_path(arg) {
                        let language = self.detect_language_from_extension(arg);
                        
                        file_activities.push(FileActivity {
                            file_path: arg.clone(),
                            language,
                            modifications: 1, // We'll aggregate these later
                            lines_added: None, // Would need git diff to determine
                            lines_removed: None,
                            first_modified: cmd.timestamp,
                            last_modified: cmd.timestamp,
                        });
                    }
                }
            }
        }

        // Aggregate multiple modifications of the same file
        self.aggregate_file_activities(file_activities)
    }

    fn is_likely_file_path(&self, arg: &str) -> bool {
        // Simple heuristic: contains a dot and doesn't start with -
        arg.contains('.') && !arg.starts_with('-') && !arg.contains(' ')
    }

    fn detect_language_from_extension(&self, file_path: &str) -> String {
        if let Some(ext) = file_path.split('.').last() {
            match ext {
                "rs" => "Rust".to_string(),
                "py" => "Python".to_string(),
                "js" => "JavaScript".to_string(),
                "ts" => "TypeScript".to_string(),
                "go" => "Go".to_string(),
                "java" => "Java".to_string(),
                "cpp" | "cc" | "cxx" => "C++".to_string(),
                "c" => "C".to_string(),
                "h" | "hpp" => "Header".to_string(),
                "md" => "Markdown".to_string(),
                "html" => "HTML".to_string(),
                "css" => "CSS".to_string(),
                "json" => "JSON".to_string(),
                "yaml" | "yml" => "YAML".to_string(),
                "toml" => "TOML".to_string(),
                _ => "Unknown".to_string(),
            }
        } else {
            "Unknown".to_string()
        }
    }

    fn aggregate_file_activities(&self, activities: Vec<FileActivity>) -> Vec<FileActivity> {
        let mut file_map: HashMap<String, Vec<FileActivity>> = HashMap::new();
        
        for activity in activities {
            file_map.entry(activity.file_path.clone()).or_default().push(activity);
        }

        file_map.into_iter().map(|(file_path, mut activities)| {
            activities.sort_by_key(|a| a.first_modified);
            
            let first_modified = activities.first().unwrap().first_modified;
            let last_modified = activities.last().unwrap().last_modified;
            let modifications = activities.len() as u32;
            let language = activities.first().unwrap().language.clone();

            FileActivity {
                file_path,
                language,
                modifications,
                lines_added: None,
                lines_removed: None,
                first_modified,
                last_modified,
            }
        }).collect()
    }

    fn analyze_project_focus(&self, commands: &[CommandExecution]) -> Vec<ProjectFocus> {
        let mut project_map: HashMap<String, Vec<&CommandExecution>> = HashMap::new();
        
        // Group commands by working directory (project)
        for cmd in commands {
            project_map.entry(cmd.working_directory.clone()).or_default().push(cmd);
        }

        project_map.into_iter().map(|(project_path, cmds)| {
            let project_name = self.extract_project_name(&project_path);
            let time_spent_minutes = self.calculate_time_spent(&cmds);
            let activity_type = self.classify_activity_type(&cmds);
            let complexity_score = self.calculate_complexity_score(&cmds);

            ProjectFocus {
                project_name,
                project_path,
                time_spent_minutes,
                activity_type,
                complexity_score,
            }
        }).collect()
    }

    fn extract_project_name(&self, path: &str) -> String {
        path.split('/').last().unwrap_or("unknown").to_string()
    }

    fn calculate_time_spent(&self, commands: &[&CommandExecution]) -> u32 {
        if commands.is_empty() {
            return 0;
        }

        let start = commands.iter().map(|c| c.timestamp).min().unwrap();
        let end = commands.iter().map(|c| c.timestamp).max().unwrap();
        
        let duration = end.signed_duration_since(start);
        duration.num_minutes().max(0) as u32
    }

    fn classify_activity_type(&self, commands: &[&CommandExecution]) -> String {
        let command_counts: HashMap<&str, usize> = commands.iter()
            .map(|c| c.command.as_str())
            .fold(HashMap::new(), |mut acc, cmd| {
                *acc.entry(cmd).or_insert(0) += 1;
                acc
            });

        // Classify based on dominant command types
        if command_counts.get("git").unwrap_or(&0) > &5 {
            "Version Control".to_string()
        } else if command_counts.get("cargo").unwrap_or(&0) > &3 {
            "Building/Testing".to_string()
        } else if command_counts.get("vim").unwrap_or(&0) > &0 || 
                  command_counts.get("nvim").unwrap_or(&0) > &0 ||
                  command_counts.get("code").unwrap_or(&0) > &0 {
            "Coding".to_string()
        } else if command_counts.get("grep").unwrap_or(&0) > &5 ||
                  command_counts.get("find").unwrap_or(&0) > &5 {
            "Research/Debugging".to_string()
        } else {
            "General Development".to_string()
        }
    }

    fn calculate_complexity_score(&self, commands: &[&CommandExecution]) -> f64 {
        let mut score = 0.0;
        
        // Base score on command diversity and frequency
        let unique_commands = commands.iter()
            .map(|c| &c.command)
            .collect::<std::collections::HashSet<_>>()
            .len();
        
        score += unique_commands as f64 * 0.1;
        
        // Add complexity for specific command types
        for cmd in commands {
            match cmd.command.as_str() {
                "git" if cmd.args.contains(&"rebase".to_string()) => score += 0.8,
                "git" if cmd.args.contains(&"merge".to_string()) => score += 0.6,
                "cargo" if cmd.args.contains(&"test".to_string()) => score += 0.4,
                "grep" | "awk" | "sed" => score += 0.3,
                "find" => score += 0.2,
                "vim" | "nvim" => score += 0.5,
                _ => score += 0.1,
            }
        }
        
        // Error rate increases complexity
        let error_rate = commands.iter()
            .filter(|c| c.exit_code.unwrap_or(0) != 0)
            .count() as f64 / commands.len() as f64;
        score += error_rate * 2.0;
        
        // Cap at 10.0
        score.min(10.0)
    }

    fn calculate_productivity_metrics(&self, commands: &[CommandExecution]) -> ProductivityMetrics {
        let total_active_time_minutes = self.calculate_total_active_time(commands);
        let deep_work_sessions = self.count_deep_work_sessions(commands);
        let context_switches = self.count_context_switches(commands);
        let error_rate = self.calculate_error_rate(commands);
        let learning_indicators = self.extract_learning_indicators(commands);
        let efficiency_score = self.calculate_efficiency_score(commands);

        ProductivityMetrics {
            total_active_time_minutes,
            deep_work_sessions,
            context_switches,
            error_rate,
            learning_indicators,
            efficiency_score,
        }
    }

    fn calculate_total_active_time(&self, commands: &[CommandExecution]) -> u32 {
        if commands.is_empty() {
            return 0;
        }

        let mut total_minutes = 0;
        let mut last_timestamp = None;

        for cmd in commands {
            if let Some(last) = last_timestamp {
                let gap = cmd.timestamp.signed_duration_since(last);
                if gap.num_minutes() <= self.session_gap_minutes {
                    total_minutes += gap.num_minutes();
                }
            }
            last_timestamp = Some(cmd.timestamp);
        }

        total_minutes.max(0) as u32
    }

    fn count_deep_work_sessions(&self, commands: &[CommandExecution]) -> u32 {
        let mut sessions = 0;
        let mut session_start = None;

        for cmd in commands {
            if let Some(start) = session_start {
                let gap = cmd.timestamp.signed_duration_since(start);
                if gap.num_minutes() > self.session_gap_minutes {
                    // Session ended, check if it was deep work
                    if gap.num_minutes() >= self.deep_work_threshold_minutes {
                        sessions += 1;
                    }
                    session_start = Some(cmd.timestamp);
                }
            } else {
                session_start = Some(cmd.timestamp);
            }
        }

        sessions
    }

    fn count_context_switches(&self, commands: &[CommandExecution]) -> u32 {
        let mut switches = 0;
        let mut last_directory: Option<&String> = None;

        for cmd in commands {
            if let Some(last_dir) = &last_directory {
                if last_dir.as_str() != cmd.working_directory {
                    switches += 1;
                }
            }
            last_directory = Some(&cmd.working_directory);
        }

        switches
    }

    fn calculate_error_rate(&self, commands: &[CommandExecution]) -> f64 {
        if commands.is_empty() {
            return 0.0;
        }

        let errors = commands.iter()
            .filter(|c| c.exit_code.unwrap_or(0) != 0)
            .count();

        errors as f64 / commands.len() as f64
    }

    fn extract_learning_indicators(&self, commands: &[CommandExecution]) -> Vec<String> {
        let mut indicators = Vec::new();

        // Help command usage
        let help_count = commands.iter()
            .filter(|c| c.args.contains(&"--help".to_string()) || c.command == "man")
            .count();

        if help_count > 0 {
            indicators.push(format!("Used help/man {} times", help_count));
        }

        // Exploration patterns
        let exploration_commands = ["ls", "find", "grep", "cat", "less"];
        let exploration_count = commands.iter()
            .filter(|c| exploration_commands.contains(&c.command.as_str()))
            .count();

        if exploration_count > 20 {
            indicators.push("High exploration activity detected".to_string());
        }

        // New tool usage
        let tools_used: std::collections::HashSet<_> = commands.iter()
            .map(|c| &c.command)
            .collect();

        if tools_used.len() > 15 {
            indicators.push(format!("Used {} different tools", tools_used.len()));
        }

        indicators
    }

    fn calculate_efficiency_score(&self, commands: &[CommandExecution]) -> f64 {
        if commands.is_empty() {
            return 0.0;
        }

        let mut score = 5.0; // Base score

        // Reduce score for high error rate
        let error_rate = self.calculate_error_rate(commands);
        score -= error_rate * 3.0;

        // Increase score for tool diversity
        let unique_tools = commands.iter()
            .map(|c| &c.command)
            .collect::<std::collections::HashSet<_>>()
            .len();
        score += (unique_tools as f64 / 10.0).min(2.0);

        // Increase score for git usage (version control)
        let git_usage = commands.iter()
            .filter(|c| c.command == "git")
            .count() as f64 / commands.len() as f64;
        score += git_usage * 2.0;

        score.max(0.0).min(10.0)
    }

    fn generate_activity_insights(&self, commands: &[CommandExecution], metrics: &ProductivityMetrics) -> Vec<ActivityInsight> {
        let mut insights = Vec::new();

        // Productivity trend insight
        if metrics.efficiency_score > 7.0 {
            insights.push(ActivityInsight {
                insight_type: InsightType::ProductivityTrend,
                confidence: 0.8,
                message: "High productivity session detected!".to_string(),
                data: serde_json::json!({
                    "efficiency_score": metrics.efficiency_score,
                    "deep_work_sessions": metrics.deep_work_sessions
                }),
                recommendations: vec![
                    "Consider documenting what made this session so productive".to_string(),
                    "Try to replicate these conditions tomorrow".to_string()
                ],
                priority: Priority::Medium,
            });
        }

        // Learning pattern insight
        if !metrics.learning_indicators.is_empty() {
            insights.push(ActivityInsight {
                insight_type: InsightType::LearningPattern,
                confidence: 0.9,
                message: "Active learning detected in this session".to_string(),
                data: serde_json::json!({
                    "indicators": metrics.learning_indicators
                }),
                recommendations: vec![
                    "Document key learnings before they're forgotten".to_string(),
                    "Consider creating notes or examples for future reference".to_string()
                ],
                priority: Priority::High,
            });
        }

        // Context switching insight
        if metrics.context_switches > 10 {
            insights.push(ActivityInsight {
                insight_type: InsightType::WorkflowEfficiency,
                confidence: 0.7,
                message: "High context switching detected".to_string(),
                data: serde_json::json!({
                    "context_switches": metrics.context_switches,
                    "recommendation": "Consider focusing on one project at a time"
                }),
                recommendations: vec![
                    "Try time-boxing focus sessions".to_string(),
                    "Use separate terminal sessions for different projects".to_string()
                ],
                priority: Priority::Medium,
            });
        }

        insights
    }
}
