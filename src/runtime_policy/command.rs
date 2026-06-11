//! Command-level side-effect classification for shell tools.

use serde_json::Value;

pub(super) fn is_read_only_bash(args: &Value) -> bool {
    let Some(command) = args.get("command").and_then(Value::as_str) else {
        return false;
    };
    is_read_only_command(command)
}

pub fn is_read_only_command(command: &str) -> bool {
    let normalized = command.trim();
    !normalized.is_empty()
        && !super::command_unsafe::rejected(normalized)
        && super::command_prefix::read_only(normalized)
}
