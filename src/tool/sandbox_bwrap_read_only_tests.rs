//! Bubblewrap preserves read-only versus writable path authority.

use super::specs;
use crate::tool::sandbox::SandboxPolicy;

#[test]
fn read_only_paths_are_not_writable_binds() {
    let policy = SandboxPolicy {
        read_only_paths: vec!["/toolchain".into()],
        ..Default::default()
    };
    let specs = specs(&policy);
    assert!(specs.contains(&("--ro-bind", "/toolchain".into())));
}