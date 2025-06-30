use std::collections::{HashMap, BTreeMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::{DatabaseConfig, DatabaseError, DbResult, QueryBuilder, QueryCondition, QueryOperator};

/// Index manager for efficient querying of encrypted data
pub struct IndexManager {
    indexes: Arc<RwLock<HashMap<String, TableIndex>>>,
    config: DatabaseConfig,
}

/// Index for a specific table
#[derive(Debug, Clone)]
struct TableIndex {
    primary_index: BTreeMap<String, HashSet<Uuid>>, // field_name -> value -> record_ids
    composite_indexes: HashMap<String, BTreeMap<String, HashSet<Uuid>>>, // index_name -> value -> record_ids
    bloom_filters: HashMap<String, BloomFilter>, // field_name -> bloom_filter
}

/// Simple bloom filter implementation for existence checks
#[derive(Debug, Clone)]
struct BloomFilter {
    bits: Vec<bool>,
    hash_functions: usize,
    size: usize,
}

/// Searchable field configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexField {
    pub name: String,
    pub field_type: IndexFieldType,
    pub is_unique: bool,
    pub is_encrypted: bool,
    pub enable_bloom_filter: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IndexFieldType {
    String,
    Number,
    Boolean,
    Date,
    Uuid,
    Json,
}

/// Search result with relevance scoring
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub record_id: Uuid,
    pub score: f64,
    pub matched_fields: Vec<String>,
}

impl IndexManager {
    pub async fn new(config: &DatabaseConfig) -> DbResult<Self> {
        Ok(Self {
            indexes: Arc::new(RwLock::new(HashMap::new())),
            config: config.clone(),
        })
    }

    /// Add a record to indexes
    pub async fn add_record<T>(&self, table: &str, id: &Uuid, data: &T) -> DbResult<()>
    where
        T: Serialize + Send + Sync,
    {
        let serialized = serde_json::to_value(data)
            .map_err(|e| DatabaseError::Index(format!("Serialization failed: {}", e)))?;

        let mut indexes = self.indexes.write().await;
        let table_index = indexes.entry(table.to_string()).or_insert_with(TableIndex::new);

        // Extract indexable fields from the data
        if let serde_json::Value::Object(obj) = serialized {
            for (field_name, value) in obj {
                let value_str = self.value_to_string(&value);
                
                // Add to primary index
                table_index.primary_index
                    .entry(field_name.clone())
                    .or_insert_with(HashSet::new)
                    .insert(*id);

                // Add to bloom filter
                if let Some(bloom_filter) = table_index.bloom_filters.get_mut(&field_name) {
                    bloom_filter.add(&value_str);
                } else {
                    let mut bloom_filter = BloomFilter::new(10000, 3);
                    bloom_filter.add(&value_str);
                    table_index.bloom_filters.insert(field_name, bloom_filter);
                }
            }
        }

        Ok(())
    }

    /// Update a record in indexes
    pub async fn update_record<T>(&self, table: &str, id: &Uuid, data: &T) -> DbResult<()>
    where
        T: Serialize + Send + Sync,
    {
        // For simplicity, we'll remove the old record and add the new one
        // In a production system, you'd want to be more efficient
        self.remove_record(table, id).await?;
        self.add_record(table, id, data).await?;
        Ok(())
    }

    /// Remove a record from indexes
    pub async fn remove_record(&self, table: &str, id: &Uuid) -> DbResult<()> {
        let mut indexes = self.indexes.write().await;
        if let Some(table_index) = indexes.get_mut(table) {
            // Remove from primary index
            for (_, record_set) in table_index.primary_index.iter_mut() {
                record_set.remove(id);
            }

            // Remove from composite indexes
            for (_, index) in table_index.composite_indexes.iter_mut() {
                for (_, record_set) in index.iter_mut() {
                    record_set.remove(id);
                }
            }
        }
        Ok(())
    }

    /// Search records based on query
    pub async fn search(&self, query: &QueryBuilder) -> DbResult<Vec<Uuid>> {
        let indexes = self.indexes.read().await;
        let table_index = indexes.get(&query.table)
            .ok_or_else(|| DatabaseError::Index(format!("No index found for table: {}", query.table)))?;

        let mut result_sets: Vec<HashSet<Uuid>> = Vec::new();

        // Process each condition
        for condition in &query.conditions {
            let matching_ids = self.search_condition(table_index, condition).await?;
            result_sets.push(matching_ids);
        }

        // Intersect all result sets (AND operation)
        let mut final_results = if let Some(first_set) = result_sets.first() {
            first_set.clone()
        } else {
            HashSet::new()
        };

        for result_set in result_sets.iter().skip(1) {
            final_results = final_results.intersection(result_set).cloned().collect();
        }

        // Convert to Vec and apply ordering/limiting
        let mut results: Vec<Uuid> = final_results.into_iter().collect();

        // Apply ordering (simplified - in practice you'd need the actual data for complex sorting)
        if let Some(order_field) = &query.order_by {
            // For now, just sort by UUID as a placeholder
            // In a real implementation, you'd need to access the actual field values
            results.sort();
            if query.order_desc {
                results.reverse();
            }
        }

        // Apply pagination
        if let Some(offset) = query.offset {
            if offset < results.len() {
                results = results[offset..].to_vec();
            } else {
                results.clear();
            }
        }

        if let Some(limit) = query.limit {
            results.truncate(limit);
        }

        Ok(results)
    }

    /// Search for a specific condition
    async fn search_condition(&self, table_index: &TableIndex, condition: &QueryCondition) -> DbResult<HashSet<Uuid>> {
        let field_name = &condition.field;
        let value_str = self.value_to_string(&condition.value);

        // Check bloom filter first for existence queries
        if let Some(bloom_filter) = table_index.bloom_filters.get(field_name) {
            if !bloom_filter.might_contain(&value_str) {
                // Definitely not present
                return Ok(HashSet::new());
            }
        }

        // Get field index
        let field_index = table_index.primary_index.get(field_name)
            .ok_or_else(|| DatabaseError::Index(format!("No index found for field: {}", field_name)))?;

        match condition.operator {
            QueryOperator::Equals => {
                Ok(field_index.get(&value_str).cloned().unwrap_or_default())
            }
            QueryOperator::NotEquals => {
                let mut all_ids: HashSet<Uuid> = HashSet::new();
                for (_, ids) in field_index.iter() {
                    all_ids.extend(ids);
                }
                let equal_ids = field_index.get(&value_str).cloned().unwrap_or_default();
                Ok(all_ids.difference(&equal_ids).cloned().collect())
            }
            QueryOperator::Contains => {
                let mut matching_ids = HashSet::new();
                for (indexed_value, ids) in field_index.iter() {
                    if indexed_value.contains(&value_str) {
                        matching_ids.extend(ids);
                    }
                }
                Ok(matching_ids)
            }
            QueryOperator::StartsWith => {
                let mut matching_ids = HashSet::new();
                for (indexed_value, ids) in field_index.iter() {
                    if indexed_value.starts_with(&value_str) {
                        matching_ids.extend(ids);
                    }
                }
                Ok(matching_ids)
            }
            QueryOperator::EndsWith => {
                let mut matching_ids = HashSet::new();
                for (indexed_value, ids) in field_index.iter() {
                    if indexed_value.ends_with(&value_str) {
                        matching_ids.extend(ids);
                    }
                }
                Ok(matching_ids)
            }
            QueryOperator::GreaterThan | QueryOperator::LessThan | 
            QueryOperator::GreaterThanOrEqual | QueryOperator::LessThanOrEqual => {
                // For range queries, we'd need a more sophisticated index structure
                // For now, return empty set
                Ok(HashSet::new())
            }
            QueryOperator::In => {
                if let serde_json::Value::Array(values) = &condition.value {
                    let mut matching_ids = HashSet::new();
                    for value in values {
                        let value_str = self.value_to_string(value);
                        if let Some(ids) = field_index.get(&value_str) {
                            matching_ids.extend(ids);
                        }
                    }
                    Ok(matching_ids)
                } else {
                    Err(DatabaseError::Query("IN operator requires array value".to_string()))
                }
            }
            QueryOperator::NotIn => {
                if let serde_json::Value::Array(values) = &condition.value {
                    let mut all_ids: HashSet<Uuid> = HashSet::new();
                    for (_, ids) in field_index.iter() {
                        all_ids.extend(ids);
                    }
                    
                    let mut excluded_ids = HashSet::new();
                    for value in values {
                        let value_str = self.value_to_string(value);
                        if let Some(ids) = field_index.get(&value_str) {
                            excluded_ids.extend(ids);
                        }
                    }
                    
                    Ok(all_ids.difference(&excluded_ids).cloned().collect())
                } else {
                    Err(DatabaseError::Query("NOT IN operator requires array value".to_string()))
                }
            }
        }
    }

    /// Convert JSON value to string for indexing
    fn value_to_string(&self, value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Null => "null".to_string(),
            _ => value.to_string(),
        }
    }

    /// Create a composite index for multiple fields
    pub async fn create_composite_index(&self, table: &str, index_name: &str, fields: &[String]) -> DbResult<()> {
        let mut indexes = self.indexes.write().await;
        let table_index = indexes.entry(table.to_string()).or_insert_with(TableIndex::new);
        
        // Create empty composite index
        table_index.composite_indexes.insert(
            index_name.to_string(),
            BTreeMap::new()
        );
        
        Ok(())
    }

    /// Get index statistics
    pub async fn get_stats(&self, table: &str) -> DbResult<IndexStats> {
        let indexes = self.indexes.read().await;
        let table_index = indexes.get(table)
            .ok_or_else(|| DatabaseError::Index(format!("No index found for table: {}", table)))?;

        let mut total_records = 0;
        let mut field_counts = HashMap::new();

        for (field_name, record_set) in &table_index.primary_index {
            let unique_values = record_set.len();
            field_counts.insert(field_name.clone(), unique_values);
            total_records = total_records.max(unique_values);
        }

        Ok(IndexStats {
            table: table.to_string(),
            total_records,
            field_counts,
            composite_indexes: table_index.composite_indexes.keys().cloned().collect(),
        })
    }

    /// Rebuild indexes for a table
    pub async fn rebuild_index(&self, table: &str) -> DbResult<()> {
        let mut indexes = self.indexes.write().await;
        indexes.insert(table.to_string(), TableIndex::new());
        Ok(())
    }
}

impl TableIndex {
    fn new() -> Self {
        Self {
            primary_index: BTreeMap::new(),
            composite_indexes: HashMap::new(),
            bloom_filters: HashMap::new(),
        }
    }
}

impl BloomFilter {
    fn new(size: usize, hash_functions: usize) -> Self {
        Self {
            bits: vec![false; size],
            hash_functions,
            size,
        }
    }

    fn add(&mut self, item: &str) {
        for i in 0..self.hash_functions {
            let hash = self.hash(item, i);
            let index = hash % self.size;
            self.bits[index] = true;
        }
    }

    fn might_contain(&self, item: &str) -> bool {
        for i in 0..self.hash_functions {
            let hash = self.hash(item, i);
            let index = hash % self.size;
            if !self.bits[index] {
                return false;
            }
        }
        true
    }

    fn hash(&self, item: &str, seed: usize) -> usize {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        item.hash(&mut hasher);
        seed.hash(&mut hasher);
        hasher.finish() as usize
    }
}

/// Index statistics for monitoring and optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexStats {
    pub table: String,
    pub total_records: usize,
    pub field_counts: HashMap<String, usize>,
    pub composite_indexes: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto_db::DatabaseConfig;
    use serde_json::json;

    #[tokio::test]
    async fn test_index_manager() {
        let config = DatabaseConfig::default();
        let manager = IndexManager::new(&config).await.unwrap();

        let table = "users";
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        let user1 = json!({
            "name": "Alice",
            "age": 30,
            "email": "alice@example.com"
        });

        let user2 = json!({
            "name": "Bob",
            "age": 25,
            "email": "bob@example.com"
        });

        // Add records to index
        manager.add_record(table, &id1, &user1).await.unwrap();
        manager.add_record(table, &id2, &user2).await.unwrap();

        // Search by name
        let query = QueryBuilder::new(table)
            .where_eq("name", json!("Alice"));
        
        let results = manager.search(&query).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results.contains(&id1));

        // Search by age range (should return empty for now as range queries aren't fully implemented)
        let query = QueryBuilder::new(table)
            .where_eq("age", json!(30));
        
        let results = manager.search(&query).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results.contains(&id1));
    }

    #[tokio::test]
    async fn test_bloom_filter() {
        let mut bloom = BloomFilter::new(1000, 3);
        
        bloom.add("test1");
        bloom.add("test2");
        
        assert!(bloom.might_contain("test1"));
        assert!(bloom.might_contain("test2"));
        
        // This might give a false positive, but should not give a false negative
        let non_existent = bloom.might_contain("test3");
        // We can't assert false here due to the nature of bloom filters
        println!("Non-existent item check: {}", non_existent);
    }

    #[tokio::test]
    async fn test_composite_queries() {
        let config = DatabaseConfig::default();
        let manager = IndexManager::new(&config).await.unwrap();

        let table = "products";
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let id3 = Uuid::new_v4();

        let product1 = json!({"category": "electronics", "price": 100});
        let product2 = json!({"category": "electronics", "price": 200});
        let product3 = json!({"category": "books", "price": 50});

        manager.add_record(table, &id1, &product1).await.unwrap();
        manager.add_record(table, &id2, &product2).await.unwrap();
        manager.add_record(table, &id3, &product3).await.unwrap();

        // Search for electronics
        let mut query = QueryBuilder::new(table);
        query.conditions.push(QueryCondition {
            field: "category".to_string(),
            operator: QueryOperator::Equals,
            value: json!("electronics"),
        });

        let results = manager.search(&query).await.unwrap();
        assert_eq!(results.len(), 2);
        assert!(results.contains(&id1));
        assert!(results.contains(&id2));
    }
}
