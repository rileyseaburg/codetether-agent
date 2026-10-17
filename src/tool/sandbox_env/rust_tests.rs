//! Rust metadata is exposed read-only without inheriting Cargo credentials.

#[test]
fn read_only_environment_isolates_cargo_writes_and_mounts_rustup_home() {
    let policy = crate::tool::sandbox::SandboxPolicy::default();
    let env = super::super::restricted(&policy);
    assert_eq!(env["HOME"], "/tmp");
    assert_eq!(env["CARGO_HOME"], "/tmp/.cargo");
    assert_eq!(env["TMPDIR"], env["CARGO_TARGET_DIR"]);
    assert_eq!(
        env["CARGO_TARGET_DIR"],
        "/tmp/.codetether/cargo-target"
    );
    assert!(!env.contains_key("CARGO_REGISTRY_TOKEN"));
    assert!(!env.contains_key("GITHUB_TOKEN"));
    if let Some(home) = super::rustup_home() {
        assert_eq!(env["RUSTUP_HOME"], home.to_string_lossy());
        assert!(crate::tool::sandbox::sandbox_toolchain::roots().contains(&home));
    }
}

#[test]
fn writable_workspace_keeps_cargo_target_in_the_workspace() {
    let mut policy = crate::tool::sandbox::SandboxPolicy::default();
    policy.allowed_paths.push("/workspace".into());
    let env = super::super::restricted(&policy);
    assert!(!env.contains_key("CARGO_TARGET_DIR"));
}