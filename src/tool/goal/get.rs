//! Native host specialization for the shared get goal tool.
pub(super) type GetGoalTool = codetether_goal_tools::GetGoalTool<super::host::Native>;
#[allow(non_upper_case_globals)]
pub(super) const GetGoalTool: GetGoalTool = GetGoalTool::new();
