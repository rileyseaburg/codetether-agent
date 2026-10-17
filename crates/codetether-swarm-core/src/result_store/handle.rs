use super::*;

/// Handle for a sub-agent to interact with the shared result store
///
/// # Examples
/// ```
/// use codetether_swarm_core::result_store::{ResultStore, SubTaskStoreHandle};
/// let handle = SubTaskStoreHandle::new(ResultStore::new_arc(), "task");
/// ```
pub struct SubTaskStoreHandle {
    store: Arc<ResultStore>,
    subtask_id: String,
}

impl SubTaskStoreHandle {
    /// Create a new handle for a specific subtask
    pub fn new(store: Arc<ResultStore>, subtask_id: impl Into<String>) -> Self {
        Self {
            store,
            subtask_id: subtask_id.into(),
        }
    }
}

impl ResultStoreContext for SubTaskStoreHandle {
    async fn publish_result(
        &self,
        key: impl Into<String> + Send,
        value: impl Serialize + Send,
        tags: Vec<String>,
    ) -> Result<SharedResult> {
        self.store
            .publish(key, &self.subtask_id, value, tags, None)
            .await
    }

    async fn get_result(&self, key: &str) -> Option<SharedResult> {
        self.store.get(key).await
    }

    async fn get_result_typed<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<T> {
        self.store.get_typed(key).await
    }
}
