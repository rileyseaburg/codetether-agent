//! write construct for core file tools.
use super::*;
/// Write a file with path validation and telemetry.
///
/// # Examples
/// ```rust
/// let _tool = codetether_file_tools::WriteTool::new();
/// ```

/// Write file contents
pub struct WriteTool;

impl Default for WriteTool {
    fn default() -> Self {
        Self::new()
    }
}

impl WriteTool {
    /// Construct the file tool.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_file_tools::WriteTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
