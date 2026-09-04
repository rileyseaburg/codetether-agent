//! Ordered bind-mount specs for a bubblewrap sandbox.
//!
//! Writable workspace roots come first, then read-only host toolchains, then
//! a read-only working directory when nothing else already covers it.

use super::super::SandboxPolicy;
use super::{absolute, tmp, writable};
use std::path::Path;

pub(super) fn mount_specs(policy: &SandboxPolicy, work_dir: &Path) -> Vec<(&'static str, String)> {
    let mut specs = Vec::new();
    for path in &policy.allowed_paths {
        absolute::push(&mut specs, "--bind", path);
    }
    for root in super::super::sandbox_toolchain::roots() {
        if !writable::covered(policy, &root) {
            absolute::push(&mut specs, "--ro-bind", &root);
        }
    }
    if !tmp::uses_tmpfs(policy, work_dir) && !writable::covered(policy, work_dir) {
        absolute::push(&mut specs, "--ro-bind", work_dir);
    }
    specs
}
