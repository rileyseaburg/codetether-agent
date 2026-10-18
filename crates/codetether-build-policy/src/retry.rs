//! Limit tool-first retry to actionable build-agent responses.
use crate::build_request_requires_tool;
use codetether_provider_core::{Message, ToolDefinition};
use std::path::Path;

/// Apply the established is build agent rule.
/// # Arguments
/// Receives the message, tool or workspace inputs named in the signature.
/// # Returns
/// The decision or matching paths without executing tools.
/// # Examples
/// ```
/// assert!(codetether_build_policy::is_build_agent("BUILD"));
/// ```
pub fn is_build_agent(agent_name: &str) -> bool {
    agent_name.eq_ignore_ascii_case("build")
}

/// Apply the established should force build tool first retry rule.
/// # Arguments
/// Receives the message, tool or workspace inputs named in the signature.
/// # Returns
/// The decision or matching paths without executing tools.
/// # Examples
/// ```
/// assert!(!codetether_build_policy::should_force_build_tool_first_retry("build", 0, &[], &[], std::path::Path::new("."), "done", false, 2));
/// ```
pub fn should_force_build_tool_first_retry(
    agent_name: &str,
    retry_count: u8,
    tool_definitions: &[ToolDefinition],
    session_messages: &[Message],
    workspace_dir: &Path,
    assistant_text: &str,
    has_tool_calls: bool,
    build_mode_tool_first_max_retries: u8,
) -> bool {
    if retry_count >= build_mode_tool_first_max_retries
        || !is_build_agent(agent_name)
        || tool_definitions.is_empty()
        || has_tool_calls
        || assistant_text.trim().is_empty()
        || !build_request_requires_tool(session_messages, workspace_dir)
    {
        return false;
    }

    true
}
