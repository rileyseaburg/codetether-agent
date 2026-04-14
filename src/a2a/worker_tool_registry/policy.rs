use crate::a2a::worker::AutoApprove;

/// Return whether a tool may run under the selected auto-approve policy.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::worker::AutoApprove;
/// use codetether_agent::a2a::worker_tool_registry::is_tool_allowed;
///
/// assert!(is_tool_allowed("read", AutoApprove::Safe));
/// assert!(is_tool_allowed("todoread", AutoApprove::Safe));
/// assert!(!is_tool_allowed("write", AutoApprove::Safe));
/// assert!(is_tool_allowed("write", AutoApprove::All));
/// ```
pub fn is_tool_allowed(tool_name: &str, auto_approve: AutoApprove) -> bool {
    matches!(auto_approve, AutoApprove::All) || is_safe_tool(tool_name)
}

/// Return whether a tool belongs to the worker's read-only safe list.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::worker_tool_registry::is_safe_tool;
///
/// assert!(is_safe_tool("read"));
/// assert!(is_safe_tool("todoread"));
/// assert!(!is_safe_tool("write"));
/// ```
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
