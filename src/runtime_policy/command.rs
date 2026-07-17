//! Command-level side-effect classification for shell tools.

use serde_json::Value;

pub(super) fn value<'a>(tool_name: &str, args: &'a Value) -> Option<&'a str> {
    match tool_name {
        "bash" => args.get("command").and_then(Value::as_str),
        "exec_command" => args.get("cmd").and_then(Value::as_str),
        _ => None,
    }
}

pub(super) fn is_read_only_shell(tool_name: &str, args: &Value) -> bool {
    value(tool_name, args).is_some_and(is_read_only_command)
}

pub fn is_read_only_command(command: &str) -> bool {
    let normalized = command.trim();
    !normalized.is_empty()
        && !super::command_unsafe::rejected(normalized)
        && super::command_prefix::read_only(normalized)
}

#[cfg(test)]
#[path = "command_exec_tests.rs"]
mod exec_tests;
