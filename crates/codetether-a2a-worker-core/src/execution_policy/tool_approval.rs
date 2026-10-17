//! Worker auto-approval decisions and the explicit safe-tool list.
use super::AutoApprove;

/// Return whether a tool may run under the selected auto-approve policy.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_worker_core::execution_policy::AutoApprove;
/// use codetether_a2a_worker_core::execution_policy::is_tool_allowed;
///
/// assert!(is_tool_allowed("read", AutoApprove::Safe));
/// assert!(is_tool_allowed("todoread", AutoApprove::Safe));
/// assert!(!is_tool_allowed("write", AutoApprove::Safe));
/// assert!(is_tool_allowed("write", AutoApprove::All));
/// ```
/// # Arguments
/// * `tool_name` - Tool identifier to check.
/// * `auto_approve` - Worker approval policy.
/// # Returns
/// Whether the policy permits this tool without another approval.
pub fn is_tool_allowed(tool_name: &str, auto_approve: AutoApprove) -> bool {
    matches!(auto_approve, AutoApprove::All) || is_safe_tool(tool_name)
}

/// Return whether a tool belongs to the worker's read-only safe list.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_worker_core::execution_policy::is_safe_tool;
///
/// assert!(is_safe_tool("read"));
/// assert!(is_safe_tool("todoread"));
/// assert!(!is_safe_tool("write"));
/// ```
/// # Arguments
/// * `tool_name` - Tool identifier to check.
/// # Returns
/// Whether the identifier occurs in the worker safe-tool list.
pub fn is_safe_tool(tool_name: &str) -> bool {
    [
        "read",
        "list",
        "glob",
        "grep",
        "codesearch",
        "lsp",
        "webfetch",
        "websearch",
        "todoread",
        "skill",
    ]
    .contains(&tool_name)
}
