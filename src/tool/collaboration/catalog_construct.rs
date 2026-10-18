//! Native constructors for shared catalog capability identifiers.
use super::*;
use crate::tool::Tool;
use std::sync::Arc;

/// Construct one tool from this native capability group.
pub(crate) fn construct(
    kind: codetether_tool_registry::catalog::ToolKind,
) -> Option<Arc<dyn Tool>> {
    use codetether_tool_registry::catalog::ToolKind;
    Some(match kind {
        ToolKind::Agent => Arc::new(crate::tool::agent::AgentTool::new()) as Arc<dyn Tool>,
        ToolKind::SpawnAgent => Arc::new(spawn::SpawnAgentTool) as Arc<dyn Tool>,
        ToolKind::FollowupTask => Arc::new(followup::FollowupTaskTool) as Arc<dyn Tool>,
        ToolKind::ListAgents => Arc::new(list::ListAgentsTool) as Arc<dyn Tool>,
        ToolKind::WaitAgent => Arc::new(wait::WaitAgentTool) as Arc<dyn Tool>,
        ToolKind::InterruptAgent => Arc::new(interrupt::InterruptAgentTool) as Arc<dyn Tool>,
        ToolKind::CloseAgent => Arc::new(close::CloseAgentTool) as Arc<dyn Tool>,
        ToolKind::ResumeAgent => Arc::new(resume::ResumeAgentTool) as Arc<dyn Tool>,
        ToolKind::SendInput => Arc::new(send_input::SendInputTool) as Arc<dyn Tool>,
        ToolKind::SendMessage => Arc::new(send_message::SendMessageTool) as Arc<dyn Tool>,
        _ => return None,
    })
}
