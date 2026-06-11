use super::super::SandboxPolicy;
use super::super::sandbox_bwrap_push::push_pair;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub(super) fn uses_tmpfs(policy: &SandboxPolicy, work_dir: &Path) -> bool {
    work_dir.starts_with("/tmp")
        && !policy
            .allowed_paths
            .iter()
            .any(|allowed| work_dir.starts_with(allowed))
}

pub(super) fn prepare_work_dir(
    out: &mut Vec<String>,
    seen: &mut HashSet<String>,
    policy: &SandboxPolicy,
    work_dir: &Path,
) {
    if !uses_tmpfs(policy, work_dir) || work_dir == Path::new("/tmp") {
        return;
    }
    let mut current = PathBuf::new();
    for component in work_dir.components() {
        current.push(component.as_os_str());
        let dir = current.display().to_string();
        if dir != "/" && dir != "/tmp" && seen.insert(dir.clone()) {
            push_pair(out, "--dir", &dir);
        }
    }
}
