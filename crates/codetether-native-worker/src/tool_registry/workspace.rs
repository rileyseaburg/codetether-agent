//! Application workspace tool specializations.
use super::Native;
use codetether_native_edit::{multiedit, undo};
use codetether_native_lsp::tool as lsp;
use codetether_tool_core::Tool;
use std::sync::Arc;
impl<A, W> codetether_worker_tools::Workspace for Native<A, W> {
    fn lsp(root: String) -> Arc<dyn Tool> {
        Arc::new(lsp::LspTool::with_root(root))
    }
    fn multiedit() -> Arc<dyn Tool> {
        Arc::new(multiedit::MultiEditTool::new())
    }
    fn undo() -> Arc<dyn Tool> {
        Arc::new(undo::UndoTool)
    }
}
