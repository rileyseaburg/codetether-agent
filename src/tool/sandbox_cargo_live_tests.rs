//! Explicit opt-in proof that Cargo writes outside a read-only workspace.

use crate::tool::sandbox::{SandboxPolicy, execute_sandboxed};

#[tokio::test]
#[ignore = "requires a host Rustup installation and available OS sandbox"]
async fn cargo_check_uses_isolated_target_for_read_only_workspace() {
    let _lock = crate::approval::test_env::lock_env();
    assert!(crate::tool::sandbox::unavailable_reason().is_none());
    let workspace = tempfile::Builder::new()
        .prefix(".codetether-cargo-sandbox-")
        .tempdir_in(std::env::current_dir().unwrap())
        .unwrap();
    std::fs::create_dir(workspace.path().join("src")).unwrap();
    let manifest = concat!(
        "[package]\n",
        "name = \"sandbox-cargo-fixture\"\n",
        "version = \"0.1.0\"\n",
        "edition = \"2024\"\n",
        "\n[workspace]\n",
    );
    let lockfile = concat!(
        "version = 4\n\n",
        "[[package]]\n",
        "name = \"sandbox-cargo-fixture\"\n",
        "version = \"0.1.0\"\n",
    );
    std::fs::write(workspace.path().join("Cargo.toml"), manifest).unwrap();
    std::fs::write(workspace.path().join("Cargo.lock"), lockfile).unwrap();
    std::fs::write(
        workspace.path().join("src/lib.rs"),
        "pub const ANSWER: u8 = 42;\n",
    )
    .unwrap();
    let policy = SandboxPolicy {
        allow_exec: true,
        timeout_secs: 60,
        ..SandboxPolicy::default()
    };
    let args = vec!["-c".into(), "cargo check --offline --locked --quiet".into()];
    let result = execute_sandboxed("sh", &args, &policy, Some(workspace.path()))
        .await
        .unwrap();
    assert!(result.success, "{}", result.output);
    assert!(!workspace.path().join("target").exists());
}