//! Application binding for the shared resume tool.
pub(super) type ResumeAgentTool =
    codetether_collaboration_tools::ResumeAgentTool<super::host::ApplicationHost>;
#[allow(non_upper_case_globals)]
pub(super) const ResumeAgentTool: ResumeAgentTool = ResumeAgentTool::new();
