//! todoreadtool for session todo tools.
use super::*;
/// Read and filter the calling session’s todo items.
///
/// # Examples
/// ```rust
/// use codetether_todo::TodoReadTool;
/// let _tool = TodoReadTool::with_root(".".into());
/// ```

pub struct TodoReadTool {
    pub(crate) root: PathBuf,
}
