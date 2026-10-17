//! todostatus for session todo tools.
use super::*;
/// Lifecycle state of a todo item.
///
/// # Examples
/// ```rust
/// use codetether_todo::TodoStatus;
/// assert!(matches!(TodoStatus::default(), TodoStatus::Pending));
/// ```

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum TodoStatus {
    #[default]
    /// Not yet started.
    Pending,
    #[serde(alias = "in_progress")]
    #[serde(alias = "inprogress")]
    /// Currently being worked on.
    InProgress,
    /// Completed.
    Done,
    /// Waiting on an external dependency.
    Blocked,
}
