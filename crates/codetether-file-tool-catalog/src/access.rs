//! Construct file access, inspection, diff and Git tools.
use codetether_tool_core::Tool;
use codetether_tool_registry::catalog::ToolKind;
use std::sync::Arc;
pub(super) fn create(kind: ToolKind) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        ToolKind::Read => Arc::new(codetether_file_tools::ReadTool::new()) as Arc<dyn Tool>,
        ToolKind::Write => Arc::new(codetether_file_tools::WriteTool::new()) as Arc<dyn Tool>,
        ToolKind::List => Arc::new(codetether_file_tools::ListTool::new()) as Arc<dyn Tool>,
        ToolKind::Glob => Arc::new(codetether_file_tools::GlobTool::new()) as Arc<dyn Tool>,
        ToolKind::Tree => Arc::new(codetether_file_inspect::TreeTool::new()) as Arc<dyn Tool>,
        ToolKind::FileInfo => {
            Arc::new(codetether_file_inspect::FileInfoTool::new()) as Arc<dyn Tool>
        }
        ToolKind::HeadTail => {
            Arc::new(codetether_file_inspect::HeadTailTool::new()) as Arc<dyn Tool>
        }
        ToolKind::Diff => Arc::new(codetether_file_diff::DiffTool::new()) as Arc<dyn Tool>,
        ToolKind::Git => Arc::new(codetether_git_tool::GitTool::new()) as Arc<dyn Tool>,
        _ => return None,
    })
}
