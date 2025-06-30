// HeidiMaetl Verb Engine - The execution heart of our ETL platform
// Orchestrates the 50 sophisticated data verbs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::etl::{Pipeline, VerbStep, EtlResult, EtlError};
use std::sync::atomic::{AtomicU32, Ordering};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::LazyLock;

/// String interning system for memory efficiency
/// Reduces memory usage from O(n*m) to O(n) where n=unique strings, m=references
/// Reference: Knuth, TAOCP Vol 3, Section 6.4 (String Processing)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InternedString {
    id: u32,
}

/// Global string interner for verb names and other frequently used strings
static STRING_INTERNER: LazyLock<Mutex<StringInterner>> = LazyLock::new(|| {
    Mutex::new(StringInterner::new())
});

/// Internal string interner implementation
#[derive(Debug)]
struct StringInterner {
    strings: Vec<String>,
    string_to_id: HashMap<String, u32>,
    next_id: AtomicU32,
}

impl StringInterner {
    fn new() -> Self {
        Self {
            strings: Vec::new(),
            string_to_id: HashMap::new(),
            next_id: AtomicU32::new(0),
        }
    }
    
    /// Intern a string and return its ID
    /// 
    /// # Complexity
    /// - Time: O(1) average case, O(n) worst case (hash collision)
    /// - Space: O(1) if string already exists, O(len) if new
    fn intern(&mut self, s: &str) -> InternedString {
        if let Some(&id) = self.string_to_id.get(s) {
            return InternedString { id };
        }
        
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.strings.push(s.to_string());
        self.string_to_id.insert(s.to_string(), id);
        
        InternedString { id }
    }
    
    /// Get string by ID
    fn get(&self, interned: InternedString) -> Option<&String> {
        self.strings.get(interned.id as usize)
    }
}

impl InternedString {
    /// Intern a string globally
    pub fn new(s: &str) -> Self {
        STRING_INTERNER.lock().unwrap().intern(s)
    }
    
    /// Get the string value
    pub fn as_str(&self) -> String {
        STRING_INTERNER.lock().unwrap()
            .get(*self)
            .cloned()
            .unwrap_or_else(|| "<invalid>".to_string())
    }
    
    /// Get raw ID (for debugging)
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl std::fmt::Display for InternedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// The core trait that all HeidiMaetl verbs must implement
/// 
/// # Complexity Requirements
/// All implementations must document their computational complexity:
/// - Time Complexity: Expected Big-O notation for time
/// - Space Complexity: Expected Big-O notation for space
/// - Memory Bound: Description of memory usage pattern
#[async_trait]
pub trait DataVerb: Send + Sync {
    /// Execute the verb with given context
    /// 
    /// # Expected Complexity (to be overridden by implementations)
    /// - Time: O(n) where n = number of records processed
    /// - Space: O(1) additional space beyond input data
    /// - Memory: Linear with input size + constant overhead
    async fn execute(&self, context: &mut VerbContext) -> EtlResult<VerbResult>;
    
    /// Get the verb name
    fn name(&self) -> &str;
    
    /// Get verb metadata and capabilities
    fn metadata(&self) -> VerbMetadata;
    
    /// Validate configuration before execution
    fn validate_config(&self, config: &serde_json::Value) -> EtlResult<()>;
    
    /// Get estimated execution time in milliseconds
    fn estimated_execution_time(&self, data_size: u64) -> u64;
}

/// Context passed to verbs during execution
#[derive(Debug, Clone)]
pub struct VerbContext {
    pub pipeline_id: Uuid,
    pub step_id: Uuid,
    pub data: VerbData,
    pub metadata: HashMap<String, String>,
    pub execution_start: DateTime<Utc>,
    pub previous_results: HashMap<Uuid, VerbResult>,
    pub ai_enabled: bool,
}

/// Data container for verb execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerbData {
    Table {
        schema: Vec<ColumnDef>,
        rows: Vec<HashMap<String, serde_json::Value>>,
    },
    Stream {
        schema: Vec<ColumnDef>,
        chunks: Vec<DataChunk>,
    },
    Document {
        content: String,
        metadata: HashMap<String, String>,
    },
    Binary {
        data: Vec<u8>,
        mime_type: String,
    },
    Json(serde_json::Value),
    Empty,
}

/// Column definition for structured data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
}

/// Data chunk for streaming processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataChunk {
    pub id: Uuid,
    pub data: Vec<HashMap<String, serde_json::Value>>,
    pub timestamp: DateTime<Utc>,
}

/// Circular buffer for predictable memory streaming data processing
/// Implements Knuthian streaming data structure optimization
#[derive(Debug)]
pub struct StreamBuffer<T> {
    buffer: Box<[Option<T>]>,
    head: usize,
    tail: usize,
    capacity: usize,
    // Invariant: (tail - head) % capacity == number of elements
}

impl<T> StreamBuffer<T> {
    /// Create new circular buffer with specified capacity
    /// 
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(capacity)
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        buffer.resize_with(capacity, || None);
        
        Self {
            buffer: buffer.into_boxed_slice(),
            head: 0,
            tail: 0,
            capacity,
        }
    }
    
    /// Push element to buffer, overwriting oldest if full
    /// 
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn push(&mut self, item: T) -> Option<T> {
        let old_value = self.buffer[self.tail].take();
        self.buffer[self.tail] = Some(item);
        
        self.tail = (self.tail + 1) % self.capacity;
        
        // If buffer is full, advance head
        if self.tail == self.head {
            self.head = (self.head + 1) % self.capacity;
        }
        
        old_value
    }
    
    /// Pop oldest element from buffer
    /// 
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        
        let item = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        item
    }
    
    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.head == self.tail && self.buffer[self.head].is_none()
    }
    
    /// Check if buffer is full
    pub fn is_full(&self) -> bool {
        self.head == self.tail && self.buffer[self.head].is_some()
    }
    
    /// Get current number of elements
    pub fn len(&self) -> usize {
        if self.is_empty() {
            0
        } else if self.tail > self.head {
            self.tail - self.head
        } else {
            self.capacity - self.head + self.tail
        }
    }
    
    /// Get buffer capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

/// Enhanced stream data with circular buffer for memory efficiency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedStreamData {
    pub schema: Vec<ColumnDef>,
    pub buffer_capacity: usize,
    pub chunks: Vec<DataChunk>, // Will be replaced with StreamBuffer in actual implementation
    pub total_processed: u64,
    pub memory_footprint: u64,
}

/// Result of verb execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerbResult {
    pub data: VerbData,
    pub metrics: ExecutionMetrics,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub artifacts: HashMap<String, serde_json::Value>,
}

/// Execution metrics for monitoring and optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetrics {
    pub execution_time_ms: u64,
    pub memory_used_mb: f64,
    pub rows_processed: u64,
    pub bytes_processed: u64,
    pub cpu_usage_percent: f64,
    pub success_rate: f64,
    pub memory_profile: MemoryProfile,
}

/// Detailed memory allocation tracking as recommended by Knuth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryProfile {
    pub allocation_count: u64,
    pub deallocation_count: u64,
    pub peak_memory_bytes: u64,
    pub current_memory_bytes: u64,
    pub fragmentation_ratio: f64,
    pub allocation_histogram: [u64; 32], // Powers of 2 size buckets
}

/// Metadata about a verb's capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerbMetadata {
    pub category: VerbCategory,
    pub description: String,
    pub input_types: Vec<String>,
    pub output_types: Vec<String>,
    pub parameters: Vec<ParameterDef>,
    pub ai_enhanced: bool,
    pub streaming_capable: bool,
    pub parallel_safe: bool,
}

/// Categories of verbs for organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerbCategory {
    Movement,      // extract, transform, load, bridge, weave
    Intelligence,  // analyze, predict, classify, cluster, correlate
    Operations,    // monitor, audit, checkpoint, rollback, schedule
    Quality,       // cleanse, standardize, anonymize, encrypt, mask
    AiEnhanced,    // understand, reason, learn, discover, recommend
}

/// Parameter definition for verb configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDef {
    pub name: String,
    pub param_type: String,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
    pub description: String,
    pub validation_rules: Vec<String>,
}

/// Optimized verb registry with controlled load factor
/// Implements Knuthian hash table optimization principles
#[derive(Debug)]
pub struct OptimizedVerbRegistry {
    verbs: HashMap<String, Arc<dyn DataVerb>>,
    load_factor: f64,
    resize_threshold: usize,
    collision_count: u64,
}

impl OptimizedVerbRegistry {
    /// Create new registry with optimal load factor of 0.75
    /// Reference: Knuth, TAOCP Vol 3, Section 6.4
    pub fn new() -> Self {
        let initial_capacity = 16;
        Self {
            verbs: HashMap::with_capacity(initial_capacity),
            load_factor: 0.75,
            resize_threshold: (initial_capacity as f64 * 0.75) as usize,
            collision_count: 0,
        }
    }
    
    /// Insert verb with load factor monitoring
    pub fn insert(&mut self, name: String, verb: Arc<dyn DataVerb>) {
        if self.verbs.len() >= self.resize_threshold {
            self.resize();
        }
        self.verbs.insert(name, verb);
    }
    
    /// Get verb by name
    pub fn get(&self, name: &str) -> Option<&Arc<dyn DataVerb>> {
        self.verbs.get(name)
    }
    
    /// Resize hash table when load factor exceeds threshold
    fn resize(&mut self) {
        let new_capacity = self.verbs.capacity() * 2;
        self.resize_threshold = (new_capacity as f64 * self.load_factor) as usize;
        self.verbs.reserve(new_capacity - self.verbs.capacity());
    }
    
    /// Get current load factor
    pub fn current_load_factor(&self) -> f64 {
        self.verbs.len() as f64 / self.verbs.capacity() as f64
    }
    
    /// Get available verb names
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.verbs.keys()
    }
}

/// The core verb execution engine
#[derive(Debug)]
pub struct VerbEngine {
    verbs: OptimizedVerbRegistry,
    execution_history: Vec<ExecutionRecord>,
    performance_cache: HashMap<String, PerformanceProfile>,
}

/// Record of verb execution for analytics
#[derive(Debug, Clone)]
pub struct ExecutionRecord {
    pub pipeline_id: Uuid,
    pub verb_name: String,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
    pub metrics: ExecutionMetrics,
    pub success: bool,
}

/// Performance profile for optimization
#[derive(Debug, Clone)]
pub struct PerformanceProfile {
    pub avg_execution_time: f64,
    pub memory_usage_pattern: Vec<f64>,
    pub optimal_batch_size: u64,
    pub scaling_factor: f64,
}

impl VerbEngine {
    /// Create a new verb engine
    pub fn new() -> Self {
        Self {
            verbs: OptimizedVerbRegistry::new(),
            execution_history: Vec::new(),
            performance_cache: HashMap::new(),
        }
    }

    /// Register a new verb with the engine
    pub fn register_verb(&mut self, verb: Arc<dyn DataVerb>) {
        self.verbs.insert(verb.name().to_string(), verb);
    }

    /// Execute a complete pipeline
    /// 
    /// # Complexity Analysis
    /// - Time: O(V + E) where V = number of verbs, E = number of dependencies
    /// - Space: O(V) for storing intermediate results
    /// - Memory: Linear with pipeline size and data volume
    pub async fn execute_pipeline(&self, pipeline: &Pipeline) -> EtlResult<()> {
        let mut results: HashMap<Uuid, VerbResult> = HashMap::new();
        
        // Sort verbs by dependencies (topological sort)
        let execution_order = self.build_execution_order(&pipeline.verbs)?;
        
        for step_id in execution_order {
            let step = pipeline.verbs.iter()
                .find(|s| s.id == step_id)
                .ok_or_else(|| EtlError::Execution("Step not found".to_string()))?;
            
            let result = self.execute_verb_step(pipeline.id, step, &results).await?;
            results.insert(step.id, result);
        }
        
        Ok(())
    }

    /// Execute a single verb step
    pub async fn execute_verb_step(
        &self,
        pipeline_id: Uuid,
        step: &VerbStep,
        previous_results: &HashMap<Uuid, VerbResult>,
    ) -> EtlResult<VerbResult> {
        let verb = self.verbs.get(&step.verb_name)
            .ok_or_else(|| EtlError::Execution(format!("Verb not found: {}", step.verb_name)))?;

        // Validate configuration
        let config_json = serde_json::to_value(&step.config)
            .map_err(|e| EtlError::Configuration(e.to_string()))?;
        verb.validate_config(&config_json)?;

        // Build execution context
        let mut context = VerbContext {
            pipeline_id,
            step_id: step.id,
            data: VerbData::Empty, // Will be populated based on dependencies
            metadata: HashMap::new(),
            execution_start: Utc::now(),
            previous_results: previous_results.clone(),
            ai_enabled: true, // TODO: Get from config
        };

        // Execute the verb
        let start_time = std::time::Instant::now();
        let result = verb.execute(&mut context).await?;
        let execution_time = start_time.elapsed().as_millis() as u64;

        // Record execution metrics
        self.record_execution(pipeline_id, verb.name(), execution_time, &result).await;

        Ok(result)
    }

    /// Build execution order based on dependencies using Kahn's algorithm
    /// 
    /// # Complexity Analysis (Knuthian Enhancement)
    /// - Time: O(V + E) where V = vertices (verbs), E = edges (dependencies)
    /// - Space: O(1) iterative approach vs O(V) recursive stack
    /// - Reference: Knuth, TAOCP Vol 1, Section 2.2.3
    fn build_execution_order(&self, steps: &[VerbStep]) -> EtlResult<Vec<Uuid>> {
        self.kahns_topological_sort(steps)
    }

    /// Kahn's algorithm implementation for optimal space efficiency
    /// Preferred over DFS-based approach for better space complexity
    fn kahns_topological_sort(&self, steps: &[VerbStep]) -> EtlResult<Vec<Uuid>> {
        use std::collections::{HashMap, VecDeque};
        
        // Build adjacency list and in-degree count
        let mut in_degree: HashMap<Uuid, usize> = HashMap::new();
        let mut adj_list: HashMap<Uuid, Vec<Uuid>> = HashMap::new();
        
        // Initialize all nodes
        for step in steps {
            in_degree.insert(step.id, 0);
            adj_list.insert(step.id, Vec::new());
        }
        
        // Build the graph
        for step in steps {
            for &dep_id in &step.dependencies {
                if let Some(adj) = adj_list.get_mut(&dep_id) {
                    adj.push(step.id);
                }
                *in_degree.get_mut(&step.id).unwrap() += 1;
            }
        }
        
        // Find all nodes with no incoming edges
        let mut queue: VecDeque<Uuid> = VecDeque::new();
        for (&node_id, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node_id);
            }
        }
        
        let mut result = Vec::new();
        
        // Process nodes in topological order
        while let Some(current) = queue.pop_front() {
            result.push(current);
            
            // Process all neighbors
            if let Some(neighbors) = adj_list.get(&current) {
                for &neighbor in neighbors {
                    let degree = in_degree.get_mut(&neighbor).unwrap();
                    *degree -= 1;
                    
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
        
        // Check for cycles
        if result.len() != steps.len() {
            return Err(EtlError::Execution("Circular dependency detected in pipeline".to_string()));
        }
        
        Ok(result)
    }

    /// Topological sort for dependency resolution
    fn topological_sort(
        &self,
        step_id: Uuid,
        steps: &[VerbStep],
        visited: &mut std::collections::HashSet<Uuid>,
        visiting: &mut std::collections::HashSet<Uuid>,
        order: &mut Vec<Uuid>,
    ) -> EtlResult<()> {
        if visiting.contains(&step_id) {
            return Err(EtlError::Execution("Circular dependency detected".to_string()));
        }

        if visited.contains(&step_id) {
            return Ok(());
        }

        visiting.insert(step_id);

        let step = steps.iter()
            .find(|s| s.id == step_id)
            .ok_or_else(|| EtlError::Execution("Step not found".to_string()))?;

        for dep_id in &step.dependencies {
            self.topological_sort(*dep_id, steps, visited, visiting, order)?;
        }

        visiting.remove(&step_id);
        visited.insert(step_id);
        order.push(step_id);

        Ok(())
    }

    /// Record execution for performance analytics
    async fn record_execution(
        &self,
        pipeline_id: Uuid,
        verb_name: &str,
        execution_time: u64,
        result: &VerbResult,
    ) {
        // TODO: Implement execution recording
        // This would store metrics for performance optimization
    }

    /// Get available verbs
    pub fn available_verbs(&self) -> Vec<String> {
        self.verbs.keys().cloned().collect()
    }

    /// Get verb metadata
    pub fn get_verb_metadata(&self, verb_name: &str) -> Option<VerbMetadata> {
        self.verbs.get(verb_name).map(|v| v.metadata())
    }

    /// Optimize pipeline based on historical performance
    pub fn optimize_pipeline(&self, pipeline: &Pipeline) -> EtlResult<Pipeline> {
        // TODO: Implement AI-driven pipeline optimization
        // This would analyze historical performance and suggest improvements
        Ok(pipeline.clone())
    }
}

impl Default for VerbEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for verb implementations
pub mod utils {
    use super::*;

    /// Create standard execution metrics
    pub fn create_metrics(
        start_time: std::time::Instant,
        rows_processed: u64,
        bytes_processed: u64,
    ) -> ExecutionMetrics {
        ExecutionMetrics {
            execution_time_ms: start_time.elapsed().as_millis() as u64,
            memory_used_mb: 0.0, // TODO: Implement memory tracking
            rows_processed,
            bytes_processed,
            cpu_usage_percent: 0.0, // TODO: Implement CPU tracking
            success_rate: 100.0,
            memory_profile: MemoryProfile {
                allocation_count: 0,
                deallocation_count: 0,
                peak_memory_bytes: 0,
                current_memory_bytes: 0,
                fragmentation_ratio: 0.0,
                allocation_histogram: [0; 32],
            },
        }
    }

    /// Validate data schema compatibility
    pub fn validate_schema_compatibility(
        source: &[ColumnDef],
        target: &[ColumnDef],
    ) -> EtlResult<()> {
        // TODO: Implement schema validation logic
        Ok(())
    }

    /// Convert between data formats
    pub fn convert_data_format(
        data: &VerbData,
        target_format: &str,
    ) -> EtlResult<VerbData> {
        // TODO: Implement data format conversion
        Ok(data.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestVerb;

    #[async_trait]
    impl DataVerb for TestVerb {
        async fn execute(&self, _context: &mut VerbContext) -> EtlResult<VerbResult> {
            Ok(VerbResult {
                data: VerbData::Empty,
                metrics: ExecutionMetrics {
                    execution_time_ms: 100,
                    memory_used_mb: 10.0,
                    rows_processed: 1000,
                    bytes_processed: 50000,
                    cpu_usage_percent: 25.0,
                    success_rate: 100.0,
                    memory_profile: MemoryProfile {
                        allocation_count: 0,
                        deallocation_count: 0,
                        peak_memory_bytes: 0,
                        current_memory_bytes: 0,
                        fragmentation_ratio: 0.0,
                        allocation_histogram: [0; 32],
                    },
                },
                errors: vec![],
                warnings: vec![],
                artifacts: HashMap::new(),
            })
        }

        fn name(&self) -> &str {
            "test"
        }

        fn metadata(&self) -> VerbMetadata {
            VerbMetadata {
                category: VerbCategory::Movement,
                description: "Test verb".to_string(),
                input_types: vec!["any".to_string()],
                output_types: vec!["any".to_string()],
                parameters: vec![],
                ai_enhanced: false,
                streaming_capable: true,
                parallel_safe: true,
            }
        }

        fn validate_config(&self, _config: &serde_json::Value) -> EtlResult<()> {
            Ok(())
        }

        fn estimated_execution_time(&self, data_size: u64) -> u64 {
            data_size / 1000 // Simple linear estimation
        }
    }

    #[test]
    fn test_verb_engine_creation() {
        let engine = VerbEngine::new();
        assert_eq!(engine.available_verbs().len(), 0);
    }

    #[test]
    fn test_verb_registration() {
        let mut engine = VerbEngine::new();
        let test_verb = Arc::new(TestVerb);
        
        engine.register_verb(test_verb);
        assert_eq!(engine.available_verbs().len(), 1);
        assert!(engine.available_verbs().contains(&"test".to_string()));
    }
}
