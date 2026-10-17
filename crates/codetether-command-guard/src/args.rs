//! Extract shell commands from tool argument objects before execution.

/// Check either shell tool's command field, without treating cwd as a write.
///
/// The sandbox's default cwd may be a system temporary directory; only writes
/// to such paths are rejected by the separate temp-write guard.
///
/// # Arguments
/// * `tool` - Selects the bash or exec_command command field.
/// * `args` - Tool argument object.
/// # Returns
/// The command rejection, if one of the mandatory checks matches.
/// # Examples
/// ```rust
/// let args = serde_json::json!({"cmd":"echo ready"});
/// assert!(codetether_command_guard::result_for_args("exec_command", &args).is_none());
/// ```
pub fn result_for_args(
    tool: &str,
    args: &serde_json::Value,
) -> Option<codetether_tool_core::ToolResult> {
    let command = match tool {
        "bash" => args.get("command"),
        "exec_command" => args.get("cmd"),
        _ => None,
    }
    .and_then(serde_json::Value::as_str)
    .unwrap_or_default();
    super::result(tool, command)
}
