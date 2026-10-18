//! File catalog binding for application-owned editor backends.
use super::*;
pub(super) fn create(kind: ToolKind) -> Option<Arc<dyn Tool>> {
    codetether_file_tool_catalog::create(
        kind,
        || Arc::new(edit::EditTool::new()) as Arc<dyn Tool>,
        || Arc::new(multiedit::MultiEditTool::new()) as Arc<dyn Tool>,
    )
}
