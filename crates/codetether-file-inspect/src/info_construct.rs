//! info construct for filesystem inspection.
/// Filesystem inspection tool.
///
/// # Examples
/// ```rust
/// let _tool = codetether_file_inspect::FileInfoTool::new();
/// ```

/// File information tool - get metadata about a file
pub struct FileInfoTool;

impl Default for FileInfoTool {
    fn default() -> Self {
        Self::new()
    }
}

impl FileInfoTool {
    /// Construct this inspection tool.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_file_inspect::FileInfoTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
