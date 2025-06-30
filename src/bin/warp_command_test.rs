use moodbridge_rust::warp_command::{WarpCommandConfig, email_service::EmailService};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    println!("🚀 WARP COMMAND Test System");
    println!("=============================");
    
    // Create configuration
    let mut config = WarpCommandConfig::default();
    
    // Try to get SMTP credentials from environment
    if let Ok(smtp_user) = env::var("SMTP_USERNAME") {
        config.smtp_username = smtp_user;
    } else {
        println!("⚠️  No SMTP_USERNAME environment variable found");
        println!("💡 For testing, using Gmail SMTP. Set these environment variables:");
        println!("   export SMTP_USERNAME=your-email@gmail.com");
        println!("   export SMTP_PASSWORD=your-app-password");
        println!();
        
        // For demo purposes, use a placeholder (will fail but show structure)
        config.smtp_username = "demo@example.com".to_string();
        config.smtp_password = "demo-password".to_string();
    }
    
    if let Ok(smtp_pass) = env::var("SMTP_PASSWORD") {
        config.smtp_password = smtp_pass;
    }
    
    println!("📧 Email Configuration:");
    println!("   SMTP Host: {}", config.smtp_host);
    println!("   SMTP Port: {}", config.smtp_port);
    println!("   From: {}", config.email_sender);
    println!("   To: {}", config.email_recipient);
    println!("   Username: {}", config.smtp_username);
    println!("   Password: {}***", config.smtp_password.chars().take(3).collect::<String>());
    println!();

    // Create WARP COMMAND service
    match WarpCommandService::new(config) {
        Ok(service) => {
            println!("✅ WARP COMMAND service created successfully");
            println!("🚀 Sending test email...");
            
            match service.email_service.send_test_email().await {
                Ok(_) => {
                    println!("🎉 SUCCESS! Test email sent to tbcolby@pm.me");
                    println!("📬 Check your email for the WARP COMMAND test message");
                },
                Err(e) => {
                    println!("❌ Failed to send test email: {}", e);
                    println!();
                    println!("🔧 Troubleshooting tips:");
                    println!("   1. Make sure SMTP_USERNAME and SMTP_PASSWORD are set");
                    println!("   2. For Gmail, use an App Password (not your regular password)");
                    println!("   3. Enable 2-factor authentication and generate an App Password");
                    println!("   4. Check firewall settings for SMTP port 587");
                }
            }
        },
        Err(e) => {
            println!("❌ Failed to create WARP COMMAND service: {}", e);
            println!();
            println!("🔧 Setup required:");
            println!("   export SMTP_USERNAME=your-email@gmail.com");
            println!("   export SMTP_PASSWORD=your-app-password");
        }
    }
    
    println!();
    println!("🦀 WARP COMMAND Test Complete");
    Ok(())
}
