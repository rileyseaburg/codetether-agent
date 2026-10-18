//! Application binding for the dedicated Ralph lifecycle tool.
pub(super) mod host;
/// Ralph tool using the application's runtime and checkout policy.
pub type RalphTool = codetether_ralph_tool::RalphTool<host::ApplicationHost>;
