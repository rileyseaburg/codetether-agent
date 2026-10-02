//! Rust metadata is exposed read-only without inheriting Cargo credentials.

#[test]
fn restricted_environment_has_isolated_cargo_and_mounted_rustup_homes() {
    let env = super::super::restricted();
    assert_eq!(env["HOME"], "/tmp");
    assert_eq!(env["CARGO_HOME"], "/tmp/.cargo");
    assert!(!env.contains_key("CARGO_REGISTRY_TOKEN"));
    assert!(!env.contains_key("GITHUB_TOKEN"));
    if let Some(home) = super::rustup_home() {
        assert_eq!(env["RUSTUP_HOME"], home.to_string_lossy());
        assert!(crate::tool::sandbox::sandbox_toolchain::roots().contains(&home));
    }
}
