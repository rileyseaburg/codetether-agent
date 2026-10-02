//! Explicit opt-in proof that ordinary Rust proxies work in the real OS sandbox.

use crate::tool::sandbox::{SandboxPolicy, execute_sandboxed};

#[tokio::test]
#[ignore = "requires a host Rustup installation and available OS sandbox"]
async fn sandboxed_rust_commands_receive_usable_toolchain_environment() {
    let _lock = crate::approval::test_env::lock_env();
    assert!(crate::tool::sandbox::unavailable_reason().is_none());
    let workspace = tempfile::tempdir().unwrap();
    let policy = SandboxPolicy {
        allowed_paths: vec![workspace.path().into()],
        allow_exec: true,
        timeout_secs: 30,
        ..SandboxPolicy::default()
    };
    let args = vec![
        "-c".into(),
        "test \"$HOME\" = /tmp && test -d \"$RUSTUP_HOME/toolchains\" && \
         test \"$CARGO_HOME\" = /tmp/.cargo && cargo --version && rustc --version"
            .into(),
    ];
    let result = execute_sandboxed("sh", &args, &policy, Some(workspace.path()))
        .await
        .unwrap();
    assert!(result.success, "{}", result.output);
    assert!(result.output.contains("cargo "), "{}", result.output);
    assert!(result.output.contains("rustc "), "{}", result.output);
}
