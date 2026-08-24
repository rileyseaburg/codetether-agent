//! Sandboxed LSP executable-resolution tests.

use super::resolve;
use std::path::Path;

#[test]
fn cargo_bin_language_server_is_exposed_read_only() {
    let Ok(found) = which::which("tetherscript") else {
        return;
    };
    let (executable, policy) = resolve("tetherscript").expect("policy");
    assert_eq!(Path::new(&executable), found);
    assert!(!policy.allow_network);
    assert!(policy.allow_exec);
    assert!(
        policy
            .read_only_paths
            .iter()
            .any(|path| path == found.parent().unwrap())
    );
    assert!(
        !policy
            .allowed_paths
            .iter()
            .any(|path| path == found.parent().unwrap())
    );
}

#[test]
fn rust_analyzer_gets_read_only_toolchain_roots() {
    if which::which("rust-analyzer").is_err() {
        return;
    }
    let (_, policy) = resolve("rust-analyzer").expect("policy");
    let rustup = Path::new(policy.environment.get("RUSTUP_HOME").expect("rustup"));
    assert!(policy.read_only_paths.iter().any(|path| path == rustup));
    assert!(!policy.allowed_paths.iter().any(|path| path == rustup));
}
