//! Map legacy fallback tool names to the runtime policy's canonical names.
/// Return the policy identifier for an MCP tool name.
/// # Arguments
/// * `name` - Registered MCP tool identifier.
/// # Returns
/// The canonical bash/write alias, or the input name unchanged.
/// # Examples
/// ```
/// assert_eq!(codetether_mcp_fallback::policy::policy_name("run_command"), "bash");
/// ```
pub fn policy_name(name: &str) -> &str {
    match name {
        "run_command" => "bash",
        "write_file" => "write",
        _ => name,
    }
}

#[cfg(test)]
#[path = "policy_tests.rs"]
mod tests;
