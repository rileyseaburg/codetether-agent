//! todowritetool for session todo tools.
use super::*;
/// Add, update, delete or clear the calling session’s todo items.
///
/// # Examples
/// ```rust
/// use codetether_todo::TodoWriteTool;
/// let _tool = TodoWriteTool::with_root(".".into());
/// ```

pub struct TodoWriteTool {
    pub(crate) root: PathBuf,
}
