//! glob construct for core file tools.
use super::*;
/// Find paths matching a root-relative glob pattern.
///
/// # Examples
/// ```rust
/// let _tool = codetether_file_tools::GlobTool::new();
/// ```

/// Find files using glob patterns
pub struct GlobTool {
    pub(crate) root: PathBuf,
}

impl Default for GlobTool {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobTool {
    /// Construct the file tool.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_file_tools::GlobTool::new();
    /// ```
    pub fn new() -> Self {
        Self {
            root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    /// Construct glob search rooted at the supplied directory.
    /// # Arguments
    /// * `root` - Root used to resolve relative patterns and display matches.
    /// # Returns
    /// A tool bound to that root.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_file_tools::GlobTool::with_root(".".into());
    /// ```
    pub fn with_root(root: PathBuf) -> Self {
        Self { root }
    }
}
