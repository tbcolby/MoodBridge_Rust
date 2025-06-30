# 🧮 Computational Engine Integration Framework

This document describes the plugin architecture for integrating computational engines like Wolfram Alpha, SymPy, MATLAB, Mathematica, and other mathematical/scientific computation services into MoodBridge_Rust.

## 🎯 Overview

The computational engine framework provides a unified, extensible plugin architecture that allows MoodBridge to leverage multiple computational services through a consistent API. This is particularly valuable for legal technology where complex calculations, statistical analysis, and data modeling are frequently required.

## 🏗️ Architecture

### Core Components

1. **`ComputationalEngine` Trait** - Defines the interface all computational engines must implement
2. **`ComputationalEngineManager`** - Manages multiple engines with routing and fallback strategies
3. **Query System** - Flexible input/output system supporting multiple formats and capabilities
4. **Plugin System** - Easy registration and configuration of new computational engines

### Key Features

- **🔌 Plugin Architecture**: Easy to add new computational engines
- **🔄 Automatic Routing**: Intelligent routing based on capabilities and performance
- **🛡️ Fallback Support**: Automatic failover between engines
- **📊 Health Monitoring**: Real-time health checks and performance metrics
- **💰 Cost Tracking**: Built-in cost estimation and usage tracking
- **🚦 Rate Limiting**: Configurable rate limiting per engine
- **🔧 Flexible I/O**: Support for multiple input/output formats
- **⚡ Async Operations**: Full async/await support for non-blocking operations

## 🚀 Getting Started

### 1. Basic Setup

```rust
use moodbridge_rust::integrations::computational::{
    ComputationalEngineManager, ComputationalQuery, RoutingStrategy
};
use moodbridge_rust::integrations::engines::{WolframAlphaEngine, WolframAlphaConfig};

// Create engine manager
let mut manager = ComputationalEngineManager::new(RoutingStrategy::BestMatch);

// Configure and register Wolfram Alpha engine
let wolfram_config = WolframAlphaConfig {
    app_id: "YOUR_APP_ID".to_string(),
    base_url: "https://api.wolframalpha.com/v2/query".to_string(),
    timeout_seconds: 30,
    rate_limit_per_hour: Some(2000),
    enable_step_by_step: true,
    enable_plots: true,
    preferred_units: Some("metric".to_string()),
};

let wolfram_engine = WolframAlphaEngine::new(wolfram_config);
manager.register_engine("wolfram_alpha".to_string(), Box::new(wolfram_engine));
```

### 2. Making Queries

```rust
// Natural language query
let query = ComputationalQuery::natural_language("integrate x^2 from 0 to 5");

// Mathematical expression query
let math_query = ComputationalQuery::mathematical_expression(
    "d/dx(sin(x^2))", 
    MathNotation::Standard
);

// Execute query with automatic engine selection
let result = manager.execute_query(query).await?;
```

### 3. Advanced Usage

```rust
// Custom query with specific requirements
let mut physics_query = ComputationalQuery::natural_language(
    "kinetic energy of 10kg object at 5 m/s"
);
physics_query.capabilities_required = vec![ComputationalCapability::Physics];
physics_query.priority = QueryPriority::High;
physics_query.output_format = OutputFormat::LaTeX;

// Execute with fallback support
let result = manager.execute_query_with_fallback(physics_query).await?;
```

## 🔧 Supported Engines

### Currently Implemented

#### Wolfram Alpha Engine
- **Capabilities**: Math, Physics, Chemistry, Statistics, Unit Conversion, Natural Language Queries
- **Input Formats**: Natural Language, Mathematical Expressions, Wolfram Language
- **Output Formats**: Plain Text, LaTeX, Images, HTML
- **Setup**: Requires Wolfram Alpha App ID from [developer.wolframalpha.com](https://developer.wolframalpha.com/)

### Planned Engines

#### SymPy Engine (Python Integration)
- **Capabilities**: Symbolic Mathematics, Calculus, Algebra
- **Benefits**: Free, open-source, excellent for symbolic computation

#### OpenAI/ChatGPT Engine
- **Capabilities**: Natural Language Math, Code Generation, Explanations
- **Benefits**: Excellent natural language understanding

#### MATLAB Engine
- **Capabilities**: Numerical Analysis, Signal Processing, Engineering
- **Benefits**: Industry standard for engineering calculations

#### Mathematica Engine
- **Capabilities**: Advanced Mathematics, Symbolic Computation
- **Benefits**: Comprehensive mathematical system

## 📊 Computational Capabilities

The framework supports various computational capabilities:

- **BasicMath**: Arithmetic, algebra, basic functions
- **AdvancedMath**: Calculus, linear algebra, differential equations
- **Statistics**: Statistical analysis, probability distributions
- **Physics**: Physics calculations and simulations
- **Chemistry**: Chemical calculations and molecular analysis
- **Engineering**: Engineering-specific calculations
- **FinancialMath**: Financial modeling and calculations
- **UnitConversion**: Converting between different units
- **DataAnalysis**: Data processing and visualization
- **MachineLearning**: ML algorithms and computations

## 🎛️ Configuration

### Engine Configuration

```rust
let wolfram_config = WolframAlphaConfig {
    app_id: "YOUR_APP_ID".to_string(),
    base_url: "https://api.wolframalpha.com/v2/query".to_string(),
    timeout_seconds: 30,
    rate_limit_per_hour: Some(2000),
    enable_step_by_step: true,
    enable_plots: true,
    preferred_units: Some("metric".to_string()),
};
```

### Routing Strategies

- **FirstCapable**: Route to first engine that can handle the query
- **BestMatch**: Route to engine with best capability match
- **Fastest**: Route to fastest engine for the query type
- **CostEffective**: Route to most cost-effective engine
- **Custom**: Implement custom routing logic

### Query Types

```rust
// Natural language
let query1 = ComputationalQuery::natural_language("solve x^2 + 2x - 8 = 0");

// Mathematical expression
let query2 = ComputationalQuery::mathematical_expression(
    "∫x²dx", 
    MathNotation::LaTeX
);

// Structured query
let mut params = HashMap::new();
params.insert("equation".to_string(), json!("x^2 + 2x - 8 = 0"));
let query3 = ComputationalQuery::structured("solve", params);
```

## 📈 Legal Technology Applications

### Financial Calculations
- Damage calculations in litigation
- Present value computations
- Statistical analysis of financial data
- Risk assessment modeling

### Statistical Analysis
- Evidence analysis and probability calculations
- Survey data analysis
- Trend analysis for case outcomes
- Comparative analysis

### Document Analysis
- Text analytics and natural language processing
- Pattern recognition in legal documents
- Sentiment analysis for jury selection
- Topic modeling for case categorization

### Compliance Calculations
- Regulatory compliance computations
- Tax calculations and modeling
- Environmental impact assessments
- Safety factor calculations

## 🔒 Security Considerations

### API Key Management
- Store API keys securely using environment variables
- Implement key rotation policies
- Monitor API usage for unusual patterns
- Use separate keys for development/production

### Data Privacy
- Ensure computational engines comply with data privacy regulations
- Consider data residency requirements
- Implement audit logging for sensitive calculations
- Use data anonymization where possible

### Rate Limiting
- Implement proper rate limiting to avoid service disruption
- Monitor usage patterns and adjust limits accordingly
- Implement graceful degradation when limits are reached
- Cache frequently used calculations

## 🚀 Usage Examples

### Running the Demo

```bash
# Run the computational engine demo
cargo run --example computational_engine_demo

# Set your Wolfram Alpha App ID (optional)
export WOLFRAM_ALPHA_APP_ID="your_app_id_here"
cargo run --example computational_engine_demo
```

### Integration with MoodBridge

```rust
// In your MoodBridge application
use moodbridge_rust::integrations::computational::*;

// Initialize computational engines during app startup
let mut engine_manager = ComputationalEngineManager::new(RoutingStrategy::BestMatch);

// Register engines based on configuration
if config.wolfram_alpha.enabled {
    let wolfram_engine = WolframAlphaEngine::new(config.wolfram_alpha);
    engine_manager.register_engine("wolfram_alpha".to_string(), Box::new(wolfram_engine));
}

// Use in your handlers
async fn calculate_damages(
    engine_manager: &ComputationalEngineManager,
    calculation_request: DamageCalculationRequest
) -> Result<CalculationResult, Error> {
    let query = ComputationalQuery::natural_language(&format!(
        "calculate present value of {} over {} years at {}% interest",
        calculation_request.amount,
        calculation_request.years,
        calculation_request.interest_rate
    ));
    
    let result = engine_manager.execute_query(query).await?;
    // Process result...
}
```

## 🔧 Development

### Adding a New Engine

1. **Create Engine Implementation**:
```rust
pub struct MyCustomEngine {
    config: MyCustomConfig,
    // ... other fields
}

#[async_trait]
impl ComputationalEngine for MyCustomEngine {
    fn supported_capabilities(&self) -> Vec<ComputationalCapability> {
        vec![ComputationalCapability::BasicMath, /* ... */]
    }
    
    async fn execute_query(&self, query: ComputationalQuery) -> IntegrationResult<ComputationalResult> {
        // Implementation...
    }
    
    // ... implement other required methods
}
```

2. **Register in Engine Manager**:
```rust
let engine = MyCustomEngine::new(config);
engine_manager.register_engine("my_engine".to_string(), Box::new(engine));
```

3. **Add to Fallback Chain**:
```rust
engine_manager.set_fallback_chain(vec![
    "wolfram_alpha".to_string(),
    "my_engine".to_string(),
]);
```

### Testing

```bash
# Run all tests
cargo test

# Run computational engine tests specifically
cargo test integrations::computational

# Run Wolfram Alpha engine tests
cargo test integrations::engines::wolfram_alpha
```

## 📚 API Reference

### Core Traits

- **`ComputationalEngine`**: Main trait for computational engine implementations
- **`PlatformIntegration`**: Base integration trait with health checking and authentication

### Key Types

- **`ComputationalQuery`**: Query request with input, capabilities, and output preferences
- **`ComputationalResult`**: Query result with output, metadata, and performance metrics
- **`ComputationalEngineManager`**: Manager for coordinating multiple engines
- **`QueryId`**: Unique identifier for tracking queries
- **`ComputationalCapability`**: Enum of supported computational capabilities

### Configuration Types

- **`WolframAlphaConfig`**: Configuration for Wolfram Alpha engine
- **`RoutingStrategy`**: Strategy for selecting engines
- **`OutputFormat`**: Supported output formats (PlainText, LaTeX, JSON, etc.)

## 🤝 Contributing

We welcome contributions to expand the computational engine framework:

1. **New Engine Implementations**: Add support for additional computational services
2. **Enhanced Routing**: Improve engine selection algorithms
3. **Caching System**: Implement intelligent caching for frequently used calculations
4. **Performance Optimization**: Optimize query processing and response handling
5. **Documentation**: Improve documentation and examples

## 📄 License

This computational engine framework is part of MoodBridge_Rust and follows the same licensing terms.

## 🔗 Related Resources

- [Wolfram Alpha API Documentation](https://products.wolframalpha.com/api/)
- [SymPy Documentation](https://docs.sympy.org/)
- [MoodBridge_Rust Main Documentation](./README.md)
- [Integration Framework Documentation](./src/integrations/README.md)

---

For questions, issues, or contributions, please refer to the main MoodBridge_Rust repository.
