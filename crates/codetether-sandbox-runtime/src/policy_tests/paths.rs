//! Reject working directories outside the allowed workspace roots.
use crate::{SandboxPolicy, execute_sandboxed};
#[tokio::test]
async fn sandbox_denies_working_dir_outside_allowed_paths() {
    let allowed = tempfile::tempdir().expect("allowed tempdir");
    let denied = tempfile::tempdir().expect("denied tempdir");
    let policy = SandboxPolicy {
        allowed_paths: vec![allowed.path().to_path_buf()],
        allow_network: true,
        allow_exec: true,
        ..SandboxPolicy::default()
    };

    let err = execute_sandboxed(
        "sh",
        &["-c".to_string(), "echo denied".to_string()],
        &policy,
        Some(denied.path()),
    )
    .await
    .expect_err("denied path should fail closed");
    assert!(err.to_string().contains("denied working directory"));
}
