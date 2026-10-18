//! Application specialization of the shared agent-management tool.
#[path = "tool_host.rs"]
mod host;
pub(super) use host::ApplicationHost;
/// Native agent-management tool retaining the existing public constructor.
pub type AgentTool = codetether_agent_tool::AgentTool<ApplicationHost>;
/// Compatibility value for existing unit-style tool construction.
#[allow(non_upper_case_globals)]
pub const AgentTool: AgentTool = AgentTool::new();
