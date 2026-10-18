//! Construct execution tools for the ordered catalog.
use crate::{Host, NativeTool};
use codetether_tool_core::Tool;
use codetether_tool_registry::catalog::ToolKind;
use std::sync::Arc;
pub(super) fn create(kind: ToolKind, host: &impl Host) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        ToolKind::TodoRead => Arc::new(codetether_todo::TodoReadTool::new()) as Arc<dyn Tool>,
        ToolKind::TodoWrite => Arc::new(codetether_todo::TodoWriteTool::new()) as Arc<dyn Tool>,
        ToolKind::Question => {
            Arc::new(codetether_question_tool::QuestionTool::new()) as Arc<dyn Tool>
        }
        ToolKind::Task => Arc::new(codetether_task_tool::TaskTool::new()) as Arc<dyn Tool>,
        ToolKind::PlanEnter => Arc::new(codetether_plan::PlanEnterTool::new()) as Arc<dyn Tool>,
        ToolKind::PlanExit => Arc::new(codetether_plan::PlanExitTool::new()) as Arc<dyn Tool>,
        ToolKind::RalphDefault => host.create(NativeTool::RalphDefault),
        ToolKind::Prd => Arc::new(codetether_prd_tool::PrdTool::new()) as Arc<dyn Tool>,
        ToolKind::Undo => host.create(NativeTool::Undo),
        ToolKind::SwarmShare => {
            Arc::new(codetether_swarm_share::SwarmShareTool::with_defaults()) as Arc<dyn Tool>
        }
        ToolKind::Invalid => Arc::new(codetether_tool_core::InvalidTool::new()) as Arc<dyn Tool>,
        ToolKind::SwarmExecute => host.create(NativeTool::SwarmExecute),
        ToolKind::RelayAutoChat => {
            Arc::new(codetether_relay_tool::RelayAutoChatTool::new()) as Arc<dyn Tool>
        }
        ToolKind::Go => host.create(NativeTool::Go),
        _ => return None,
    })
}
