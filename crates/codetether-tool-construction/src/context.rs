//! Construct context tools for the ordered catalog.
use crate::{Host, NativeTool};
use codetether_tool_core::Tool;
use codetether_tool_registry::catalog::ToolKind;
use std::sync::Arc;
pub(super) fn create(kind: ToolKind, host: &impl Host) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        ToolKind::Lsp => host.create(NativeTool::Lsp),
        ToolKind::ContextReset => {
            Arc::new(codetether_context_reset::ContextResetTool) as Arc<dyn Tool>
        }
        ToolKind::ContextBrowse => host.create(NativeTool::ContextBrowse),
        ToolKind::ContextBudget => host.create(NativeTool::ContextBudget),
        ToolKind::ContextPin => host.create(NativeTool::ContextPin),
        ToolKind::ContextSummarizeCached => host.create(NativeTool::ContextSummarizeCached),
        ToolKind::Skill => Arc::new(codetether_skill_tool::SkillTool::new()) as Arc<dyn Tool>,
        ToolKind::Memory => host.create(NativeTool::Memory),
        _ => return None,
    })
}
