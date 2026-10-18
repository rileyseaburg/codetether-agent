//! Native host specialization for the shared edit goal tool.
pub(super) type EditGoalTool = codetether_goal_tools::EditGoalTool<super::host::Native>;
#[allow(non_upper_case_globals)]
pub(super) const EditGoalTool: EditGoalTool = EditGoalTool::new();
