//! Native verifier/storage specialization for the shared update_goal tool.
pub(super) type UpdateGoalTool = codetether_goal_tools::UpdateGoalTool<super::host::Native>;
#[allow(non_upper_case_globals)]
pub(super) const UpdateGoalTool: UpdateGoalTool = UpdateGoalTool::new();
