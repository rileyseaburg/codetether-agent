//! Native execution tool constructors.
use super::*;
pub(super) fn create(kind: ToolKind) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        ToolKind::TodoRead => Arc::new(todo::TodoReadTool::new()) as Arc<dyn Tool>,
        ToolKind::TodoWrite => Arc::new(todo::TodoWriteTool::new()) as Arc<dyn Tool>,
        ToolKind::Question => Arc::new(question::QuestionTool::new()) as Arc<dyn Tool>,
        ToolKind::Task => Arc::new(task::TaskTool::new()) as Arc<dyn Tool>,
        ToolKind::PlanEnter => Arc::new(plan::PlanEnterTool::new()) as Arc<dyn Tool>,
        ToolKind::PlanExit => Arc::new(plan::PlanExitTool::new()) as Arc<dyn Tool>,
        ToolKind::RalphDefault => Arc::new(ralph::RalphTool::new()) as Arc<dyn Tool>,
        ToolKind::Prd => Arc::new(prd::PrdTool::new()) as Arc<dyn Tool>,
        ToolKind::Undo => Arc::new(undo::UndoTool) as Arc<dyn Tool>,
        ToolKind::SwarmShare => {
            Arc::new(swarm_share::SwarmShareTool::with_defaults()) as Arc<dyn Tool>
        }
        ToolKind::Invalid => Arc::new(invalid::InvalidTool::new()) as Arc<dyn Tool>,
        ToolKind::SwarmExecute => Arc::new(swarm_execute::SwarmExecuteTool::new()) as Arc<dyn Tool>,
        ToolKind::RelayAutoChat => {
            Arc::new(relay_autochat::RelayAutoChatTool::new()) as Arc<dyn Tool>
        }
        ToolKind::Go => Arc::new(go::GoTool::new()) as Arc<dyn Tool>,
        _ => return None,
    })
}
