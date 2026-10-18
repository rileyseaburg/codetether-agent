//! construct for bounded grep searches.
/// Search local text files with bounded time, file size and result counts.
///
/// # Examples
/// ```rust
/// let _tool = codetether_grep_tool::GrepTool::new();
/// ```

/// Search for text in files
pub struct GrepTool;

impl Default for GrepTool {
    fn default() -> Self {
        Self::new()
    }
}

impl GrepTool {
    /// Construct the grep tool.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_grep_tool::GrepTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
