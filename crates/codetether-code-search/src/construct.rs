//! construct for workspace code search.
use super::*;
/// Search workspace code with context and bounded results.
///
/// # Examples
/// ```rust
/// let _tool = codetether_code_search::CodeSearchTool::with_root(".".into());
/// ```

pub struct CodeSearchTool {
    pub(crate) root: PathBuf,
}

impl Default for CodeSearchTool {
    fn default() -> Self {
        Self::new()
    }
}
impl CodeSearchTool {
    /// Construct code search rooted at the current directory.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_code_search::CodeSearchTool::new();
    /// ```
    pub fn new() -> Self {
        Self {
            root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    /// Construct code search for an explicit root.
    /// # Arguments
    /// * `root` - Workspace root for traversal and relative output paths.
    /// # Returns
    /// A search tool bound to that root.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_code_search::CodeSearchTool::with_root(".".into());
    /// ```
    pub fn with_root(root: PathBuf) -> Self {
        Self { root }
    }
}
