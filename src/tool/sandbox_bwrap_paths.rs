use super::SandboxPolicy;
use super::sandbox_bwrap_push::{push_pair, push_triple};
#[path = "sandbox_bwrap_absolute.rs"]
mod absolute;
#[path = "sandbox_bwrap_protected.rs"]
mod protected;
#[path = "sandbox_bwrap_tmp.rs"]
mod tmp;
#[path = "sandbox_bwrap_writable.rs"]
mod writable;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub(super) fn mounts(out: &mut Vec<String>, policy: &SandboxPolicy, work_dir: &Path) {
    let mut dirs = HashSet::new();
    let mut binds = HashSet::new();
    for (op, path) in mount_specs(policy, work_dir) {
        add_parent_dirs(out, &mut dirs, &path);
        if binds.insert((op, path.clone())) {
            push_triple(out, op, &path, &path);
        }
    }
    protected::mounts(out, policy);
    tmp::prepare_work_dir(out, &mut dirs, policy, work_dir);
}

fn mount_specs(policy: &SandboxPolicy, work_dir: &Path) -> Vec<(&'static str, String)> {
    let mut specs = Vec::new();
    for path in &policy.allowed_paths {
        absolute::push(&mut specs, "--bind", path);
    }
    if !tmp::uses_tmpfs(policy, work_dir) && !writable::covered(policy, work_dir) {
        absolute::push(&mut specs, "--ro-bind", work_dir);
    }
    specs
}

fn add_parent_dirs(out: &mut Vec<String>, seen: &mut HashSet<String>, path: &str) {
    let Some(parent) = Path::new(path).parent() else {
        return;
    };
    let mut current = PathBuf::new();
    for component in parent.components() {
        current.push(component.as_os_str());
        let dir = current.display().to_string();
        if dir != "/" && seen.insert(dir.clone()) {
            push_pair(out, "--dir", &dir);
        }
    }
}
