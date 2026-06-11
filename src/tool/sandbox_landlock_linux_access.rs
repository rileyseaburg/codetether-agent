use std::path::Path;

use super::sys;
use crate::tool::sandbox::SandboxPolicy;

pub(super) fn work_dir(policy: &SandboxPolicy, work_dir: &Path) -> u64 {
    if policy
        .allowed_paths
        .iter()
        .any(|path| work_dir.starts_with(path))
    {
        sys::READ_ACCESS | sys::WRITE_ACCESS
    } else {
        sys::READ_ACCESS
    }
}
