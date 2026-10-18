//! Construct search tools for the ordered catalog.
use codetether_tool_core::Tool;
use codetether_tool_registry::catalog::ToolKind;
use std::sync::Arc;
pub(super) fn create(kind: ToolKind) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        ToolKind::Grep => Arc::new(codetether_grep_tool::GrepTool::new()) as Arc<dyn Tool>,
        ToolKind::Ripgrep => Arc::new(codetether_ripgrep_tool::RipgrepTool::new()) as Arc<dyn Tool>,
        ToolKind::CodeSearch => {
            Arc::new(codetether_code_search::CodeSearchTool::new()) as Arc<dyn Tool>
        }
        ToolKind::TokenCount => Arc::new(codetether_token_count::TokenCountTool) as Arc<dyn Tool>,
        _ => return None,
    })
}
