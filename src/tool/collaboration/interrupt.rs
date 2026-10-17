//! Application binding for the shared interrupt tool.
pub(super) type InterruptAgentTool =
    codetether_collaboration_tools::InterruptAgentTool<super::host::ApplicationHost>;
#[allow(non_upper_case_globals)]
pub(super) const InterruptAgentTool: InterruptAgentTool = InterruptAgentTool::new();
