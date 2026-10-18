//! Reject network commands before starting a process.
use crate::{SandboxPolicy, execute_sandboxed};
#[tokio::test]
async fn sandbox_denies_network_commands_when_network_disabled() {
    let policy = SandboxPolicy {
        allow_network: false,
        allow_exec: true,
        ..SandboxPolicy::default()
    };
    let err = execute_sandboxed(
        "sh",
        &["-c".to_string(), "curl https://example.com".to_string()],
        &policy,
        None,
    )
    .await
    .expect_err("network command should fail closed");
    assert!(err.to_string().contains("denies network access"));
}
#[tokio::test]
async fn sandbox_denies_dequoted_network_commands() {
    let policy = SandboxPolicy {
        allow_network: false,
        allow_exec: true,
        ..SandboxPolicy::default()
    };
    let err = execute_sandboxed(
        "sh",
        &["-c".to_string(), "c\"\"url https://example.com".to_string()],
        &policy,
        None,
    )
    .await
    .expect_err("dequoted network command should fail closed");
    assert!(err.to_string().contains("denies network access"));
}
