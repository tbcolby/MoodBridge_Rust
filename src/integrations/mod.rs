// Computational Engine Integrations Module
// Provides plugin system for various computational engines

pub mod engines;

use serde::{Deserialize, Serialize};

// Re-export engine modules
pub use engines::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    pub name: String,
    pub enabled: bool,
    pub api_key: Option<String>,
    pub rate_limit: u32,
    pub timeout_seconds: u32,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            enabled: true,
            api_key: None,
            rate_limit: 100,
            timeout_seconds: 30,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineResult {
    pub engine: String,
    pub result: serde_json::Value,
    pub execution_time_ms: u64,
    pub success: bool,
    pub error: Option<String>,
}

// Plugin registry for dynamic engine loading
pub struct EngineRegistry {
    engines: std::collections::HashMap<String, Box<dyn Engine>>,
}

pub trait Engine: Send + Sync {
    fn name(&self) -> &str;
    fn execute(&self, query: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
    fn is_available(&self) -> bool;
}

impl EngineRegistry {
    pub fn new() -> Self {
        Self {
            engines: std::collections::HashMap::new(),
        }
    }

    pub fn register_engine(&mut self, engine: Box<dyn Engine>) {
        self.engines.insert(engine.name().to_string(), engine);
    }

    pub fn execute(&self, engine_name: &str, query: &str) -> Result<EngineResult, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();
        
        match self.engines.get(engine_name) {
            Some(engine) => {
                match engine.execute(query) {
                    Ok(result) => Ok(EngineResult {
                        engine: engine_name.to_string(),
                        result,
                        execution_time_ms: start.elapsed().as_millis() as u64,
                        success: true,
                        error: None,
                    }),
                    Err(e) => Ok(EngineResult {
                        engine: engine_name.to_string(),
                        result: serde_json::Value::Null,
                        execution_time_ms: start.elapsed().as_millis() as u64,
                        success: false,
                        error: Some(e.to_string()),
                    }),
                }
            }
            None => Err(format!("Engine {} not found", engine_name).into()),
        }
    }

    pub fn list_engines(&self) -> Vec<String> {
        self.engines.keys().cloned().collect()
    }
}
