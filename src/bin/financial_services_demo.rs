// Financial Services Demo Application
// Built on top of MoodBridge_Rust platform

use std::io::{self, Write};
use moodbridge_rust::demo_app::{FinancialServicesDemo, ScenarioCategory, ComplexityLevel};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Clear screen and show banner
    print!("\x1B[2J\x1B[1;1H");
    
    show_banner();
    
    // Initialize the demo application
    let mut demo = FinancialServicesDemo::new();
    
    println!("🚀 Initializing Financial Services Compliance Demo...\n");
    
    match demo.initialize().await {
        Ok(_) => {
            println!("✅ Demo application initialized successfully!\n");
            run_main_menu(&mut demo).await?;
        },
        Err(e) => {
            println!("❌ Failed to initialize demo application: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}

fn show_banner() {
    println!(r#"
╔═══════════════════════════════════════════════════════════════════════════════╗
║                                                                               ║
║    🏦 FINANCIAL SERVICES COMPLIANCE DEMO                                     ║
║    Built on MoodBridge_Rust Platform                                         ║
║                                                                               ║
║    🔒 Data Compliance & Archival Solutions                                   ║
║    📊 Real-time Monitoring & Reporting                                       ║
║    ⚖️  Regulatory Framework Management                                        ║
║    🔗 Enterprise Integration Platform                                         ║
║                                                                               ║
╚═══════════════════════════════════════════════════════════════════════════════╝
    "#);
}

async fn run_main_menu(demo: &mut FinancialServicesDemo) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        show_main_menu();
        
        let choice = get_user_input("Enter your choice (1-6): ")?;
        
        match choice.trim() {
            "1" => show_demo_scenarios(demo).await?,
            "2" => show_system_status(demo).await?,
            "3" => show_compliance_dashboard(demo).await?,
            "4" => run_interactive_demo(demo).await?,
            "5" => show_help(),
            "6" => {
                println!("\n👋 Thank you for using the Financial Services Compliance Demo!");
                println!("🔗 Visit https://github.com/your-repo for more information\n");
                break;
            },
            _ => {
                println!("❌ Invalid choice. Please enter a number between 1-6.\n");
            }
        }
        
        println!("\nPress Enter to continue...");
        let _ = get_user_input("")?;
    }
    
    Ok(())
}

fn show_main_menu() {
    print!("\x1B[2J\x1B[1;1H");
    println!("╔═══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                          MAIN DEMO MENU                                      ║");
    println!("╠═══════════════════════════════════════════════════════════════════════════════╣");
    println!("║                                                                               ║");
    println!("║  1. 📋 View Available Demo Scenarios                                         ║");
    println!("║  2. 🏥 System Status & Health Check                                          ║");
    println!("║  3. 📊 Compliance Dashboard                                                   ║");
    println!("║  4. 🎮 Run Interactive Demo                                                   ║");
    println!("║  5. ❓ Help & Documentation                                                   ║");
    println!("║  6. 🚪 Exit                                                                   ║");
    println!("║                                                                               ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════╝\n");
}

async fn show_demo_scenarios(demo: &FinancialServicesDemo) -> Result<(), Box<dyn std::error::Error>> {
    print!("\x1B[2J\x1B[1;1H");
    println!("╔═══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                         AVAILABLE DEMO SCENARIOS                             ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════╝\n");
    
    let scenarios = demo.list_scenarios();
    
    for (i, scenario) in scenarios.iter().enumerate() {
        println!("┌─────────────────────────────────────────────────────────────────────────────┐");
        println!("│ {}. {:<75} │", i + 1, scenario.name);
        println!("├─────────────────────────────────────────────────────────────────────────────┤");
        println!("│ Description: {:<63} │", truncate_string(&scenario.description, 63));
        println!("│ Category:    {:<63} │", format!("{:?}", scenario.category));
        println!("│ Complexity:  {:<63} │", format!("{:?}", scenario.complexity_level));
        println!("│ Duration:    {:<63} │", scenario.estimated_duration);
        println!("│ Records:     {:<63} │", format!("{} records", scenario.data_volume.record_count));
        println!("│ Frameworks:  {:<63} │", scenario.compliance_frameworks.join(", "));
        println!("└─────────────────────────────────────────────────────────────────────────────┘\n");
    }
    
    println!("📋 Total Scenarios Available: {}", scenarios.len());
    println!("🎯 Categories: Compliance Setup, Data Archival, Risk Assessment, Integration");
    println!("📈 Complexity Levels: Basic, Intermediate, Advanced, Expert, Enterprise\n");
    
    Ok(())
}

async fn show_system_status(demo: &FinancialServicesDemo) -> Result<(), Box<dyn std::error::Error>> {
    print!("\x1B[2J\x1B[1;1H");
    println!("╔═══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                           SYSTEM STATUS                                      ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════╝\n");
    
    println!("🔧 CORE COMPONENTS STATUS:");
    println!("├─ 🔒 Compliance Engine:     ✅ Online");
    println!("├─ 📋 Audit Trail Manager:   ✅ Online");
    println!("├─ 🗄️  Data Archival System: ✅ Online");
    println!("├─ 📊 Reporting Dashboard:   ✅ Online");
    println!("├─ 🔗 Integration Manager:   ✅ Online");
    println!("├─ ⚠️  Risk Assessment:      ✅ Online");
    println!("└─ ⚖️  Regulatory Framework: ✅ Online\n");
    
    println!("📊 SYSTEM METRICS:");
    println!("├─ Application ID: {}", demo.app_id);
    println!("├─ Version:       {}", demo.version);
    println!("├─ Uptime:        {}", format_uptime(&demo.created_at));
    println!("├─ Memory Usage:  ~15.2 MB");
    println!("├─ CPU Usage:     2.3%");
    println!("└─ Network:       Connected\n");
    
    println!("🔗 INTEGRATION STATUS:");
    println!("├─ Salesforce:    🟡 Ready (Not Connected)");
    println!("├─ AWS:           🟡 Ready (Not Connected)");
    println!("├─ Azure:         🟡 Ready (Not Connected)");
    println!("├─ Snowflake:     🟡 Ready (Not Connected)");
    println!("└─ Database:      ✅ Connected\n");
    
    println!("📈 COMPLIANCE STATUS:");
    println!("├─ SOX Framework:  ✅ Configured");
    println!("├─ GDPR Framework: ✅ Configured");
    println!("├─ CCPA Framework: ✅ Configured");
    println!("├─ Active Rules:   {} rules", demo.compliance_engine.rules.len());
    println!("└─ Audit Entries:  {} entries\n", demo.audit_trail.audit_logs.len());
    
    Ok(())
}

async fn show_compliance_dashboard(demo: &FinancialServicesDemo) -> Result<(), Box<dyn std::error::Error>> {
    print!("\x1B[2J\x1B[1;1H");
    println!("╔═══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                         COMPLIANCE DASHBOARD                                 ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════╝\n");
    
    println!("🎯 COMPLIANCE SCORE OVERVIEW:");
    println!("┌─────────────────────────────────────────────────────────────────────────────┐");
    println!("│                                                                             │");
    println!("│  Overall Compliance Score: 95.2% 🟢                                        │");
    println!("│                                                                             │");
    println!("│  ████████████████████████████████████████████████████████████████████░░░   │");
    println!("│                                                                             │");
    println!("└─────────────────────────────────────────────────────────────────────────────┘\n");
    
    println!("📊 FRAMEWORK BREAKDOWN:");
    println!("├─ SOX (Sarbanes-Oxley):     97.8% ✅ [Excellent]");
    println!("├─ GDPR (Data Protection):   94.1% ✅ [Good]");
    println!("├─ CCPA (Privacy Rights):    93.7% ✅ [Good]");
    println!("└─ Internal Policies:        96.4% ✅ [Excellent]\n");
    
    println!("⚠️  RECENT ALERTS:");
    println!("├─ 🟡 Data retention policy expires in 30 days (Customer Records)");
    println!("├─ 🟡 Audit trail review required (Weekly Schedule)");
    println!("└─ 🟢 All critical compliance checks passed\n");
    
    println!("📈 AUDIT ACTIVITY (Last 7 Days):");
    println!("├─ Compliance Scans:     12 completed");
    println!("├─ Data Validations:     45 successful");
    println!("├─ Policy Updates:       3 applied");
    println!("├─ Risk Assessments:     2 conducted");
    println!("└─ Reports Generated:    8 distributed\n");
    
    println!("🔄 AUTOMATED PROCESSES:");
    println!("├─ Daily Compliance Scan:      ✅ Scheduled");
    println!("├─ Weekly Audit Report:        ✅ Scheduled");
    println!("├─ Monthly Risk Assessment:    ✅ Scheduled");
    println!("└─ Quarterly Review:           ✅ Scheduled\n");
    
    Ok(())
}

async fn run_interactive_demo(demo: &mut FinancialServicesDemo) -> Result<(), Box<dyn std::error::Error>> {
    print!("\x1B[2J\x1B[1;1H");
    println!("╔═══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                        INTERACTIVE DEMO RUNNER                               ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════╝\n");
    
    let scenarios = demo.list_scenarios();
    
    println!("📋 Available Demo Scenarios:\n");
    for (i, scenario) in scenarios.iter().enumerate() {
        println!("{}. {} ({})", 
                 i + 1, 
                 scenario.name, 
                 scenario.estimated_duration);
        println!("   └─ {}", truncate_string(&scenario.description, 70));
        println!();
    }
    
    let choice = get_user_input("Select a scenario to run (1-2, or 'q' to quit): ")?;
    
    match choice.trim() {
        "1" => {
            println!("\n🚀 Starting Basic Compliance Setup Demo...\n");
            let result = demo.execute_scenario("basic_compliance_setup").await?;
            show_execution_result(&result);
        },
        "2" => {
            println!("\n🚀 Starting Advanced Salesforce Archival Demo...\n");
            let result = demo.execute_scenario("advanced_salesforce_archival").await?;
            show_execution_result(&result);
        },
        "q" | "Q" => {
            println!("🔙 Returning to main menu...");
            return Ok(());
        },
        _ => {
            println!("❌ Invalid choice. Please enter 1, 2, or 'q'.");
        }
    }
    
    Ok(())
}

fn show_execution_result(result: &moodbridge_rust::demo_app::DemoExecutionResult) {
    println!("\n╔═══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                          DEMO EXECUTION RESULTS                              ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════╝\n");
    
    println!("📊 EXECUTION SUMMARY:");
    println!("├─ Scenario ID:       {}", result.scenario_id);
    println!("├─ Execution ID:      {}", result.execution_id);
    println!("├─ Status:            {:?}", result.status);
    println!("├─ Duration:          {:.2}s", result.duration_seconds.unwrap_or(0.0));
    println!("├─ Steps Completed:   {}", result.step_results.len());
    println!("└─ Errors:            {}\n", result.error_log.len());
    
    println!("🔍 STEP DETAILS:");
    for step in &result.step_results {
        let status_icon = match step.status {
            moodbridge_rust::demo_app::ExecutionStatus::Completed => "✅",
            moodbridge_rust::demo_app::ExecutionStatus::Failed => "❌",
            _ => "🔄",
        };
        println!("├─ Step {}: {} ({:.2}s) {}", 
                 step.step_number, 
                 status_icon,
                 step.duration_seconds.unwrap_or(0.0),
                 match step.status {
                     moodbridge_rust::demo_app::ExecutionStatus::Completed => "Success",
                     moodbridge_rust::demo_app::ExecutionStatus::Failed => "Failed",
                     _ => "Running",
                 });
    }
    
    println!("\n📈 PERFORMANCE METRICS:");
    println!("├─ Records Processed: {}", result.metrics.total_records_processed);
    println!("├─ Data Volume:       {:.1} MB", result.metrics.total_data_volume_mb);
    println!("├─ Avg Response:      {:.1} ms", result.performance_metrics.response_times.average_ms);
    println!("└─ Throughput:        {:.0} req/s\n", result.performance_metrics.throughput.requests_per_second);
    
    println!("🔒 COMPLIANCE VALIDATION:");
    println!("├─ Overall Score:     {:.1}%", result.compliance_validation.overall_compliance_score);
    println!("├─ Audit Complete:    {:.1}%", result.compliance_validation.audit_trail_completeness);
    println!("├─ Retention Policy:  {}", if result.compliance_validation.data_retention_compliance { "✅ Compliant" } else { "❌ Non-compliant" });
    println!("├─ Security:          {}", if result.compliance_validation.security_requirements_met { "✅ Met" } else { "❌ Failed" });
    println!("└─ Violations:        {}\n", result.compliance_validation.regulatory_violations.len());
    
    if !result.error_log.is_empty() {
        println!("⚠️  ERRORS:");
        for error in &result.error_log {
            println!("├─ {}: {}", error.error_type, error.message);
        }
        println!();
    }
}

fn show_help() {
    print!("\x1B[2J\x1B[1;1H");
    println!("╔═══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                          HELP & DOCUMENTATION                                ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════╝\n");
    
    println!("📖 ABOUT THIS DEMO:");
    println!("This Financial Services Compliance Demo showcases a comprehensive data");
    println!("management and compliance platform built on the MoodBridge_Rust framework.");
    println!("It demonstrates key capabilities inspired by enterprise data management");
    println!("solutions like Odaseva for Salesforce.\n");
    
    println!("🎯 KEY FEATURES:");
    println!("├─ 🔒 Compliance Management:  Multi-framework support (SOX, GDPR, CCPA)");
    println!("├─ 🗄️  Data Archival:         Automated retention policies and lifecycle mgmt");
    println!("├─ 📋 Audit Trails:          Comprehensive logging and audit capabilities");
    println!("├─ 📊 Real-time Reporting:   Interactive dashboards and automated reports");
    println!("├─ 🔗 Enterprise Integration: Salesforce, AWS, Azure, Snowflake connectivity");
    println!("├─ ⚠️  Risk Assessment:       Automated risk analysis and mitigation");
    println!("└─ ⚖️  Regulatory Framework:  Built-in compliance validation and monitoring\n");
    
    println!("🚀 DEMO SCENARIOS:");
    println!("1. Basic Compliance Setup:");
    println!("   └─ Demonstrates initial compliance framework configuration");
    println!("2. Advanced Salesforce Archival:");
    println!("   └─ Shows complex data archival with enterprise integration\n");
    
    println!("🔧 NAVIGATION:");
    println!("├─ Use number keys to select menu options");
    println!("├─ Press Enter to confirm selections");
    println!("├─ Type 'q' to quit from sub-menus");
    println!("└─ All operations are read-only and safe to execute\n");
    
    println!("🔗 TECHNICAL DETAILS:");
    println!("├─ Built with: Rust, Tokio, Serde, Chrono");
    println!("├─ Architecture: Modular, async, type-safe");
    println!("├─ Performance: <5ms response times, >1000 req/s");
    println!("└─ Security: OAuth2, TLS 1.3, encrypted storage\n");
}

fn get_user_input(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input)
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len-3])
    }
}

fn format_uptime(created_at: &chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(*created_at);
    
    let seconds = duration.num_seconds();
    let minutes = seconds / 60;
    let hours = minutes / 60;
    
    if hours > 0 {
        format!("{}h {}m", hours, minutes % 60)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds % 60)
    } else {
        format!("{}s", seconds)
    }
}
