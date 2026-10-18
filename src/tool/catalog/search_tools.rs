//! Native search tool construction.
use super::*;
pub(super) fn create(kind: ToolKind) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        ToolKind::Grep => Arc::new(search::GrepTool::new()) as Arc<dyn Tool>,
        ToolKind::Ripgrep => Arc::new(ripgrep::RipgrepTool::new()) as Arc<dyn Tool>,
        ToolKind::CodeSearch => Arc::new(codesearch::CodeSearchTool::new()) as Arc<dyn Tool>,
        ToolKind::TokenCount => Arc::new(token_count::TokenCountTool) as Arc<dyn Tool>,
        _ => return None,
    })
}
