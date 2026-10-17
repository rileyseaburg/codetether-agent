//! construct for the question tool.
use super::*;

impl Default for QuestionTool {
    fn default() -> Self {
        Self::new()
    }
}

impl QuestionTool {
    /// Construct the stateless runtime adapter.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_question_tool::QuestionTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
