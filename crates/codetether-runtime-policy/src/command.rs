//! Command-level side-effect classification for shell tools.

use serde_json::Value;

/// Read the command argument for a supported shell tool.
///
/// # Arguments
/// * `tool_name` — Tool identifier (bash or exec_command).
/// * `args` — Invocation arguments.
/// # Returns
/// The borrowed command, or `None` for another tool or missing string.
/// # Examples
/// ```
/// let args = serde_json::json!({"cmd": "pwd"});
/// assert_eq!(codetether_runtime_policy::command::value("exec_command", &args), Some("pwd"));
/// ```
pub fn value<'a>(tool_name: &str, args: &'a Value) -> Option<&'a str> {
    match tool_name {
        "bash" => args.get("command").and_then(Value::as_str),
        "exec_command" => args.get("cmd").and_then(Value::as_str),
        _ => None,
    }
}

/// Classify the command argument of a shell invocation.
///
/// # Arguments
/// * `tool_name` — Tool identifier.
/// * `args` — Invocation arguments.
/// # Returns
/// Whether a supported shell command matches the read-only policy.
/// # Examples
/// ```
/// assert!(codetether_runtime_policy::command::is_read_only_shell("bash", &serde_json::json!({"command": "pwd"})));
/// ```
pub fn is_read_only_shell(tool_name: &str, args: &Value) -> bool {
    value(tool_name, args).is_some_and(is_read_only_command)
}

/// Classify a command using the shared syntax guard and read-only prefixes.
///
/// # Arguments
/// * `command` — Shell command text.
/// # Returns
/// Whether the command is eligible for the read-only fast path.
/// # Examples
/// ```
/// assert!(codetether_runtime_policy::command::is_read_only_command("pwd"));
/// assert!(!codetether_runtime_policy::command::is_read_only_command("rm file"));
/// ```
pub fn is_read_only_command(command: &str) -> bool {
    let normalized = command.trim();
    !normalized.is_empty()
        && !super::command_unsafe::rejected(normalized)
        && super::command_prefix::read_only(normalized)
}
