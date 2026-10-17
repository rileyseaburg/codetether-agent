//! enter construct for the planning tools.
use super::*;
/// Enter planning mode with an ordered list of steps.
///
/// # Examples
/// ```rust
/// let _tool = codetether_plan::PlanEnterTool::new();
/// ```

pub struct PlanEnterTool;

impl Default for PlanEnterTool {
    fn default() -> Self {
        Self::new()
    }
}

impl PlanEnterTool {
    /// Construct the planning tool.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_plan::PlanEnterTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
