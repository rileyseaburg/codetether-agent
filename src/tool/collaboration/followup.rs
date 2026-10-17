//! Application binding for the shared followup tool.
pub(super) type FollowupTaskTool =
    codetether_collaboration_tools::FollowupTaskTool<super::host::ApplicationHost>;
#[allow(non_upper_case_globals)]
pub(super) const FollowupTaskTool: FollowupTaskTool = FollowupTaskTool::new();
