// Cache-Oblivious Algorithms Implementation
// Knuthian Optimization Step 8: Cache-Oblivious Data Structures and Algorithms

use std::cmp::{min, max};
use std::mem;
use crate::algorithms::{ComplexityTracker, AlgorithmMetrics};

/// Cache-oblivious matrix multiplication using recursive blocking
#[derive(Debug)]
pub struct CacheOptimalMatrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
    metrics: AlgorithmMetrics,
}

impl<T: Clone + Default + std::ops::Add<Output = T> + std::ops::Mul<Output = T>> CacheOptimalMatrix<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![T::default(); rows * cols],
            rows,
            cols,
            metrics: AlgorithmMetrics::new("CacheObliviousMatrix", "O(n^3)", "O(n^2)"),
        }
    }

    pub fn from_vec(data: Vec<T>, rows: usize, cols: usize) -> Self {
        assert_eq!(data.len(), rows * cols);
        Self {
            data,
            rows,
            cols,
            metrics: AlgorithmMetrics::new("CacheObliviousMatrix", "O(n^3)", "O(n^2)"),
        }
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row * self.cols + col] = value;
    }

    /// Cache-oblivious matrix multiplication using recursive subdivision
    pub fn multiply(&mut self, other: &CacheOptimalMatrix<T>) -> CacheOptimalMatrix<T> 
    where 
        T: Copy + std::ops::AddAssign,
    {
        assert_eq!(self.cols, other.rows);
        
        let mut result = CacheOptimalMatrix::new(self.rows, other.cols);
        
        self.multiply_recursive(
            other, 
            &mut result,
            0, 0, 0, 0, 0, 0,
            self.rows, self.cols, other.cols
        );
        
        result
    }

    fn multiply_recursive(
        &mut self,
        b: &CacheOptimalMatrix<T>,
        c: &mut CacheOptimalMatrix<T>,
        a_row: usize, a_col: usize,
        b_row: usize, b_col: usize,
        c_row: usize, c_col: usize,
        rows: usize, inner: usize, cols: usize,
    ) where 
        T: Copy + std::ops::AddAssign,
    {
        self.metrics.complexity.record_operation();
        
        // Base case: use simple multiplication for small matrices
        if rows <= 32 && inner <= 32 && cols <= 32 {
            for i in 0..rows {
                for j in 0..cols {
                    for k in 0..inner {
                        let a_val = *self.get(a_row + i, a_col + k);
                        let b_val = *b.get(b_row + k, b_col + j);
                        let current = *c.get(c_row + i, c_col + j);
                        c.set(c_row + i, c_col + j, current + a_val * b_val);
                    }
                }
            }
            return;
        }

        // Recursive case: divide matrices
        if rows >= max(inner, cols) {
            let mid = rows / 2;
            // Split A horizontally, C horizontally
            self.multiply_recursive(b, c, a_row, a_col, b_row, b_col, c_row, c_col, mid, inner, cols);
            self.multiply_recursive(b, c, a_row + mid, a_col, b_row, b_col, c_row + mid, c_col, 
                                  rows - mid, inner, cols);
        } else if inner >= max(rows, cols) {
            let mid = inner / 2;
            // Split A vertically, B horizontally
            self.multiply_recursive(b, c, a_row, a_col, b_row, b_col, c_row, c_col, rows, mid, cols);
            self.multiply_recursive(b, c, a_row, a_col + mid, b_row + mid, b_col, c_row, c_col, 
                                  rows, inner - mid, cols);
        } else {
            let mid = cols / 2;
            // Split B vertically, C vertically
            self.multiply_recursive(b, c, a_row, a_col, b_row, b_col, c_row, c_col, rows, inner, mid);
            self.multiply_recursive(b, c, a_row, a_col, b_row, b_col + mid, c_row, c_col + mid, 
                                  rows, inner, cols - mid);
        }
    }

    /// Cache-oblivious matrix transpose
    pub fn transpose(&self) -> CacheOptimalMatrix<T> 
    where 
        T: Copy,
    {
        let mut result = CacheOptimalMatrix::new(self.cols, self.rows);
        self.transpose_recursive(&mut result, 0, 0, 0, 0, self.rows, self.cols);
        result
    }

    fn transpose_recursive(
        &self,
        result: &mut CacheOptimalMatrix<T>,
        src_row: usize, src_col: usize,
        dst_row: usize, dst_col: usize,
        rows: usize, cols: usize,
    ) where 
        T: Copy,
    {
        // Base case: transpose small blocks directly
        if rows <= 16 && cols <= 16 {
            for i in 0..rows {
                for j in 0..cols {
                    let value = *self.get(src_row + i, src_col + j);
                    result.set(dst_row + j, dst_col + i, value);
                }
            }
            return;
        }

        // Recursive case
        if rows >= cols {
            let mid = rows / 2;
            self.transpose_recursive(result, src_row, src_col, dst_row, dst_col, mid, cols);
            self.transpose_recursive(result, src_row + mid, src_col, dst_row, dst_col + mid, 
                                   rows - mid, cols);
        } else {
            let mid = cols / 2;
            self.transpose_recursive(result, src_row, src_col, dst_row, dst_col, rows, mid);
            self.transpose_recursive(result, src_row, src_col + mid, dst_row + mid, dst_col, 
                                   rows, cols - mid);
        }
    }

    pub fn get_metrics(&self) -> &AlgorithmMetrics {
        &self.metrics
    }
}

/// Cache-oblivious sorting using funnelsort
pub struct CacheObliviousSorter<T> {
    metrics: AlgorithmMetrics,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Clone + Ord> CacheObliviousSorter<T> {
    pub fn new() -> Self {
        Self {
            metrics: AlgorithmMetrics::new("CacheObliviousSort", "O(n log n)", "O(n)"),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Cache-oblivious sorting using recursive merge sort with optimal base case
    pub fn sort(&mut self, data: &mut [T]) {
        if data.len() <= 1 {
            return;
        }
        
        self.cache_oblivious_sort(data);
    }

    fn cache_oblivious_sort(&mut self, data: &mut [T]) {
        self.metrics.complexity.record_operation();
        
        let n = data.len();
        if n <= 32 {
            // Use insertion sort for small arrays (cache-friendly)
            self.insertion_sort(data);
            return;
        }

        // Divide
        let mid = n / 2;
        self.cache_oblivious_sort(&mut data[..mid]);
        self.cache_oblivious_sort(&mut data[mid..]);
        
        // Merge using cache-oblivious merge
        self.cache_oblivious_merge(data, mid);
    }

    fn insertion_sort(&mut self, data: &mut [T]) {
        for i in 1..data.len() {
            let mut j = i;
            while j > 0 && data[j] < data[j - 1] {
                data.swap(j, j - 1);
                j -= 1;
            }
            self.metrics.complexity.record_operation();
        }
    }

    fn cache_oblivious_merge(&mut self, data: &mut [T], mid: usize) {
        let n = data.len();
        let mut temp = Vec::with_capacity(n);
        
        // Copy data to temporary array
        for item in data.iter() {
            temp.push(item.clone());
        }
        
        let (left, right) = temp.split_at(mid);
        self.merge_arrays(data, left, right);
    }

    fn merge_arrays(&mut self, result: &mut [T], left: &[T], right: &[T]) {
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        
        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                result[k] = left[i].clone();
                i += 1;
            } else {
                result[k] = right[j].clone();
                j += 1;
            }
            k += 1;
            self.metrics.complexity.record_operation();
        }
        
        while i < left.len() {
            result[k] = left[i].clone();
            i += 1;
            k += 1;
        }
        
        while j < right.len() {
            result[k] = right[j].clone();
            j += 1;
            k += 1;
        }
    }

    pub fn get_metrics(&self) -> &AlgorithmMetrics {
        &self.metrics
    }
}

/// Cache-oblivious B-tree implementation
#[derive(Debug)]
pub struct CacheObliviousBTree<K, V> {
    root: Option<Box<BTreeNode<K, V>>>,
    height: usize,
    size: usize,
    metrics: AlgorithmMetrics,
}

#[derive(Debug)]
struct BTreeNode<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
    children: Vec<Box<BTreeNode<K, V>>>,
    is_leaf: bool,
}

impl<K: Ord + Clone, V: Clone> CacheObliviousBTree<K, V> {
    pub fn new() -> Self {
        Self {
            root: None,
            height: 0,
            size: 0,
            metrics: AlgorithmMetrics::new("CacheObliviousBTree", "O(log_B n)", "O(n)"),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.metrics.complexity.record_operation();
        
        if self.root.is_none() {
            let mut node = BTreeNode::new_leaf();
            node.keys.push(key);
            node.values.push(value);
            self.root = Some(Box::new(node));
            self.height = 1;
        } else {
            if let Some(new_root) = self.insert_into_node(&mut self.root.as_mut().unwrap(), key, value) {
                let mut new_root_node = BTreeNode::new_internal();
                new_root_node.children.push(self.root.take().unwrap());
                new_root_node.children.push(new_root);
                // In practice, you'd need to handle key promotion
                self.root = Some(Box::new(new_root_node));
                self.height += 1;
            }
        }
        self.size += 1;
    }

    fn insert_into_node(&mut self, node: &mut BTreeNode<K, V>, key: K, value: V) 
        -> Option<Box<BTreeNode<K, V>>> {
        
        if node.is_leaf {
            // Insert into leaf
            let pos = node.keys.binary_search(&key).unwrap_or_else(|e| e);
            node.keys.insert(pos, key);
            node.values.insert(pos, value);
            
            // Check if split is needed (simplified - using fixed threshold)
            if node.keys.len() > 4 {
                return Some(self.split_leaf(node));
            }
        } else {
            // Find child to insert into
            let child_index = node.keys.binary_search(&key).unwrap_or_else(|e| e);
            if let Some(new_child) = self.insert_into_node(&mut node.children[child_index], key, value) {
                // Handle child split
                node.children.insert(child_index + 1, new_child);
                // In practice, you'd promote the median key
                
                if node.children.len() > 5 {
                    return Some(self.split_internal(node));
                }
            }
        }
        
        None
    }

    fn split_leaf(&self, node: &mut BTreeNode<K, V>) -> Box<BTreeNode<K, V>> {
        let mid = node.keys.len() / 2;
        let mut new_node = BTreeNode::new_leaf();
        
        new_node.keys = node.keys.split_off(mid);
        new_node.values = node.values.split_off(mid);
        
        Box::new(new_node)
    }

    fn split_internal(&self, node: &mut BTreeNode<K, V>) -> Box<BTreeNode<K, V>> {
        let mid = node.children.len() / 2;
        let mut new_node = BTreeNode::new_internal();
        
        new_node.children = node.children.split_off(mid);
        
        Box::new(new_node)
    }

    pub fn search(&mut self, key: &K) -> Option<&V> {
        self.metrics.complexity.record_operation();
        self.search_in_node(self.root.as_ref()?, key)
    }

    fn search_in_node(&self, node: &BTreeNode<K, V>, key: &K) -> Option<&V> {
        match node.keys.binary_search(key) {
            Ok(index) => {
                if node.is_leaf {
                    Some(&node.values[index])
                } else {
                    self.search_in_node(&node.children[index], key)
                }
            }
            Err(index) => {
                if node.is_leaf {
                    None
                } else {
                    self.search_in_node(&node.children[index], key)
                }
            }
        }
    }

    pub fn get_metrics(&self) -> &AlgorithmMetrics {
        &self.metrics
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<K, V> BTreeNode<K, V> {
    fn new_leaf() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
            is_leaf: true,
        }
    }

    fn new_internal() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
            is_leaf: false,
        }
    }
}

/// Cache-oblivious algorithms collection
pub struct CacheObliviousAlgorithms;

impl CacheObliviousAlgorithms {
    /// Cache-oblivious scanning with optimal I/O complexity
    pub fn scan_with_pattern<T, F>(data: &[T], predicate: F) -> Vec<usize>
    where
        F: Fn(&T) -> bool,
        T: Clone,
    {
        let mut results = Vec::new();
        let block_size = Self::optimal_block_size(data.len());
        
        for (block_start, chunk) in data.chunks(block_size).enumerate() {
            for (i, item) in chunk.iter().enumerate() {
                if predicate(item) {
                    results.push(block_start * block_size + i);
                }
            }
        }
        
        results
    }

    /// Calculate optimal block size for cache-oblivious algorithms
    fn optimal_block_size(n: usize) -> usize {
        // Heuristic: use sqrt(n) for optimal cache performance
        (n as f64).sqrt().ceil() as usize
    }

    /// Cache-oblivious distribution sort
    pub fn distribution_sort<T: Clone + Ord>(data: &mut [T]) {
        let n = data.len();
        if n <= 32 {
            data.sort();
            return;
        }

        // Sample splitters
        let sample_size = (n as f64).sqrt() as usize;
        let mut splitters = Vec::new();
        
        for i in 0..sample_size {
            let index = (i * n) / sample_size;
            splitters.push(data[index].clone());
        }
        splitters.sort();

        // Distribute into buckets
        let mut buckets: Vec<Vec<T>> = vec![Vec::new(); sample_size + 1];
        
        for item in data.iter() {
            let bucket_index = splitters.binary_search(item).unwrap_or_else(|e| e);
            buckets[bucket_index].push(item.clone());
        }

        // Recursively sort buckets and merge
        let mut result_index = 0;
        for bucket in &mut buckets {
            Self::distribution_sort(bucket);
            for item in bucket {
                data[result_index] = item.clone();
                result_index += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_oblivious_matrix_multiply() {
        let mut a = CacheOptimalMatrix::from_vec(vec![1, 2, 3, 4], 2, 2);
        let b = CacheOptimalMatrix::from_vec(vec![5, 6, 7, 8], 2, 2);
        let c = a.multiply(&b);
        assert!(c.get_metrics().complexity.operations_count > 0);
    }

    #[test]
    fn test_cache_oblivious_sort() {
        let mut sorter = CacheObliviousSorter::new();
        let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        sorter.sort(&mut data);
        assert_eq!(data, vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
    }

    #[test]
    fn test_cache_oblivious_btree() {
        let mut tree = CacheObliviousBTree::new();
        tree.insert(1, "one");
        tree.insert(2, "two");
        tree.insert(3, "three");
        
        assert_eq!(tree.search(&2), Some(&"two"));
        assert_eq!(tree.search(&4), None);
    }

    #[test]
    fn test_distribution_sort() {
        let mut data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        CacheObliviousAlgorithms::distribution_sort(&mut data);
        assert_eq!(data, vec![1, 1, 2, 3, 4, 5, 5, 6, 9]);
    }
}
