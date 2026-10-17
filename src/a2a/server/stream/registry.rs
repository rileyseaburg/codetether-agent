//! Load providers or mark the stream task failed.
use super::*;
pub(super) async fn load(
    tasks: &DashMap<String, Task>,
    task_id: &str,
) -> Option<Arc<crate::provider::ProviderRegistry>> {
    match crate::provider::ProviderRegistry::shared_from_vault().await {
        Ok(r) => Some(r),
        Err(e) => {
            tracing::error!(target: "codetether_agent::a2a::server", "Failed to load provider registry: {}", e);
            if let Some(mut t) = tasks.get_mut(task_id) {
                t.status.state = TaskState::Failed;
                t.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
            }
            None
        }
    }
}
