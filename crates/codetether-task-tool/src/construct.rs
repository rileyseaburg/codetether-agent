//! construct for in-memory task tracking.
use super::*;

impl Default for TaskTool {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskTool {
    /// Construct a tool sharing the process task registry.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_task_tool::TaskTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
