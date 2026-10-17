//! construct for file comparisons.
use super::*;
/// Compare files or inspect Git changes using the system diff tools.
///
/// # Examples
/// ```rust
/// let _tool = codetether_file_diff::DiffTool::new();
/// ```

/// Diff tool - compare files or show git diff
pub struct DiffTool;

impl Default for DiffTool {
    fn default() -> Self {
        Self::new()
    }
}

impl DiffTool {
    /// Construct the file-difference tool.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_file_diff::DiffTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
