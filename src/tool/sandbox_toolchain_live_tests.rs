//! Live check that host toolchains resolve inside the OS sandbox.
//!
//! Skipped when no sandbox runner is available on the host, so CI without
//! bubblewrap or Landlock still passes.

use super::super::{SandboxPolicy, execute_sandboxed};
use super::ENV;

#[tokio::test]
async fn sandboxed_command_resolves_configured_toolchain_binary() {
    if super::super::unavailable_reason().is_some() {
        return;
    }
    let _lock = crate::approval::test_env::lock_env();
    let toolchain = tempfile::tempdir().expect("toolchain");
    let bin = toolchain.path().join("bin");
    std::fs::create_dir_all(&bin).expect("bin dir");
    let tool = bin.join("ct-toolchain-probe");
    std::fs::write(&tool, "#!/bin/sh\necho toolchain-ok\n").expect("script");
    let mut perms = std::fs::metadata(&tool).expect("meta").permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut perms, 0o755);
    std::fs::set_permissions(&tool, perms).expect("chmod");
    let host_path = std::env::var_os("PATH").unwrap_or_default();
    let with_tool = std::env::join_paths(
        std::iter::once(bin.clone()).chain(std::env::split_paths(&host_path)),
    )
    .expect("join");
    unsafe { std::env::set_var("PATH", &with_tool) };
    unsafe { std::env::set_var(ENV, toolchain.path()) };
    let workspace = tempfile::tempdir().expect("workspace");
    let policy = SandboxPolicy {
        allowed_paths: vec![workspace.path().into()],
        allow_exec: true,
        timeout_secs: 30,
        ..SandboxPolicy::default()
    };
    let args = vec!["-c".to_string(), "ct-toolchain-probe".to_string()];
    let result = execute_sandboxed("sh", &args, &policy, Some(workspace.path())).await;
    unsafe { std::env::set_var("PATH", host_path) };
    unsafe { std::env::remove_var(ENV) };
    let result = result.expect("sandboxed run");
    assert!(result.output.contains("toolchain-ok"), "{result:?}");
}
