//! list construct for core file tools.
use super::*;
/// List names and entry types in a directory.
///
/// # Examples
/// ```rust
/// let _tool = codetether_file_tools::ListTool::new();
/// ```

/// List directory contents
pub struct ListTool;

impl Default for ListTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ListTool {
    /// Construct the file tool.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_file_tools::ListTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
