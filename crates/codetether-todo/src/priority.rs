//! priority for session todo tools.
use super::*;
/// Priority of a todo item.
///
/// # Examples
/// ```rust
/// use codetether_todo::Priority;
/// assert!(matches!(Priority::default(), Priority::Medium));
/// ```

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    /// Low urgency.
    Low,
    #[default]
    /// Normal urgency and the default.
    Medium,
    /// High urgency.
    High,
    /// Highest urgency.
    Critical,
}
