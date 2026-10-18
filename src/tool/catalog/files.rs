//! Bind the two file editors to the application Morph backend.
use super::*;
use codetether_tool_construction::NativeTool;
pub(super) fn create(kind: NativeTool) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        NativeTool::Edit => Arc::new(edit::EditTool::new()) as Arc<dyn Tool>,
        NativeTool::MultiEdit => Arc::new(multiedit::MultiEditTool::new()) as Arc<dyn Tool>,
        _ => return None,
    })
}
