//! Extract shell commands from tool argument objects before execution.

/// Check either shell tool's command field, without treating cwd as a write.
///
/// The sandbox's default cwd may be a system temporary directory; only writes
/// to such paths are rejected by the separate temp-write guard.
pub(crate) fn result_for_args(
    tool: &str,
    args: &serde_json::Value,
) -> Option<crate::tool::ToolResult> {
    let command = match tool {
        "bash" => args.get("command"),
        "exec_command" => args.get("cmd"),
        _ => None,
    }
    .and_then(serde_json::Value::as_str)
    .unwrap_or_default();
    super::result(tool, command)
}
