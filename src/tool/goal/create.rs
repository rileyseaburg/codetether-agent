//! Native host specialization for the shared create goal tool.
pub(super) type CreateGoalTool = codetether_goal_tools::CreateGoalTool<super::host::Native>;
#[allow(non_upper_case_globals)]
pub(super) const CreateGoalTool: CreateGoalTool = CreateGoalTool::new();
