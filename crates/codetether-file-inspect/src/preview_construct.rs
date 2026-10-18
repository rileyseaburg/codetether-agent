//! preview construct for filesystem inspection.
/// Filesystem inspection tool.
///
/// # Examples
/// ```rust
/// let _tool = codetether_file_inspect::HeadTailTool::new();
/// ```

/// Head/Tail tool - quickly peek at file beginning or end
pub struct HeadTailTool;

impl Default for HeadTailTool {
    fn default() -> Self {
        Self::new()
    }
}

impl HeadTailTool {
    /// Construct this inspection tool.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_file_inspect::HeadTailTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
