//! Application workspace tool specializations.
use super::Native;
use crate::tool::{Tool, lsp, multiedit, undo};
use std::sync::Arc;
impl codetether_worker_tools::Workspace for Native {
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
