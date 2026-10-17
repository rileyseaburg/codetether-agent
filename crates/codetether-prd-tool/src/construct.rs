//! construct for PRD construction and validation.
use super::*;

impl Default for PrdTool {
    fn default() -> Self {
        Self::new()
    }
}

impl PrdTool {
    /// Construct the stateless PRD tool.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_prd_tool::PrdTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
