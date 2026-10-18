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
        ToolKind::SessionTask => Arc::new(session_task::SessionTaskTool::new()) as Arc<dyn Tool>,
        ToolKind::GetGoal => Arc::new(get::GetGoalTool) as Arc<dyn Tool>,
        ToolKind::CreateGoal => Arc::new(create::CreateGoalTool) as Arc<dyn Tool>,
        ToolKind::UpdateGoal => Arc::new(update::UpdateGoalTool) as Arc<dyn Tool>,
        ToolKind::EditGoal => Arc::new(edit::EditGoalTool) as Arc<dyn Tool>,
        _ => return None,
    })
}
