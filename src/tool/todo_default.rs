//! Default construction for todo read and write tools.

use super::{TodoReadTool, TodoWriteTool};

impl Default for TodoReadTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TodoWriteTool {
    fn default() -> Self {
        Self::new()
    }
}
