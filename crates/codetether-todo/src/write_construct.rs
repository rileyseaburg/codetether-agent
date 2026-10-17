//! write construct for session todo tools.
use super::*;

impl Default for TodoWriteTool {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoWriteTool {
    /// Construct the tool rooted at the current directory, falling back to dot.
    /// # Examples
    /// ```rust
    /// let _tool = codetether_todo::TodoWriteTool::new();
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
    /// let _tool = codetether_todo::TodoWriteTool::with_root(".".into());
    /// ```
    pub fn with_root(root: PathBuf) -> Self {
        Self { root }
    }

    pub(crate) fn load_todos(&self, params: &Value) -> Result<Vec<TodoItem>> {
        todo_path::load(&self.root, params)
    }

    pub(crate) fn save_todos(&self, params: &Value, todos: &[TodoItem]) -> Result<()> {
        todo_path::save(&self.root, params, todos)
    }

    pub(crate) fn generate_id(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        format!("todo_{}", now.as_millis())
    }
}
