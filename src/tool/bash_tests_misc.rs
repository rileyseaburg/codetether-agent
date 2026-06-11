use super::super::{BashTool, Tool, interactive_auth_risk_reason, looks_like_auth_prompt};
use serde_json::json;

#[tokio::test]
async fn bash_with_default_cwd_runs_there() {
    let dir = tempfile::tempdir().unwrap();
    let tool = BashTool::with_cwd(dir.path().to_path_buf());
    let result = tool.execute(json!({ "command": "pwd -P" })).await.unwrap();
    let expected = std::fs::canonicalize(dir.path()).unwrap();
    assert_eq!(
        std::path::Path::new(result.output.trim()),
        expected.as_path()
    );
}

#[tokio::test]
async fn unsandboxed_bash_timeout_reports_unsafe_metadata() {
    let tool = BashTool {
        timeout_secs: 1,
        sandboxed: false,
        default_cwd: None,
    };
    let result = tool
        .execute(json!({ "command": "sleep 30" }))
        .await
        .unwrap();
    assert!(!result.success);
    assert_eq!(result.metadata.get("sandboxed"), Some(&json!(false)));
    assert_eq!(result.metadata.get("unsafe_execution"), Some(&json!(true)));
}

#[test]
fn detects_interactive_auth_risk() {
    assert!(interactive_auth_risk_reason("sudo apt update").is_some());
    assert!(interactive_auth_risk_reason("ssh user@host").is_some());
    assert!(interactive_auth_risk_reason("sudo -n apt update").is_none());
    assert!(interactive_auth_risk_reason("ssh -o BatchMode=yes user@host").is_none());
}

#[test]
fn detects_auth_prompt_output() {
    assert!(looks_like_auth_prompt("[sudo] password for riley:"));
    assert!(looks_like_auth_prompt(
        "sudo: a terminal is required to read the password"
    ));
    assert!(!looks_like_auth_prompt("command completed successfully"));
}
