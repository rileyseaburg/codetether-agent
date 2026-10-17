//! read construct for session todo tools.
use super::*;

impl Default for TodoReadTool {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoReadTool {
    /// Construct the tool rooted at the current directory, falling back to dot.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_todo::TodoReadTool::new();
    /// ```
    pub fn new() -> Self {
        Self {
            root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    /// Construct the tool with an explicit checkout root.
    /// # Arguments
    /// * `root` - Fallback directory when a session-specific path is unavailable.
    /// # Returns
    /// A tool using the supplied root.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_todo::TodoReadTool::with_root(".".into());
    /// ```
    pub fn with_root(root: PathBuf) -> Self {
        Self { root }
    }

    pub(crate) fn load_todos(&self, params: &Value) -> Result<Vec<TodoItem>> {
        todo_path::load(&self.root, params)
    }
}
