// Probabilistic Data Structures Implementation
// Knuthian Optimization Step 9: Bloom Filters, HyperLogLog, and Count-Min Sketch

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::f64::consts::LN_2;
use crate::algorithms::{ComplexityTracker, AlgorithmMetrics};

/// Bloom Filter for membership testing with false positives
#[derive(Debug, Clone)]
pub struct BloomFilter {
    bit_array: Vec<bool>,
    size: usize,
    hash_functions: usize,
    items_count: usize,
    metrics: AlgorithmMetrics,
}

impl BloomFilter {
    /// Create new Bloom filter with optimal parameters
    pub fn new(expected_items: usize, false_positive_rate: f64) -> Self {
        let size = Self::optimal_size(expected_items, false_positive_rate);
        let hash_functions = Self::optimal_hash_functions(size, expected_items);
        
        Self {
            bit_array: vec![false; size],
            size,
            hash_functions,
            items_count: 0,
            metrics: AlgorithmMetrics::new("BloomFilter", "O(k)", "O(m)"),
        }
    }

    /// Calculate optimal bit array size
    fn optimal_size(n: usize, p: f64) -> usize {
        let size = -(n as f64 * p.ln()) / (LN_2 * LN_2);
        size.ceil() as usize
    }

    /// Calculate optimal number of hash functions
    fn optimal_hash_functions(m: usize, n: usize) -> usize {
        let k = (m as f64 / n as f64) * LN_2;
        (k.ceil() as usize).max(1)
    }

    /// Add item to the filter
    pub fn insert<T: Hash>(&mut self, item: &T) {
        self.metrics.complexity.record_operation();
        
        for i in 0..self.hash_functions {
            let hash = self.hash_with_seed(item, i);
            let index = (hash % self.size as u64) as usize;
            self.bit_array[index] = true;
        }
        
        self.items_count += 1;
    }

    /// Test if item might be in the set (no false negatives)
    pub fn contains<T: Hash>(&mut self, item: &T) -> bool {
        self.metrics.complexity.record_operation();
        
        for i in 0..self.hash_functions {
            let hash = self.hash_with_seed(item, i);
            let index = (hash % self.size as u64) as usize;
            if !self.bit_array[index] {
                return false;
            }
        }
        
        true
    }

    /// Hash with seed for multiple hash functions
    fn hash_with_seed<T: Hash>(&self, item: &T, seed: usize) -> u64 {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        seed.hash(&mut hasher);
        hasher.finish()
    }

    /// Estimate current false positive probability
    pub fn false_positive_probability(&self) -> f64 {
        let ratio = self.items_count as f64 / self.size as f64;
        (1.0 - (-(self.hash_functions as f64) * ratio).exp()).powi(self.hash_functions as i32)
    }

    /// Union with another Bloom filter (same parameters required)
    pub fn union(&mut self, other: &BloomFilter) -> Result<(), String> {
        if self.size != other.size || self.hash_functions != other.hash_functions {
            return Err("Bloom filters must have same parameters".to_string());
        }

        for i in 0..self.size {
            self.bit_array[i] = self.bit_array[i] || other.bit_array[i];
        }

        self.items_count = self.items_count.max(other.items_count);
        Ok(())
    }

    /// Intersection with another Bloom filter
    pub fn intersection(&mut self, other: &BloomFilter) -> Result<(), String> {
        if self.size != other.size || self.hash_functions != other.hash_functions {
            return Err("Bloom filters must have same parameters".to_string());
        }

        for i in 0..self.size {
            self.bit_array[i] = self.bit_array[i] && other.bit_array[i];
        }

        self.items_count = self.items_count.min(other.items_count);
        Ok(())
    }

    pub fn get_metrics(&self) -> &AlgorithmMetrics {
        &self.metrics
    }

    pub fn len(&self) -> usize {
        self.items_count
    }

    pub fn is_empty(&self) -> bool {
        self.items_count == 0
    }
}

/// HyperLogLog for cardinality estimation
#[derive(Debug, Clone)]
pub struct HyperLogLog {
    buckets: Vec<u8>,
    bucket_count: usize,
    alpha: f64,
    metrics: AlgorithmMetrics,
}

impl HyperLogLog {
    /// Create new HyperLogLog with 2^precision buckets
    pub fn new(precision: u8) -> Self {
        let bucket_count = 1 << precision;
        let alpha = Self::calculate_alpha(bucket_count);
        
        Self {
            buckets: vec![0; bucket_count],
            bucket_count,
            alpha,
            metrics: AlgorithmMetrics::new("HyperLogLog", "O(1)", "O(2^p)"),
        }
    }

    fn calculate_alpha(m: usize) -> f64 {
        match m {
            16 => 0.673,
            32 => 0.697,
            64 => 0.709,
            _ => 0.7213 / (1.0 + 1.079 / m as f64),
        }
    }

    /// Add item to the estimator
    pub fn add<T: Hash>(&mut self, item: &T) {
        self.metrics.complexity.record_operation();
        
        let hash = self.hash_item(item);
        let bucket_index = (hash & (self.bucket_count - 1) as u64) as usize;
        let leading_zeros = (hash >> (64 - 6)).leading_zeros() as u8 + 1;
        
        self.buckets[bucket_index] = self.buckets[bucket_index].max(leading_zeros);
    }

    fn hash_item<T: Hash>(&self, item: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        hasher.finish()
    }

    /// Estimate cardinality
    pub fn estimate(&mut self) -> f64 {
        self.metrics.complexity.record_operation();
        
        let raw_estimate = self.alpha * (self.bucket_count as f64).powi(2) / 
            self.buckets.iter().map(|&b| 2.0_f64.powi(-(b as i32))).sum::<f64>();

        // Apply small range and large range corrections
        if raw_estimate <= 2.5 * self.bucket_count as f64 {
            // Small range correction
            let zeros = self.buckets.iter().filter(|&&b| b == 0).count();
            if zeros != 0 {
                return (self.bucket_count as f64) * (self.bucket_count as f64 / zeros as f64).ln();
            }
        }

        if raw_estimate <= (1.0 / 30.0) * (2.0_f64.powi(32)) {
            raw_estimate
        } else {
            // Large range correction
            -2.0_f64.powi(32) * (1.0 - raw_estimate / 2.0_f64.powi(32)).ln()
        }
    }

    /// Merge with another HyperLogLog (union operation)
    pub fn merge(&mut self, other: &HyperLogLog) -> Result<(), String> {
        if self.bucket_count != other.bucket_count {
            return Err("HyperLogLog instances must have same precision".to_string());
        }

        for i in 0..self.bucket_count {
            self.buckets[i] = self.buckets[i].max(other.buckets[i]);
        }

        Ok(())
    }

    pub fn get_metrics(&self) -> &AlgorithmMetrics {
        &self.metrics
    }
}

/// Count-Min Sketch for frequency estimation
#[derive(Debug, Clone)]
pub struct CountMinSketch {
    table: Vec<Vec<u32>>,
    width: usize,
    depth: usize,
    total_count: u64,
    metrics: AlgorithmMetrics,
}

impl CountMinSketch {
    /// Create new Count-Min Sketch with given error bounds
    pub fn new(epsilon: f64, delta: f64) -> Self {
        let width = (std::f64::consts::E / epsilon).ceil() as usize;
        let depth = (1.0 / delta).ln().ceil() as usize;
        
        Self {
            table: vec![vec![0; width]; depth],
            width,
            depth,
            total_count: 0,
            metrics: AlgorithmMetrics::new("CountMinSketch", "O(log(1/δ))", "O(log(1/δ)/ε)"),
        }
    }

    /// Add item with given count
    pub fn add<T: Hash>(&mut self, item: &T, count: u32) {
        self.metrics.complexity.record_operation();
        
        for i in 0..self.depth {
            let hash = self.hash_with_seed(item, i);
            let col = (hash % self.width as u64) as usize;
            self.table[i][col] += count;
        }
        
        self.total_count += count as u64;
    }

    /// Estimate frequency of item
    pub fn estimate<T: Hash>(&mut self, item: &T) -> u32 {
        self.metrics.complexity.record_operation();
        
        let mut min_count = u32::MAX;
        
        for i in 0..self.depth {
            let hash = self.hash_with_seed(item, i);
            let col = (hash % self.width as u64) as usize;
            min_count = min_count.min(self.table[i][col]);
        }
        
        min_count
    }

    fn hash_with_seed<T: Hash>(&self, item: &T, seed: usize) -> u64 {
        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        seed.hash(&mut hasher);
        hasher.finish()
    }

    /// Merge with another Count-Min Sketch
    pub fn merge(&mut self, other: &CountMinSketch) -> Result<(), String> {
        if self.width != other.width || self.depth != other.depth {
            return Err("Count-Min Sketches must have same dimensions".to_string());
        }

        for i in 0..self.depth {
            for j in 0..self.width {
                self.table[i][j] += other.table[i][j];
            }
        }

        self.total_count += other.total_count;
        Ok(())
    }

    /// Estimate total number of items
    pub fn total_count(&self) -> u64 {
        self.total_count
    }

    /// Get heavy hitters (items with frequency > threshold)
    pub fn heavy_hitters(&self, threshold: f64) -> Vec<(usize, u32)> {
        let mut heavy_hitters = Vec::new();
        let min_count = (threshold * self.total_count as f64) as u32;

        // This is a simplified version - in practice you'd need to track actual items
        for i in 0..self.depth {
            for j in 0..self.width {
                if self.table[i][j] >= min_count {
                    heavy_hitters.push((j, self.table[i][j]));
                }
            }
        }

        heavy_hitters.sort_by(|a, b| b.1.cmp(&a.1));
        heavy_hitters.dedup_by(|a, b| a.0 == b.0);
        heavy_hitters
    }

    pub fn get_metrics(&self) -> &AlgorithmMetrics {
        &self.metrics
    }
}

/// Skip List with probabilistic balancing
#[derive(Debug)]
pub struct SkipList<K, V> {
    head: Box<SkipNode<K, V>>,
    max_level: usize,
    level: usize,
    size: usize,
    metrics: AlgorithmMetrics,
}

#[derive(Debug)]
struct SkipNode<K, V> {
    key: Option<K>,
    value: Option<V>,
    forward: Vec<Option<Box<SkipNode<K, V>>>>,
}

impl<K: Ord + Clone, V: Clone> SkipList<K, V> {
    pub fn new(max_level: usize) -> Self {
        let head = Box::new(SkipNode {
            key: None,
            value: None,
            forward: vec![None; max_level + 1],
        });

        Self {
            head,
            max_level,
            level: 0,
            size: 0,
            metrics: AlgorithmMetrics::new("SkipList", "O(log n)", "O(n)"),
        }
    }

    /// Generate random level using geometric distribution
    fn random_level(&self) -> usize {
        let mut level = 0;
        while level < self.max_level && rand::random::<f64>() < 0.5 {
            level += 1;
        }
        level
    }

    /// Insert key-value pair
    pub fn insert(&mut self, key: K, value: V) {
        self.metrics.complexity.record_operation();
        
        let mut update = vec![None; self.max_level + 1];
        let mut current = &mut self.head;

        // Find insertion position
        for i in (0..=self.level).rev() {
            while let Some(ref next) = current.forward[i] {
                if let Some(ref next_key) = next.key {
                    if *next_key < key {
                        current = current.forward[i].as_mut().unwrap();
                        continue;
                    }
                }
                break;
            }
            update[i] = Some(current as *mut SkipNode<K, V>);
        }

        let new_level = self.random_level();
        if new_level > self.level {
            for i in (self.level + 1)..=new_level {
                update[i] = Some(&mut *self.head as *mut SkipNode<K, V>);
            }
            self.level = new_level;
        }

        let mut new_node = Box::new(SkipNode {
            key: Some(key),
            value: Some(value),
            forward: vec![None; self.max_level + 1],
        });

        // Update forward pointers
        for i in 0..=new_level {
            if let Some(update_ptr) = update[i] {
                unsafe {
                    new_node.forward[i] = (*update_ptr).forward[i].take();
                    (*update_ptr).forward[i] = Some(new_node);
                    new_node = (*update_ptr).forward[i].as_mut().unwrap();
                }
            }
        }

        self.size += 1;
    }

    /// Search for key
    pub fn search(&mut self, key: &K) -> Option<&V> {
        self.metrics.complexity.record_operation();
        
        let mut current = &self.head;

        for i in (0..=self.level).rev() {
            while let Some(ref next) = current.forward[i] {
                if let Some(ref next_key) = next.key {
                    match next_key.cmp(key) {
                        std::cmp::Ordering::Less => {
                            current = next;
                            continue;
                        }
                        std::cmp::Ordering::Equal => {
                            return next.value.as_ref();
                        }
                        std::cmp::Ordering::Greater => break,
                    }
                }
                break;
            }
        }

        None
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn get_metrics(&self) -> &AlgorithmMetrics {
        &self.metrics
    }
}

/// Probabilistic data structure collection
pub struct ProbabilisticStructures;

impl ProbabilisticStructures {
    /// Create optimal Bloom filter for given constraints
    pub fn create_bloom_filter(expected_items: usize, max_fp_rate: f64) -> BloomFilter {
        BloomFilter::new(expected_items, max_fp_rate)
    }

    /// Create HyperLogLog with precision for given max cardinality
    pub fn create_hyperloglog(max_cardinality: u64) -> HyperLogLog {
        let precision = (max_cardinality as f64).log2().ceil() as u8;
        HyperLogLog::new(precision.min(16).max(4))
    }

    /// Create Count-Min Sketch for frequency analysis
    pub fn create_count_min_sketch(error_rate: f64, confidence: f64) -> CountMinSketch {
        CountMinSketch::new(error_rate, 1.0 - confidence)
    }

    /// Estimate set operations using probabilistic structures
    pub fn estimate_set_similarity(
        set1: &[String], 
        set2: &[String], 
        fp_rate: f64
    ) -> f64 {
        let expected_size = set1.len().max(set2.len());
        let mut bloom1 = BloomFilter::new(expected_size, fp_rate);
        let mut bloom2 = BloomFilter::new(expected_size, fp_rate);

        for item in set1 {
            bloom1.insert(item);
        }

        for item in set2 {
            bloom2.insert(item);
        }

        // Approximate Jaccard similarity using Bloom filters
        let mut intersection_count = 0;
        let mut union_count = 0;

        for item in set1.iter().chain(set2.iter()) {
            let in_both = bloom1.contains(item) && bloom2.contains(item);
            let in_either = bloom1.contains(item) || bloom2.contains(item);

            if in_both {
                intersection_count += 1;
            }
            if in_either {
                union_count += 1;
            }
        }

        if union_count == 0 {
            0.0
        } else {
            intersection_count as f64 / union_count as f64
        }
    }
}

// Simplified rand implementation for testing
mod rand {
    use std::cell::RefCell;
    
    thread_local! {
        static RNG_STATE: RefCell<u64> = RefCell::new(1234567890);
    }
    
    pub fn random<T>() -> T 
    where 
        T: From<f64>,
    {
        RNG_STATE.with(|state| {
            let mut s = state.borrow_mut();
            *s = (*s).wrapping_mul(1103515245).wrapping_add(12345);
            let normalized = (*s as f64) / (u64::MAX as f64);
            T::from(normalized)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bloom_filter() {
        let mut filter = BloomFilter::new(1000, 0.01);
        filter.insert(&"hello");
        filter.insert(&"world");
        
        assert!(filter.contains(&"hello"));
        assert!(filter.contains(&"world"));
        assert!(!filter.contains(&"absent"));
    }

    #[test]
    fn test_hyperloglog() {
        let mut hll = HyperLogLog::new(8);
        for i in 0..1000 {
            hll.add(&i);
        }
        
        let estimate = hll.estimate();
        assert!(estimate > 900.0 && estimate < 1100.0);
    }

    #[test]
    fn test_count_min_sketch() {
        let mut cms = CountMinSketch::new(0.01, 0.01);
        cms.add(&"hello", 5);
        cms.add(&"world", 3);
        cms.add(&"hello", 2);
        
        assert!(cms.estimate(&"hello") >= 7);
        assert!(cms.estimate(&"world") >= 3);
    }

    #[test]
    fn test_skip_list() {
        let mut skip_list = SkipList::new(10);
        skip_list.insert(1, "one");
        skip_list.insert(2, "two");
        skip_list.insert(3, "three");
        
        assert_eq!(skip_list.search(&2), Some(&"two"));
        assert_eq!(skip_list.search(&4), None);
    }

    #[test]
    fn test_probabilistic_set_similarity() {
        let set1 = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let set2 = vec!["b".to_string(), "c".to_string(), "d".to_string()];
        
        let similarity = ProbabilisticStructures::estimate_set_similarity(&set1, &set2, 0.01);
        assert!(similarity > 0.0 && similarity <= 1.0);
    }
}
