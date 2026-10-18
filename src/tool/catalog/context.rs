//! Native context tool constructors.
use super::*;
pub(super) fn create(kind: ToolKind) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        ToolKind::Lsp => Arc::new(lsp::LspTool::with_root(
            std::env::current_dir()
                .map(|p| format!("file://{}", p.display()))
                .unwrap_or_default(),
        )) as Arc<dyn Tool>,
        ToolKind::ContextReset => Arc::new(context_reset::ContextResetTool) as Arc<dyn Tool>,
        ToolKind::ContextBrowse => Arc::new(context_browse::ContextBrowseTool) as Arc<dyn Tool>,
        ToolKind::ContextBudget => Arc::new(context_budget::ContextBudgetTool) as Arc<dyn Tool>,
        ToolKind::ContextPin => Arc::new(context_pin::ContextPinTool) as Arc<dyn Tool>,
        ToolKind::ContextSummarizeCached => {
            Arc::new(context_summarize::ContextSummarizeTool::cached_only()) as Arc<dyn Tool>
        }
        ToolKind::Skill => Arc::new(skill::SkillTool::new()) as Arc<dyn Tool>,
        ToolKind::Memory => Arc::new(memory::MemoryTool::new()) as Arc<dyn Tool>,
        _ => return None,
    })
}
