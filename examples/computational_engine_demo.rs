//! # Computational Engine Integration Demo
//! 
//! This example demonstrates how to use the computational engine plugin architecture
//! to integrate Wolfram Alpha and other mathematical/scientific computation tools.

use std::collections::HashMap;
use moodbridge_rust::integrations::{
    IntegrationConfig, CredentialConfig, ConnectionConfig, FeatureConfig,
    RateLimitConfig, RetryPolicyConfig, AuthenticationType, PlatformIntegration,
};
use moodbridge_rust::integrations::computational::{
    ComputationalEngineManager, ComputationalQuery, ComputationalCapability,
    RoutingStrategy, MathNotation, OutputFormat, QueryPriority,
};
use moodbridge_rust::integrations::engines::{WolframAlphaEngine, WolframAlphaConfig};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    println!("🧮 Computational Engine Integration Demo");
    println!("==========================================");
    
    // Configure Wolfram Alpha engine
    let wolfram_config = WolframAlphaConfig {
        app_id: "YOUR_WOLFRAM_ALPHA_APP_ID".to_string(), // Replace with actual App ID
        base_url: "https://api.wolframalpha.com/v2/query".to_string(),
        timeout_seconds: 30,
        rate_limit_per_hour: Some(2000),
        enable_step_by_step: true,
        enable_plots: true,
        preferred_units: Some("metric".to_string()),
    };
    
    // Create the Wolfram Alpha engine
    let mut wolfram_engine = WolframAlphaEngine::new(wolfram_config);
    
    // Create integration config for initialization
    let mut custom_settings = HashMap::new();
    custom_settings.insert("app_id".to_string(), 
        serde_json::Value::String("YOUR_WOLFRAM_ALPHA_APP_ID".to_string()));
    
    let integration_config = IntegrationConfig {
        platform: "wolfram_alpha".to_string(),
        enabled: true,
        credentials: CredentialConfig {
            auth_type: AuthenticationType::ApiKey,
            client_id: None,
            client_secret: None,
            tenant_id: None,
            private_key_path: None,
            token_endpoint: None,
            scope: None,
        },
        connection: ConnectionConfig {
            base_url: "https://api.wolframalpha.com".to_string(),
            api_version: Some("v2".to_string()),
            region: None,
            pool_size: Some(5),
            keep_alive: true,
            use_tls: true,
        },
        features: FeatureConfig {
            enable_sync: false,
            enable_real_time: false,
            enable_analytics: true,
            enable_file_storage: false,
            batch_size: 1,
            sync_interval_seconds: 0,
        },
        rate_limiting: RateLimitConfig {
            requests_per_minute: Some(100),
            requests_per_hour: Some(2000),
            requests_per_day: None,
            burst_capacity: Some(10),
        },
        retry_policy: RetryPolicyConfig {
            max_retries: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 10000,
            backoff_multiplier: 2.0,
            retry_on_timeout: true,
        },
        timeout: Duration::from_secs(30),
        custom_settings,
    };
    
    // Initialize the engine
    println!("🔧 Initializing Wolfram Alpha engine...");
    match wolfram_engine.initialize(&integration_config).await {
        Ok(_) => println!("✅ Wolfram Alpha engine initialized successfully"),
        Err(e) => {
            println!("❌ Failed to initialize engine: {}", e);
            println!("💡 Make sure to set a valid Wolfram Alpha App ID");
            return Ok(()); // Continue with demo using mock queries
        }
    }
    
    // Check engine health
    println!("\n🏥 Checking engine health...");
    match wolfram_engine.health_check().await {
        Ok(health) => {
            println!("✅ Engine health: {:?}", health.status);
            if let Some(response_time) = health.response_time_ms {
                println!("⏱️  Response time: {}ms", response_time);
            }
        }
        Err(e) => {
            println!("❌ Health check failed: {}", e);
        }
    }
    
    // Create computational engine manager
    let mut engine_manager = ComputationalEngineManager::new(RoutingStrategy::BestMatch);
    
    // Register the Wolfram Alpha engine
    engine_manager.register_engine(
        "wolfram_alpha".to_string(),
        Box::new(wolfram_engine),
    );
    
    // Set fallback chain (for when we have multiple engines)
    engine_manager.set_fallback_chain(vec!["wolfram_alpha".to_string()]);
    
    println!("\n🔍 Available computational capabilities:");
    let capabilities = engine_manager.get_engine_capabilities();
    for (engine_name, caps) in capabilities {
        println!("  🔹 {}: {:?}", engine_name, caps);
    }
    
    // Example queries to demonstrate different capabilities
    let example_queries = vec![
        // Basic math
        ComputationalQuery::natural_language("integrate x^2 from 0 to 5"),
        
        // Mathematical expression
        ComputationalQuery::mathematical_expression("d/dx(sin(x^2))", MathNotation::Standard),
        
        // Physics calculation
        {
            let mut query = ComputationalQuery::natural_language("kinetic energy of 10kg object at 5 m/s");
            query.capabilities_required = vec![ComputationalCapability::Physics];
            query.priority = QueryPriority::High;
            query
        },
        
        // Unit conversion
        {
            let mut query = ComputationalQuery::natural_language("convert 100 fahrenheit to celsius");
            query.capabilities_required = vec![ComputationalCapability::UnitConversion];
            query.output_format = OutputFormat::JSON;
            query
        },
        
        // Statistics
        {
            let mut query = ComputationalQuery::natural_language("standard deviation of [1,2,3,4,5,6,7,8,9,10]");
            query.capabilities_required = vec![ComputationalCapability::Statistics];
            query.output_format = OutputFormat::LaTeX;
            query
        },
    ];
    
    println!("\n🚀 Running example computational queries:");
    println!("==========================================");
    
    for (i, query) in example_queries.iter().enumerate() {
        println!("\n📊 Query {}: {:?}", i + 1, extract_query_text(query));
        println!("   Required capabilities: {:?}", query.capabilities_required);
        println!("   Output format: {:?}", query.output_format);
        
        // Validate query first
        println!("   🔍 Validating query...");
        // Note: Since we don't have actual engines available, we'll skip execution
        // In a real scenario with valid API keys, you would do:
        // match engine_manager.execute_query(query.clone()).await {
        //     Ok(result) => {
        //         println!("   ✅ Query executed successfully");
        //         println!("   ⏱️  Execution time: {}ms", result.execution_time_ms);
        //         if let Some(output) = &result.result {
        //             println!("   📄 Result: {:?}", output.content);
        //         }
        //     }
        //     Err(e) => {
        //         println!("   ❌ Query failed: {}", e);
        //     }
        // }
        
        println!("   📝 Query would be executed with fallback support");
    }
    
    // Demonstrate engine health monitoring
    println!("\n🏥 Engine Health Monitoring:");
    println!("============================");
    let health_results = engine_manager.get_engines_health().await;
    for (engine_name, health_result) in health_results {
        match health_result {
            Ok(health) => {
                println!("🔹 {}: {:?} ({}ms)", 
                    engine_name, 
                    health.status,
                    health.response_time_ms.unwrap_or(0)
                );
            }
            Err(e) => {
                println!("🔹 {}: Error - {}", engine_name, e);
            }
        }
    }
    
    // Show how to extend with additional engines
    println!("\n🔌 Plugin Architecture Benefits:");
    println!("================================");
    println!("✅ Easy to add new computational engines (SymPy, MATLAB, etc.)");
    println!("✅ Automatic routing and fallback between engines");
    println!("✅ Consistent API across different computational services");
    println!("✅ Built-in rate limiting and error handling");
    println!("✅ Health monitoring and usage statistics");
    println!("✅ Support for multiple input/output formats");
    println!("✅ Query validation and cost estimation");
    
    println!("\n🎯 Next Steps:");
    println!("==============");
    println!("1. Get a Wolfram Alpha App ID from https://developer.wolframalpha.com/");
    println!("2. Add other computational engines (SymPy, OpenAI, etc.)");
    println!("3. Integrate with MoodBridge's legal workflow system");
    println!("4. Add caching for frequently used calculations");
    println!("5. Implement cost tracking and optimization");
    
    Ok(())
}

/// Helper function to extract query text for display
fn extract_query_text(query: &ComputationalQuery) -> String {
    match &query.input {
        moodbridge_rust::integrations::computational::QueryInputFormat::NaturalLanguage(text) => text.clone(),
        moodbridge_rust::integrations::computational::QueryInputFormat::Mathematical { expression, .. } => expression.clone(),
        moodbridge_rust::integrations::computational::QueryInputFormat::Structured { operation, .. } => operation.clone(),
        moodbridge_rust::integrations::computational::QueryInputFormat::Code { code, .. } => code.clone(),
    }
}
