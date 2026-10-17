//! SubTaskContext execution contract.
use super::*;

/// Context passed to a subtask
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let context = codetether_swarm_core::SubTaskContext::default();
/// assert!(context.dependency_results.is_empty());
/// ```
pub struct SubTaskContext {
    /// Parent task information
    pub parent_task: Option<String>,

    /// Results from dependency subtasks
    pub dependency_results: HashMap<String, String>,

    /// Shared files or resources
    pub shared_resources: Vec<String>,

    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}
