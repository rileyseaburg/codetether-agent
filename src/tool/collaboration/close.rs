//! Application binding for the shared close tool.
pub(super) type CloseAgentTool =
    codetether_collaboration_tools::CloseAgentTool<super::host::ApplicationHost>;
#[allow(non_upper_case_globals)]
pub(super) const CloseAgentTool: CloseAgentTool = CloseAgentTool::new();
