//! Approved escalation performs real offline Cargo and Git writes.

use crate::tool::{Tool, command_session::Registry, exec_command::ExecCommandTool};
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn approved_escalation_runs_cargo_and_git_writes() {
    let _lock = crate::approval::test_env::lock_env();
    let (dir, _env, args) = super::fixture::command(
        "cargo update --offline && git init -q && git add Cargo.toml Cargo.lock",
        true,
    );
    std::fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"approval-write-probe\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .unwrap();
    std::fs::create_dir(dir.path().join("src")).unwrap();
    std::fs::write(dir.path().join("src/lib.rs"), "//! Local write fixture.\n").unwrap();
    let tool = ExecCommandTool::new(Arc::new(Registry::default()), Some(dir.path().into()));
    let result = tool.execute(args).await.unwrap();
    assert!(result.success, "{}", result.output);
    assert_eq!(result.metadata["exit_code"], json!(0), "{}", result.output);
    assert_eq!(result.metadata["sandboxed"], json!(false));
    assert_eq!(
        result.metadata["execution_diagnostics"]["effective_sandbox_mode"],
        "unsandboxed"
    );
    assert!(dir.path().join("Cargo.lock").is_file());
    assert!(dir.path().join(".git/index").is_file());
}
