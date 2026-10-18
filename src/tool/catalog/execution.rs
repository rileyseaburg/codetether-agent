//! Native execution tool constructors.
use super::*;
use codetether_tool_construction::NativeTool;
pub(super) fn create(kind: NativeTool) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        NativeTool::RalphDefault => Arc::new(ralph::RalphTool::new()) as Arc<dyn Tool>,
        NativeTool::Undo => Arc::new(undo::UndoTool) as Arc<dyn Tool>,
        NativeTool::SwarmExecute => {
            Arc::new(swarm_execute::SwarmExecuteTool::new()) as Arc<dyn Tool>
        }
        NativeTool::Go => Arc::new(go::GoTool::new()) as Arc<dyn Tool>,
        _ => return None,
    })
}
