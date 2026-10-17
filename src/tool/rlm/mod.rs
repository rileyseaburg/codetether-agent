//! Application binding for the dedicated Recursive Language Model tool.
mod ctx;
mod host;
/// RLM tool using the application's existing router and session context.
pub type RlmTool = codetether_rlm_tool::RlmTool<host::ApplicationHost>;
