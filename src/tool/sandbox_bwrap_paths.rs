use super::SandboxPolicy;
use super::sandbox_bwrap_push::{push_pair, push_triple};
#[path = "sandbox_bwrap_paths_parts.rs"]
mod parts;
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
    if policy.protect_metadata {
        parts::protected_mounts(out, policy);
    }
    parts::prepare_work_dir(out, &mut dirs, policy, work_dir);
}

fn mount_specs(policy: &SandboxPolicy, work_dir: &Path) -> Vec<(&'static str, String)> {
    let mut specs = Vec::new();
    for path in &policy.allowed_paths {
        parts::push_absolute(&mut specs, "--bind", path);
    }
    specs.extend(parts::read_only_specs(policy));
    if !parts::uses_tmpfs(policy, work_dir) && !parts::writable_covered(policy, work_dir) {
        parts::push_absolute(&mut specs, "--ro-bind", work_dir);
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