//! Serializable input and default step count.
use serde::{Deserialize, Serialize};
/// Number of recent commits to undo, or preview before deciding.
///
/// # Examples
/// ```rust
/// let input = codetether_undo_tool::UndoInput { steps: 1, preview: true };
/// assert!(input.preview);
/// ```

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoInput {
    /// Number of commits to undo (default: 1)
    #[serde(default = "default_steps")]
    pub steps: usize,
    /// Show what would be undone without actually doing it
    #[serde(default)]
    pub preview: bool,
}

fn default_steps() -> usize {
    1
}
