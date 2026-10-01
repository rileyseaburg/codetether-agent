//! Ignored extra fields and approval metadata cannot mask the real command.

#[test]
fn python_execution_uses_the_executing_tools_command_field() {
    for (tool, args) in [
        (
            "exec_command",
            serde_json::json!({
                "cmd": "python3 script.py", "command": "date",
                "approval_id": "fixture", "sandbox_permissions": "require_escalated"
            }),
        ),
        (
            "bash",
            serde_json::json!({"command": "python3 script.py", "cmd": "date"}),
        ),
    ] {
        let blocked = crate::tool::shell_command_guard::result_for_args(tool, &args)
            .expect("the executing field must be checked");
        assert!(!blocked.success);
        assert!(blocked.output.contains("PYTHON_EXECUTION_BLOCKED"));
    }
}
