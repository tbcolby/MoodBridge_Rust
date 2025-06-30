//! # SymPy Computational Engine Plugin
//! 
//! This module implements the SymPy integration as a computational engine plugin.
//! SymPy is a Python library for symbolic mathematics that excels at algebraic manipulation,
//! calculus, equation solving, and symbolic computation.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::process::Command;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use tokio::process::Command as AsyncCommand;

use crate::integrations::{
    IntegrationResult, IntegrationError, IntegrationHealth, ConnectionStatus, 
    PlatformIntegration, IntegrationConfig, IntegrationCapability,
    AuthenticationResult,
};

use super::super::computational::{
    ComputationalEngine, ComputationalCapability, ComputationalQuery, ComputationalResult,
    QueryId, QueryStatus, EngineUsageStats, ValidationResult, QueryInputFormat,
    OutputFormat, QueryOutput, QueryCost, VisualizationData, RateLimitStatus, MathNotation,
};

/// SymPy engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymPyConfig {
    pub python_executable: String,
    pub sympy_script_path: String,
    pub timeout_seconds: u64,
    pub enable_latex_output: bool,
    pub enable_plotting: bool,
    pub precision: Option<u32>,
    pub max_expression_length: Option<usize>,
}

impl Default for SymPyConfig {
    fn default() -> Self {
        Self {
            python_executable: "python3".to_string(),
            sympy_script_path: "./scripts/sympy_engine.py".to_string(),
            timeout_seconds: 30,
            enable_latex_output: true,
            enable_plotting: true,
            precision: Some(50),
            max_expression_length: Some(10000),
        }
    }
}

/// SymPy computational engine implementation
pub struct SymPyEngine {
    config: SymPyConfig,
    usage_stats: EngineUsageStats,
    python_available: bool,
    sympy_available: bool,
}

impl SymPyEngine {
    pub fn new(config: SymPyConfig) -> Self {
        Self {
            config,
            usage_stats: EngineUsageStats {
                total_queries: 0,
                successful_queries: 0,
                failed_queries: 0,
                average_execution_time_ms: 0.0,
                total_cost: None,
                rate_limit_status: RateLimitStatus {
                    requests_remaining: None,
                    reset_time: None,
                    daily_limit: None,
                    monthly_limit: None,
                },
                last_query_time: None,
            },
            python_available: false,
            sympy_available: false,
        }
    }

    /// Check if Python and SymPy are available
    async fn check_dependencies(&mut self) -> IntegrationResult<()> {
        // Check Python
        let python_check = Command::new(&self.config.python_executable)
            .arg("--version")
            .output();
            
        self.python_available = python_check.is_ok();
        
        if !self.python_available {
            return Err(IntegrationError::ConfigurationError {
                message: format!("Python executable '{}' not found", self.config.python_executable),
            });
        }

        // Check SymPy
        let sympy_check = Command::new(&self.config.python_executable)
            .arg("-c")
            .arg("import sympy; print(sympy.__version__)")
            .output();
            
        self.sympy_available = sympy_check.is_ok();
        
        if !self.sympy_available {
            return Err(IntegrationError::ConfigurationError {
                message: "SymPy library not found. Install with: pip install sympy".to_string(),
            });
        }

        Ok(())
    }

    /// Create the SymPy script for execution
    fn create_sympy_script(&self, query: &ComputationalQuery) -> String {
        let query_text = match &query.input {
            QueryInputFormat::NaturalLanguage(text) => text.clone(),
            QueryInputFormat::Mathematical { expression, .. } => expression.clone(),
            QueryInputFormat::Structured { operation, parameters } => {
                format!("{} with {}", operation, 
                    parameters.iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect::<Vec<_>>()
                        .join(", "))
            }
            QueryInputFormat::Code { code, .. } => code.clone(),
        };

        let output_latex = matches!(query.output_format, OutputFormat::LaTeX) && self.config.enable_latex_output;
        let precision = self.config.precision.unwrap_or(50);

        format!(r#"
import sympy as sp
import json
import sys
from sympy import *
from sympy.parsing.sympy_parser import parse_expr
from sympy.parsing.latex import parse_latex

# Set precision
sp.init_printing()
if {precision} > 0:
    sp.N._context.prec = {precision}

def safe_eval(expression_str):
    """Safely evaluate mathematical expressions using SymPy"""
    try:
        # Try to parse as SymPy expression
        expr = parse_expr(expression_str, transformations='all')
        return expr
    except:
        try:
            # Try as LaTeX if that fails
            expr = parse_latex(expression_str)
            return expr
        except:
            # Last resort: try direct evaluation in SymPy namespace
            return eval(expression_str, {{"__builtins__": {{}}, **globals()}})

def process_query(query_text):
    """Process the mathematical query and return results"""
    result = {{}}
    
    try:
        # Handle different types of queries
        if any(keyword in query_text.lower() for keyword in ['solve', 'equation']):
            # Equation solving
            if '=' in query_text:
                left, right = query_text.split('=', 1)
                equation = Eq(safe_eval(left.strip()), safe_eval(right.strip()))
                variables = list(equation.free_symbols)
                if variables:
                    solution = solve(equation, variables[0])
                    result['solution'] = str(solution)
                    result['type'] = 'equation_solution'
                else:
                    result['error'] = 'No variables found in equation'
            else:
                expr = safe_eval(query_text)
                result['simplified'] = str(simplify(expr))
                result['type'] = 'simplification'
                
        elif any(keyword in query_text.lower() for keyword in ['integrate', 'integral']):
            # Integration
            expr = safe_eval(query_text.replace('integrate', '').replace('integral', '').strip())
            variables = list(expr.free_symbols)
            if variables:
                integral_result = integrate(expr, variables[0])
                result['integral'] = str(integral_result)
                result['type'] = 'integration'
            else:
                result['error'] = 'No variables found for integration'
                
        elif any(keyword in query_text.lower() for keyword in ['differentiate', 'derivative', 'diff']):
            # Differentiation
            expr = safe_eval(query_text.replace('differentiate', '').replace('derivative', '').replace('diff', '').strip())
            variables = list(expr.free_symbols)
            if variables:
                derivative = diff(expr, variables[0])
                result['derivative'] = str(derivative)
                result['type'] = 'differentiation'
            else:
                result['error'] = 'No variables found for differentiation'
                
        elif any(keyword in query_text.lower() for keyword in ['limit']):
            # Limits - basic implementation
            expr = safe_eval(query_text.replace('limit', '').strip())
            variables = list(expr.free_symbols)
            if variables:
                limit_result = limit(expr, variables[0], 0)  # Default to limit as x->0
                result['limit'] = str(limit_result)
                result['type'] = 'limit'
            else:
                result['error'] = 'No variables found for limit'
                
        elif any(keyword in query_text.lower() for keyword in ['expand', 'factor', 'simplify']):
            # Algebraic manipulation
            expr = safe_eval(query_text)
            if 'expand' in query_text.lower():
                result['expanded'] = str(expand(expr))
                result['type'] = 'expansion'
            elif 'factor' in query_text.lower():
                result['factored'] = str(factor(expr))
                result['type'] = 'factorization'
            else:
                result['simplified'] = str(simplify(expr))
                result['type'] = 'simplification'
                
        else:
            # General expression evaluation
            expr = safe_eval(query_text)
            result['expression'] = str(expr)
            result['simplified'] = str(simplify(expr))
            result['type'] = 'evaluation'
            
            # Try to provide additional insights
            if expr.free_symbols:
                result['variables'] = [str(var) for var in expr.free_symbols]
            
            # Check if it's a number
            if expr.is_number:
                result['numerical_value'] = float(expr.evalf())
        
        # Add LaTeX representation if requested
        if {output_latex}:
            try:
                result['latex'] = latex(safe_eval(query_text))
            except:
                pass
                
        result['success'] = True
        
    except Exception as e:
        result['success'] = False
        result['error'] = str(e)
        
    return result

# Main execution
query_text = r"""{query_text}"""
result = process_query(query_text)
print(json.dumps(result, indent=2))
"#, query_text = query_text, output_latex = output_latex, precision = precision)
    }

    /// Execute SymPy computation
    async fn execute_sympy(&self, script: &str) -> IntegrationResult<serde_json::Value> {
        let mut cmd = AsyncCommand::new(&self.config.python_executable);
        cmd.arg("-c").arg(script);
        
        let output = tokio::time::timeout(
            Duration::from_secs(self.config.timeout_seconds),
            cmd.output()
        ).await
        .map_err(|_| IntegrationError::TimeoutError {
            timeout: Duration::from_secs(self.config.timeout_seconds),
        })?
        .map_err(|e| IntegrationError::InternalError {
            message: format!("Failed to execute Python script: {}", e),
        })?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(IntegrationError::InternalError {
                message: format!("SymPy execution failed: {}", error_msg),
            });
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str(&stdout).map_err(|e| IntegrationError::InternalError {
            message: format!("Failed to parse SymPy output: {}", e),
        })
    }

    /// Convert SymPy result to our result format
    fn convert_sympy_result(&self, sympy_result: serde_json::Value, query_id: QueryId, execution_time: u64) -> ComputationalResult {
        let success = sympy_result.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
        
        if !success {
            let error_msg = sympy_result.get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown SymPy error");
                
            return ComputationalResult {
                query_id,
                engine_name: "sympy".to_string(),
                success: false,
                result: None,
                error: Some(error_msg.to_string()),
                execution_time_ms: execution_time,
                cost: Some(QueryCost {
                    credits_used: Some(0), // SymPy is free
                    monetary_cost: Some(0.0),
                    currency: Some("USD".to_string()),
                    rate_limit_consumed: Some(0),
                }),
                confidence: None,
                alternatives: Vec::new(),
                metadata: HashMap::new(),
            };
        }

        let mut outputs = Vec::new();
        
        // Extract different types of results
        if let Some(solution) = sympy_result.get("solution").and_then(|v| v.as_str()) {
            outputs.push(QueryOutput {
                format: OutputFormat::PlainText,
                content: serde_json::Value::String(solution.to_string()),
                description: Some("Solution".to_string()),
                visualization: None,
                references: Vec::new(),
            });
        }
        
        if let Some(simplified) = sympy_result.get("simplified").and_then(|v| v.as_str()) {
            outputs.push(QueryOutput {
                format: OutputFormat::PlainText,
                content: serde_json::Value::String(simplified.to_string()),
                description: Some("Simplified Expression".to_string()),
                visualization: None,
                references: Vec::new(),
            });
        }
        
        if let Some(integral) = sympy_result.get("integral").and_then(|v| v.as_str()) {
            outputs.push(QueryOutput {
                format: OutputFormat::PlainText,
                content: serde_json::Value::String(integral.to_string()),
                description: Some("Integral".to_string()),
                visualization: None,
                references: Vec::new(),
            });
        }
        
        if let Some(derivative) = sympy_result.get("derivative").and_then(|v| v.as_str()) {
            outputs.push(QueryOutput {
                format: OutputFormat::PlainText,
                content: serde_json::Value::String(derivative.to_string()),
                description: Some("Derivative".to_string()),
                visualization: None,
                references: Vec::new(),
            });
        }
        
        if let Some(latex) = sympy_result.get("latex").and_then(|v| v.as_str()) {
            outputs.push(QueryOutput {
                format: OutputFormat::LaTeX,
                content: serde_json::Value::String(latex.to_string()),
                description: Some("LaTeX Representation".to_string()),
                visualization: None,
                references: Vec::new(),
            });
        }

        let primary_result = outputs.first().cloned();
        let alternatives = if outputs.len() > 1 {
            outputs[1..].to_vec()
        } else {
            Vec::new()
        };

        ComputationalResult {
            query_id,
            engine_name: "sympy".to_string(),
            success: true,
            result: primary_result,
            error: None,
            execution_time_ms: execution_time,
            cost: Some(QueryCost {
                credits_used: Some(0), // SymPy is free
                monetary_cost: Some(0.0),
                currency: Some("USD".to_string()),
                rate_limit_consumed: Some(0),
            }),
            confidence: Some(0.95), // SymPy is very reliable for symbolic math
            alternatives,
            metadata: {
                let mut meta = HashMap::new();
                if let Some(query_type) = sympy_result.get("type").and_then(|v| v.as_str()) {
                    meta.insert("query_type".to_string(), serde_json::Value::String(query_type.to_string()));
                }
                meta
            },
        }
    }

    /// Update usage statistics
    fn update_stats(&mut self, success: bool, execution_time_ms: u64) {
        self.usage_stats.total_queries += 1;
        if success {
            self.usage_stats.successful_queries += 1;
        } else {
            self.usage_stats.failed_queries += 1;
        }
        
        let total_time = self.usage_stats.average_execution_time_ms * (self.usage_stats.total_queries - 1) as f64;
        self.usage_stats.average_execution_time_ms = (total_time + execution_time_ms as f64) / self.usage_stats.total_queries as f64;
        
        self.usage_stats.last_query_time = Some(Utc::now());
    }
}

#[async_trait]
impl PlatformIntegration for SymPyEngine {
    fn platform_name(&self) -> &'static str {
        "sympy"
    }

    fn capabilities(&self) -> Vec<IntegrationCapability> {
        vec![
            IntegrationCapability::Analytics,
        ]
    }

    async fn health_check(&self) -> IntegrationResult<IntegrationHealth> {
        let start = Instant::now();
        
        // Test basic SymPy functionality
        let test_script = r#"
import sympy as sp
import json
result = {"success": True, "version": sp.__version__}
print(json.dumps(result))
"#;

        let status = match AsyncCommand::new(&self.config.python_executable)
            .arg("-c")
            .arg(test_script)
            .output()
            .await
        {
            Ok(output) if output.status.success() => ConnectionStatus::Healthy,
            Ok(_) => ConnectionStatus::Degraded { 
                reason: "SymPy test failed".to_string() 
            },
            Err(e) => ConnectionStatus::Unhealthy { 
                error: e.to_string() 
            },
        };

        let response_time = start.elapsed().as_millis() as u64;

        Ok(IntegrationHealth {
            platform_name: "SymPy".to_string(),
            status,
            last_checked: Utc::now(),
            response_time_ms: Some(response_time),
            capabilities: vec![IntegrationCapability::Analytics],
            rate_limit_remaining: None, // No rate limits for local execution
            rate_limit_reset: None,
            metadata: HashMap::new(),
        })
    }

    async fn initialize(&mut self, _config: &IntegrationConfig) -> IntegrationResult<()> {
        self.check_dependencies().await
    }

    async fn shutdown(&mut self) -> IntegrationResult<()> {
        // Nothing to cleanup for SymPy
        Ok(())
    }

    async fn authenticate(&mut self) -> IntegrationResult<AuthenticationResult> {
        Ok(AuthenticationResult {
            success: self.python_available && self.sympy_available,
            access_token: None,
            refresh_token: None,
            expires_at: None,
            token_type: Some("Local".to_string()),
            scope: None,
            metadata: HashMap::new(),
        })
    }

    async fn refresh_auth(&mut self) -> IntegrationResult<AuthenticationResult> {
        self.authenticate().await
    }
}

#[async_trait]
impl ComputationalEngine for SymPyEngine {
    fn supported_capabilities(&self) -> Vec<ComputationalCapability> {
        vec![
            ComputationalCapability::BasicMath,
            ComputationalCapability::AdvancedMath,
            ComputationalCapability::SymbolicMath,
            ComputationalCapability::NumericalAnalysis,
            ComputationalCapability::Statistics,
        ]
    }

    fn supported_input_formats(&self) -> Vec<QueryInputFormat> {
        vec![
            QueryInputFormat::NaturalLanguage("example".to_string()),
            QueryInputFormat::Mathematical {
                expression: "example".to_string(),
                notation: MathNotation::Standard,
            },
            QueryInputFormat::Mathematical {
                expression: "example".to_string(),
                notation: MathNotation::LaTeX,
            },
            QueryInputFormat::Mathematical {
                expression: "example".to_string(),
                notation: MathNotation::SymPy,
            },
            QueryInputFormat::Code {
                language: "python".to_string(),
                code: "example".to_string(),
            },
        ]
    }

    fn supported_output_formats(&self) -> Vec<OutputFormat> {
        vec![
            OutputFormat::PlainText,
            OutputFormat::LaTeX,
            OutputFormat::JSON,
        ]
    }

    fn can_handle_query(&self, query: &ComputationalQuery) -> bool {
        if !self.python_available || !self.sympy_available {
            return false;
        }

        let supported_caps = self.supported_capabilities();
        
        // Check if we support at least one required capability
        query.capabilities_required.iter().any(|cap| supported_caps.contains(cap)) &&
        
        // Check if input format is supported
        match &query.input {
            QueryInputFormat::NaturalLanguage(_) => true,
            QueryInputFormat::Mathematical { notation, .. } => {
                matches!(notation, MathNotation::Standard | MathNotation::LaTeX | MathNotation::SymPy)
            }
            QueryInputFormat::Code { language, .. } => language == "python",
            QueryInputFormat::Structured { .. } => true,
        }
    }

    async fn execute_query(&self, query: ComputationalQuery) -> IntegrationResult<ComputationalResult> {
        let start_time = Instant::now();
        let query_id = query.query_id.clone();
        
        let script = self.create_sympy_script(&query);
        let sympy_result = self.execute_sympy(&script).await?;
        
        let execution_time = start_time.elapsed().as_millis() as u64;
        Ok(self.convert_sympy_result(sympy_result, query_id, execution_time))
    }

    async fn get_query_status(&self, _query_id: &QueryId) -> IntegrationResult<QueryStatus> {
        // SymPy executes synchronously
        Ok(QueryStatus::Completed)
    }

    async fn cancel_query(&self, _query_id: &QueryId) -> IntegrationResult<()> {
        // SymPy doesn't support query cancellation
        Err(IntegrationError::FeatureNotSupported {
            feature: "Query cancellation".to_string(),
        })
    }

    async fn get_usage_stats(&self) -> IntegrationResult<EngineUsageStats> {
        Ok(self.usage_stats.clone())
    }

    async fn validate_query(&self, query: &ComputationalQuery) -> IntegrationResult<ValidationResult> {
        let mut warnings = Vec::new();
        let mut suggestions = Vec::new();
        
        // Check query complexity
        let query_text = match &query.input {
            QueryInputFormat::NaturalLanguage(text) => text,
            QueryInputFormat::Mathematical { expression, .. } => expression,
            _ => "",
        };
        
        if let Some(max_len) = self.config.max_expression_length {
            if query_text.len() > max_len {
                warnings.push(format!("Query length ({}) exceeds maximum ({})", query_text.len(), max_len));
            }
        }
        
        // Check for potentially slow operations
        if query_text.contains("solve") && query_text.len() > 100 {
            warnings.push("Complex equation solving may take longer to execute".to_string());
        }
        
        if query_text.contains("integrate") && query_text.contains("**") {
            suggestions.push("High-power integrations may require symbolic simplification".to_string());
        }

        Ok(ValidationResult {
            is_valid: self.can_handle_query(query),
            estimated_cost: Some(QueryCost {
                credits_used: Some(0),
                monetary_cost: Some(0.0),
                currency: Some("USD".to_string()),
                rate_limit_consumed: Some(0),
            }),
            estimated_execution_time: Some(Duration::from_secs(5)),
            warnings,
            suggestions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::integrations::computational::*;

    #[test]
    fn test_sympy_engine_creation() {
        let config = SymPyConfig::default();
        let engine = SymPyEngine::new(config);
        assert_eq!(engine.platform_name(), "sympy");
    }

    #[test]
    fn test_supported_capabilities() {
        let config = SymPyConfig::default();
        let engine = SymPyEngine::new(config);
        let capabilities = engine.supported_capabilities();
        
        assert!(capabilities.contains(&ComputationalCapability::SymbolicMath));
        assert!(capabilities.contains(&ComputationalCapability::AdvancedMath));
        assert!(capabilities.contains(&ComputationalCapability::BasicMath));
    }

    #[test]
    fn test_script_generation() {
        let config = SymPyConfig::default();
        let engine = SymPyEngine::new(config);
        
        let query = ComputationalQuery::mathematical_expression("x^2 + 2*x + 1", MathNotation::Standard);
        let script = engine.create_sympy_script(&query);
        
        assert!(script.contains("import sympy"));
        assert!(script.contains("x^2 + 2*x + 1"));
    }
}
