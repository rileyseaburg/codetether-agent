use super::*;

/// A typed result entry in the shared store
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```no_run
/// # async fn example(store: &codetether_swarm_core::result_store::ResultStore) -> anyhow::Result<()> {
/// let result = store.publish("key", "task", "value", vec![], None).await?;
/// assert_eq!(result.key, "key");
/// # Ok(())
/// # }
/// ```
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

/// Notification sent when a result is published
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let notification = codetether_swarm_core::result_store::ResultNotification { key: "result".into(), producer_id: "task".into(), tags: vec![] };
/// assert_eq!(notification.producer_id, "task");
/// ```
pub struct ResultNotification {
    /// The key of the published result
    pub key: String,

    /// The producer subtask ID
    pub producer_id: String,

    /// Tags associated with the result
    pub tags: Vec<String>,
}
