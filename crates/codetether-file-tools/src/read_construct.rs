//! read construct for core file tools.
/// Read a file with line selection and telemetry.
///
/// # Examples
/// ```rust
/// let _tool = codetether_file_tools::ReadTool::new();
/// ```

/// Read file contents
pub struct ReadTool;

impl Default for ReadTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadTool {
    /// Construct the file tool.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_file_tools::ReadTool::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}
