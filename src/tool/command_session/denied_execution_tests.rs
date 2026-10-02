//! A denied escalation must fail before any child process writes.

use crate::tool::{Tool, command_session::Registry, exec_command::ExecCommandTool};
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn denied_escalation_does_not_execute_or_create_git_directory() {
    let _lock = crate::approval::test_env::lock_env();
    let (dir, _env, args) = super::fixture::command(
        "cargo update --offline && git init -q && git add Cargo.toml Cargo.lock",
        false,
    );
    std::fs::write(dir.path().join("Cargo.toml"),
        "[package]\nname = \"approval-write-probe\"\nversion = \"0.1.0\"\n[lib]\npath = \"lib.rs\"\n").unwrap();
    std::fs::write(dir.path().join("lib.rs"), "//! Denied fixture.\n").unwrap();
    let tool = ExecCommandTool::new(Arc::new(Registry::default()), Some(dir.path().into()));
    let result = tool.execute(args).await.unwrap();
    assert!(!result.success);
    assert_eq!(
        result.metadata["error_code"],
        json!("SANDBOX_ESCALATION_NOT_AUTHORIZED")
    );
    assert_eq!(
        result.metadata["execution_diagnostics"]["effective_sandbox_mode"],
        "not-started"
    );
    assert!(!dir.path().join(".git").exists());
    assert!(!dir.path().join("Cargo.lock").exists());
}
