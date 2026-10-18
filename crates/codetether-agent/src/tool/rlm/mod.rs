//! Application binding for the dedicated Recursive Language Model tool.
/// RLM tool using the application's existing router and session context.
pub type RlmTool = codetether_rlm_tool::RlmTool<
    codetether_rlm_tool::application::ApplicationHost<crate::session::SessionBus>,
>;
