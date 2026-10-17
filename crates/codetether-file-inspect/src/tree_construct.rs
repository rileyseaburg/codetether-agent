//! tree construct for filesystem inspection.
use super::*;
/// Filesystem inspection tool.
///
/// # Examples
/// ```rust
/// let _tool = codetether_file_inspect::TreeTool::new();
/// ```

/// Tree view of directory structure
pub struct TreeTool;

impl Default for TreeTool {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeTool {
    /// Construct this inspection tool.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_file_inspect::TreeTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
