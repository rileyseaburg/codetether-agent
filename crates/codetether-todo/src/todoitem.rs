//! todoitem for session todo tools.
use super::*;
/// One persisted todo item.
///
/// # Examples
/// ```rust
/// use codetether_todo::TodoItem;
/// let item = TodoItem { id: "t1".into(), content: "review".into(), status: codetether_todo::TodoStatus::Pending, priority: codetether_todo::Priority::Medium, created_at: None };
/// assert_eq!(item.id, "t1");
/// ```

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    /// Stable identifier generated when the item is added.
    pub id: String,
    /// Task description shown to the agent.
    pub content: String,
    /// Current task state.
    pub status: TodoStatus,
    #[serde(default)]
    /// Scheduling priority.
    pub priority: Priority,
    #[serde(default)]
    /// Creation timestamp, when recorded.
    pub created_at: Option<String>,
}
