//! Pure guard regressions: no interpreter or child process is executed.

use crate::tool::shell_command_guard::{result, result_for_args};
use serde_json::json;

mod allowed;
mod blocked;
mod fields;

#[test]
fn python_execution_blocks_both_shell_tools() {
    for &command in blocked::COMMANDS {
        for tool in ["bash", "exec_command"] {
            let args = if tool == "bash" {
                json!({"command": command})
            } else {
                json!({"cmd": command})
            };
            let blocked = result_for_args(tool, &args).expect(command);
            assert!(!blocked.success, "{tool}: {command}");
            assert!(
                blocked.output.contains("PYTHON_EXECUTION_BLOCKED"),
                "{command}"
            );
            assert_eq!(blocked.metadata.get("tool"), Some(&json!(tool)));
        }
    }
}

#[test]
fn python_execution_allows_mentions_and_other_native_commands() {
    for &command in allowed::COMMANDS {
        assert!(result("bash", command).is_none(), "{command}");
        assert!(result("exec_command", command).is_none(), "{command}");
    }
}
