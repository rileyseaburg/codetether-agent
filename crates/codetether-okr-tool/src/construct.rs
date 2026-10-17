//! construct for OKR tool dispatch.
use super::*;
/// Tool adapter for persistent objectives, key results and execution runs.
///
/// # Examples
/// ```rust
/// let _tool = codetether_okr_tool::OkrTool::new();
/// ```

pub struct OkrTool;

impl Default for OkrTool {
    fn default() -> Self {
        Self::new()
    }
}

impl OkrTool {
    /// Construct the OKR tool without opening a repository.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_okr_tool::OkrTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
