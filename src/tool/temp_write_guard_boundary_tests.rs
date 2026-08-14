//! End-to-end checks that the temp ban holds at real tool boundaries.

use crate::tool::{Tool, file::WriteTool, shell_command_guard};
use serde_json::json;

#[tokio::test]
async fn write_tool_refuses_temp_target_and_creates_nothing() {
    let target = std::env::temp_dir().join("codetether-guard-probe.txt");
    let _ = std::fs::remove_file(&target);
    let args = json!({"path": target.display().to_string(), "content": "must not exist"});

    let result = WriteTool::new().execute(args).await.expect("tool runs");

    assert!(!result.success, "temp write must be refused");
    assert!(
        !target.exists(),
        "refusal must not create {}",
        target.display()
    );
}

#[tokio::test]
async fn write_tool_still_accepts_workspace_targets() {
    // Relative workspace path, deliberately not under the system temp root.
    let target = std::path::Path::new("target").join("codetether-guard-allowed.txt");
    std::fs::create_dir_all("target").expect("target dir");
    let args = json!({"path": target.display().to_string(), "content": "ok"});

    let result = WriteTool::new().execute(args).await.expect("tool runs");

    assert!(
        result.success,
        "workspace write must pass: {}",
        result.output
    );
    assert_eq!(std::fs::read_to_string(&target).expect("written"), "ok");
    let _ = std::fs::remove_file(&target);
}

#[test]
fn shell_guard_refuses_an_absolute_temp_write() {
    let args = json!({"command": "printf x > /tmp/escaped.txt"});
    assert!(shell_command_guard::result_for_args("bash", &args).is_some());
}

#[test]
fn shell_guard_allows_a_temp_cwd_because_the_sandbox_relies_on_it() {
    let args = json!({"command": "cargo test", "cwd": std::env::temp_dir()});
    assert!(shell_command_guard::result_for_args("bash", &args).is_none());
}
