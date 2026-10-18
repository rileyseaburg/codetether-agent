//! Native constructors for the goal lifecycle and legacy session-task catalog.
use crate::{Host, VerifierFactory};
use codetether_tool_core::Tool;
use codetether_tool_registry::catalog::ToolKind;
use std::sync::Arc;
/// Construct one tool from the native goal capability group.
/// # Arguments
/// * kind - Catalog identifier requested by the registry.
/// # Returns
/// The matching native tool, or None for identifiers outside this group.
/// # Examples
/// ```no_run
/// # fn example<F: codetether_native_goal_tools::VerifierFactory + 'static>() {
/// let tool = codetether_native_goal_tools::catalog::construct::<F>(
///     codetether_tool_registry::catalog::ToolKind::CreateGoal,
/// );
/// assert!(tool.is_some());
/// # }
/// ```
pub fn construct<F: VerifierFactory + 'static>(kind: ToolKind) -> Option<Arc<dyn Tool>> {
    Some(match kind {
        ToolKind::SessionTask => {
            Arc::new(codetether_session_task::SessionTaskTool::<Host<F>>::new()) as Arc<dyn Tool>
        }
        ToolKind::GetGoal => {
            Arc::new(codetether_goal_tools::GetGoalTool::<Host<F>>::new()) as Arc<dyn Tool>
        }
        ToolKind::CreateGoal => {
            Arc::new(codetether_goal_tools::CreateGoalTool::<Host<F>>::new()) as Arc<dyn Tool>
        }
        ToolKind::UpdateGoal => {
            Arc::new(codetether_goal_tools::UpdateGoalTool::<Host<F>>::new()) as Arc<dyn Tool>
        }
        ToolKind::EditGoal => {
            Arc::new(codetether_goal_tools::EditGoalTool::<Host<F>>::new()) as Arc<dyn Tool>
        }
        _ => return None,
    })
}
