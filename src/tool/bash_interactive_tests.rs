use super::super::{BashTool, Tool};
use super::shell_reason;
use serde_json::json;

#[test]
fn rejects_interactive_shell_flags() {
    assert!(shell_reason("bash -i").is_some());
    assert!(shell_reason("/bin/bash -ic 'echo hi'").is_some());
    assert!(shell_reason("env FOO=bar zsh --interactive").is_some());
}

#[test]
fn permits_noninteractive_shell_commands() {
    assert!(shell_reason("bash -c 'echo hi'").is_none());
    assert!(shell_reason("printf '%s' hi").is_none());
}

#[tokio::test]
async fn tool_rejects_interactive_shell_before_spawn() {
    let result = BashTool::new()
        .execute(json!({"command": "bash -ic 'echo hi'"}))
        .await
        .expect("tool result");
    assert!(!result.success);
    assert!(
        result.output.contains("require a TTY"),
        "unexpected output: {}",
        result.output
    );
    assert!(!result.output.contains("job control"));
}
