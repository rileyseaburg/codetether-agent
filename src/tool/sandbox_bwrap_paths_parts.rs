//! Focused Bubblewrap path-policy helpers.

use super::SandboxPolicy;
use std::collections::HashSet;
use std::path::Path;

#[path = "sandbox_bwrap_absolute.rs"]
mod absolute;
#[path = "sandbox_bwrap_protected.rs"]
mod protected;
#[path = "sandbox_bwrap_read_only.rs"]
mod read_only;
#[path = "sandbox_bwrap_tmp.rs"]
mod tmp;
#[path = "sandbox_bwrap_writable.rs"]
mod writable;

pub(super) fn protected_mounts(out: &mut Vec<String>, policy: &SandboxPolicy) {
    protected::mounts(out, policy);
}

pub(super) fn prepare_work_dir(
    out: &mut Vec<String>, seen: &mut HashSet<String>, policy: &SandboxPolicy, work_dir: &Path,
) {
    tmp::prepare_work_dir(out, seen, policy, work_dir);
}

pub(super) fn push_absolute(
    specs: &mut Vec<(&'static str, String)>, op: &'static str, path: &Path,
) {
    absolute::push(specs, op, path);
}

pub(super) fn read_only_specs(policy: &SandboxPolicy) -> Vec<(&'static str, String)> {
    read_only::specs(policy)
}

pub(super) fn uses_tmpfs(policy: &SandboxPolicy, work_dir: &Path) -> bool {
    tmp::uses_tmpfs(policy, work_dir)
}

pub(super) fn writable_covered(policy: &SandboxPolicy, work_dir: &Path) -> bool {
    writable::covered(policy, work_dir)
}