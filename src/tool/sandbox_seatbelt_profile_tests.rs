use super::build;
use crate::tool::sandbox::SandboxPolicy;
use std::path::Path;

#[path = "sandbox_seatbelt_profile_roots_tests.rs"]
mod roots_tests;

fn policy(allow_network: bool) -> SandboxPolicy {
    SandboxPolicy {
        allowed_paths: vec!["/workspace".into()],
        allow_network,
        allow_exec: true,
        ..SandboxPolicy::default()
    }
}

fn profile(allow_network: bool) -> String {
    build(
        &policy(allow_network),
        Path::new("/workspace"),
        Path::new("/tmp"),
    )
}

#[test]
fn profile_denies_by_default() {
    let text = profile(false);
    assert!(text.starts_with("(version 1)\n(deny default)"));
}

#[test]
fn profile_grants_writes_only_under_allowed_roots() {
    let text = profile(false);
    assert!(text.contains("(allow file-write* (subpath \"/workspace\"))"));
    assert!(!text.contains("(allow file-write* (subpath \"/etc\"))"));
}

#[test]
fn profile_denies_protected_workspace_paths_after_granting_root() {
    let text = profile(false);
    let allow = text
        .find("(allow file-write* (subpath \"/workspace\"))")
        .expect("workspace write rule");
    let deny = text
        .find("(deny file-write* (subpath \"/workspace/.git\"))")
        .expect("git deny rule");
    assert!(deny > allow, "deny must follow allow to win in SBPL");
}

#[test]
fn profile_scopes_network_and_temp_writes() {
    assert!(!profile(false).contains("(allow network*)"));
    assert!(profile(true).contains("(allow network*)"));
    assert!(profile(false).contains("(allow file-write* (subpath \"/tmp\"))"));
}
