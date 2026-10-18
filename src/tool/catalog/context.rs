//! Native context tool constructors.
use super::*;
use codetether_tool_construction::NativeTool;
pub(super) fn create(kind: NativeTool) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        NativeTool::Lsp => Arc::new(lsp::LspTool::with_root(
            std::env::current_dir()
                .map(|p| format!("file://{}", p.display()))
                .unwrap_or_default(),
        )) as Arc<dyn Tool>,
        NativeTool::ContextBrowse => Arc::new(context_browse::ContextBrowseTool) as Arc<dyn Tool>,
        NativeTool::ContextBudget => Arc::new(context_budget::ContextBudgetTool) as Arc<dyn Tool>,
        NativeTool::ContextPin => Arc::new(context_pin::ContextPinTool) as Arc<dyn Tool>,
        NativeTool::ContextSummarizeCached => {
            Arc::new(context_summarize::ContextSummarizeTool::cached_only()) as Arc<dyn Tool>
        }
        NativeTool::Memory => Arc::new(memory::MemoryTool::new()) as Arc<dyn Tool>,
        _ => return None,
    })
}
