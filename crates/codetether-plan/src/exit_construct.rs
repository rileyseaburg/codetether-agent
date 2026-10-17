//! exit construct for the planning tools.
use super::*;
/// Report plan progress and exit when complete or summarized.
///
/// # Examples
/// ```rust
/// let _tool = codetether_plan::PlanExitTool::new();
/// ```

pub struct PlanExitTool;

impl Default for PlanExitTool {
    fn default() -> Self {
        Self::new()
    }
}

impl PlanExitTool {
    /// Construct the planning tool.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_plan::PlanExitTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
