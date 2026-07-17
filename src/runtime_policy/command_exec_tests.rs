use crate::config::{Config, PermissionAction};
use crate::runtime_policy::evaluate_tool_invocation_with_config;
use serde_json::json;

#[test]
fn exec_command_uses_the_same_command_policy_as_bash() {
    let safe = json!({"cmd": "date"});
    let unsafe_command = json!({"cmd": "cargo test"});
    assert!(
        evaluate_tool_invocation_with_config(&Config::default(), "exec_command", &safe).is_none()
    );
    assert!(
        evaluate_tool_invocation_with_config(&Config::default(), "exec_command", &unsafe_command)
            .is_some()
    );
}

#[test]
fn write_stdin_reuses_process_authority_but_honors_explicit_denial() {
    let args = json!({"session_id": 1, "chars": "yes\n"});
    assert!(
        evaluate_tool_invocation_with_config(&Config::default(), "write_stdin", &args).is_none()
    );
    let mut config = Config::default();
    config
        .permissions
        .tools
        .insert("write_stdin".into(), PermissionAction::Deny);
    assert!(evaluate_tool_invocation_with_config(&config, "write_stdin", &args).is_some());
}
