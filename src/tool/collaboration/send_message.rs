//! Application binding for the shared send message tool.
pub(super) type SendMessageTool =
    codetether_collaboration_tools::SendMessageTool<super::host::ApplicationHost>;
#[allow(non_upper_case_globals)]
pub(super) const SendMessageTool: SendMessageTool = SendMessageTool::new();
