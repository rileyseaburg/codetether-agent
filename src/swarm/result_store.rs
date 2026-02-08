//! Shared result store for sub-agent result sharing
//!
//! This module provides a mechanism for sub-agents to share intermediate results
//! during execution, allowing dependent sub-agents to access results without
//! waiting for the entire swarm to finish.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};

/// A typed result entry in the shared store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedResult {
    /// Unique key for this result
    pub key: String,

    /// The subtask ID that produced this result
    pub producer_id: String,

    /// The result value (JSON)
    pub value: Value,

    /// Schema/type information for the value
    pub schema: ResultSchema,

    /// Timestamp when the result was published
    pub published_at: chrono::DateTime<chrono::Utc>,

    /// Optional expiration time
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Tags for categorization and filtering
    pub tags: Vec<String>,
}

/// Schema information for a shared result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultSchema {
    /// Type name (e.g., "string", "number", "object", "array")
    pub type_name: String,

    /// Optional description of the result
    pub description: Option<String>,

    /// For objects, the expected field types
    pub fields: Option<HashMap<String, String>>,
}

impl ResultSchema {
    /// Create a schema from a serde_json::Value
    pub fn from_value(value: &Value) -> Self {
        let type_name = match value {
            Value::Null => "null".to_string(),
            Value::Bool(_) => "boolean".to_string(),
            Value::Number(n) => {
                if n.is_i64() || n.is_u64() {
                    "integer".to_string()
                } else {
                    "number".to_string()
                }
            }
            Value::String(_) => "string".to_string(),
            Value::Array(_) => "array".to_string(),
            Value::Object(_) => "object".to_string(),
        };

        let fields = if let Value::Object(obj) = value {
            let mut field_types = HashMap::new();
            for (key, val) in obj {
                field_types.insert(key.clone(), Self::from_value(val).type_name);
            }
            Some(field_types)
        } else {
            None
        };

        Self {
            type_name,
            description: None,
            fields,
        }
    }
}

/// Subscription pattern for result notifications
#[derive(Debug, Clone)]
pub enum SubscriptionPattern {
    /// Exact key match
    Exact(String),
    /// Prefix match (key starts with)
    Prefix(String),
    /// Tag match (result has any of these tags)
    Tag(Vec<String>),
    /// Producer match (results from specific subtask)
    Producer(String),
    /// All results
    All,
}

/// Notification sent when a result is published
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultNotification {
    /// The key of the published result
    pub key: String,

    /// The producer subtask ID
    pub producer_id: String,

    /// Tags associated with the result
    pub tags: Vec<String>,
}

/// Shared result store for sub-agent communication
pub struct ResultStore {
    /// All stored results by key
    results: RwLock<HashMap<String, SharedResult>>,

    /// Broadcast channel for result notifications
    notification_tx: broadcast::Sender<ResultNotification>,

    /// Subscriptions by pattern (for efficient filtering)
    subscriptions: RwLock<HashMap<String, Vec<SubscriptionPattern>>>,
}

impl ResultStore {
    /// Create a new result store
    pub fn new() -> Self {
        let (notification_tx, _) = broadcast::channel(1000);
        Self {
            results: RwLock::new(HashMap::new()),
            notification_tx,
            subscriptions: RwLock::new(HashMap::new()),
        }
    }

    /// Create a new result store wrapped in an Arc
    pub fn new_arc() -> Arc<Self> {
        Arc::new(Self::new())
    }

    /// Publish a result to the store
    pub async fn publish(
        &self,
        key: impl Into<String>,
        producer_id: impl Into<String>,
        value: impl Serialize,
        tags: Vec<String>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<SharedResult> {
        let key = key.into();
        let producer_id = producer_id.into();

        let value = serde_json::to_value(value)?;
        let schema = ResultSchema::from_value(&value);

        let result = SharedResult {
            key: key.clone(),
            producer_id: producer_id.clone(),
            value,
            schema,
            published_at: chrono::Utc::now(),
            expires_at,
            tags: tags.clone(),
        };

        // Store the result
        {
            let mut results = self.results.write().await;
            results.insert(key.clone(), result.clone());
        }

        // Notify subscribers
        let notification = ResultNotification {
            key: key.clone(),
            producer_id,
            tags,
        };

        // Broadcast to all listeners (they filter based on their subscription)
        let _ = self.notification_tx.send(notification);

        tracing::info!(key = %key, "Published shared result");

        Ok(result)
    }

    /// Get a result by key
    pub async fn get(&self, key: &str) -> Option<SharedResult> {
        let results = self.results.read().await;
        results.get(key).cloned()
    }

    /// Get a result and deserialize it to a specific type
    pub async fn get_typed<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<T> {
        let result = self
            .get(key)
            .await
            .ok_or_else(|| anyhow!("Result not found: {}", key))?;

        serde_json::from_value(result.value)
            .map_err(|e| anyhow!("Failed to deserialize result: {}", e))
    }

    /// Query results by tags
    pub async fn query_by_tags(&self, tags: &[String]) -> Vec<SharedResult> {
        let results = self.results.read().await;
        results
            .values()
            .filter(|r| tags.iter().any(|t| r.tags.contains(t)))
            .cloned()
            .collect()
    }

    /// Query results by producer
    pub async fn query_by_producer(&self, producer_id: &str) -> Vec<SharedResult> {
        let results = self.results.read().await;
        results
            .values()
            .filter(|r| r.producer_id == producer_id)
            .cloned()
            .collect()
    }

    /// Query results by key prefix
    pub async fn query_by_prefix(&self, prefix: &str) -> Vec<SharedResult> {
        let results = self.results.read().await;
        results
            .values()
            .filter(|r| r.key.starts_with(prefix))
            .cloned()
            .collect()
    }

    /// Subscribe to result notifications
    pub fn subscribe(&self) -> broadcast::Receiver<ResultNotification> {
        self.notification_tx.subscribe()
    }

    /// Register a subscription pattern for a subtask
    pub async fn register_subscription(
        &self,
        subtask_id: impl Into<String>,
        pattern: SubscriptionPattern,
    ) {
        let subtask_id = subtask_id.into();
        let mut subscriptions = self.subscriptions.write().await;
        subscriptions
            .entry(subtask_id)
            .or_default()
            .push(pattern);
    }

    /// Unregister all subscriptions for a subtask
    pub async fn unregister_subscriptions(&self, subtask_id: &str) {
        let mut subscriptions = self.subscriptions.write().await;
        subscriptions.remove(subtask_id);
    }

    /// Check if a result matches a subscription pattern
    pub fn matches_pattern(result: &SharedResult, pattern: &SubscriptionPattern) -> bool {
        match pattern {
            SubscriptionPattern::Exact(key) => result.key == *key,
            SubscriptionPattern::Prefix(prefix) => result.key.starts_with(prefix),
            SubscriptionPattern::Tag(tags) => tags.iter().any(|t| result.tags.contains(t)),
            SubscriptionPattern::Producer(producer) => result.producer_id == *producer,
            SubscriptionPattern::All => true,
        }
    }

    /// Get all results (for debugging/inspection)
    pub async fn get_all(&self) -> Vec<SharedResult> {
        let results = self.results.read().await;
        results.values().cloned().collect()
    }

    /// Remove expired results
    pub async fn cleanup_expired(&self) -> usize {
        let now = chrono::Utc::now();
        let mut results = self.results.write().await;
        let keys_to_remove: Vec<String> = results
            .values()
            .filter(|r| r.expires_at.map(|exp| exp <= now).unwrap_or(false))
            .map(|r| r.key.clone())
            .collect();

        for key in &keys_to_remove {
            results.remove(key);
        }

        keys_to_remove.len()
    }

    /// Clear all results
    pub async fn clear(&self) {
        let mut results = self.results.write().await;
        results.clear();
    }
}

impl Default for ResultStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait for SubTaskContext to integrate with ResultStore
pub trait ResultStoreContext {
    /// Publish a result to the shared store
    fn publish_result(
        &self,
        key: impl Into<String>,
        value: impl Serialize,
        tags: Vec<String>,
    ) -> impl std::future::Future<Output = Result<SharedResult>> + Send;

    /// Get a result from the shared store
    fn get_result(&self, key: &str) -> impl std::future::Future<Output = Option<SharedResult>> + Send;

    /// Get a typed result from the shared store
    fn get_result_typed<T: for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> impl std::future::Future<Output = Result<T>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_publish_and_get() {
        let store = ResultStore::new();

        store
            .publish("test-key", "task-1", "hello world", vec![], None)
            .await
            .unwrap();

        let result = store.get("test-key").await.unwrap();
        assert_eq!(result.key, "test-key");
        assert_eq!(result.producer_id, "task-1");
        assert_eq!(result.value, Value::String("hello world".to_string()));
    }

    #[tokio::test]
    async fn test_get_typed() {
        let store = ResultStore::new();

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct TestData {
            name: String,
            count: i32,
        }

        let data = TestData {
            name: "test".to_string(),
            count: 42,
        };

        store
            .publish("typed-key", "task-1", &data, vec![], None)
            .await
            .unwrap();

        let retrieved: TestData = store.get_typed("typed-key").await.unwrap();
        assert_eq!(retrieved, data);
    }

    #[tokio::test]
    async fn test_query_by_tags() {
        let store = ResultStore::new();

        store
            .publish("key-1", "task-1", "value-1", vec!["tag-a".to_string()], None)
            .await
            .unwrap();

        store
            .publish("key-2", "task-2", "value-2", vec!["tag-b".to_string()], None)
            .await
            .unwrap();

        store
            .publish("key-3", "task-1", "value-3", vec!["tag-a".to_string(), "tag-c".to_string()], None)
            .await
            .unwrap();

        let results = store.query_by_tags(&["tag-a".to_string()]).await;
        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn test_query_by_prefix() {
        let store = ResultStore::new();

        store
            .publish("prefix/key-1", "task-1", "value-1", vec![], None)
            .await
            .unwrap();

        store
            .publish("prefix/key-2", "task-2", "value-2", vec![], None)
            .await
            .unwrap();

        store
            .publish("other/key-3", "task-1", "value-3", vec![], None)
            .await
            .unwrap();

        let results = store.query_by_prefix("prefix/").await;
        assert_eq!(results.len(), 2);
    }

    #[tokio::test]
    async fn test_subscription_notifications() {
        let store = ResultStore::new();
        let mut rx = store.subscribe();

        store
            .publish("notify-key", "task-1", "value", vec!["tag-1".to_string()], None)
            .await
            .unwrap();

        let notification = rx.try_recv().unwrap();
        assert_eq!(notification.key, "notify-key");
        assert_eq!(notification.producer_id, "task-1");
    }

    #[tokio::test]
    async fn test_matches_pattern() {
        let result = SharedResult {
            key: "test/key".to_string(),
            producer_id: "task-1".to_string(),
            value: Value::Null,
            schema: ResultSchema::from_value(&Value::Null),
            published_at: chrono::Utc::now(),
            expires_at: None,
            tags: vec!["tag-a".to_string()],
        };

        assert!(ResultStore::matches_pattern(&result, &SubscriptionPattern::Exact("test/key".to_string())));
        assert!(!ResultStore::matches_pattern(&result, &SubscriptionPattern::Exact("other".to_string())));
        assert!(ResultStore::matches_pattern(&result, &SubscriptionPattern::Prefix("test/".to_string())));
        assert!(ResultStore::matches_pattern(&result, &SubscriptionPattern::Tag(vec!["tag-a".to_string()])));
        assert!(ResultStore::matches_pattern(&result, &SubscriptionPattern::Producer("task-1".to_string())));
        assert!(ResultStore::matches_pattern(&result, &SubscriptionPattern::All));
    }
}
