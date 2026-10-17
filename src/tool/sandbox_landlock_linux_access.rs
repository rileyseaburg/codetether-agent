use std::path::Path;

use super::sys;
use crate::tool::sandbox::SandboxPolicy;

pub(super) fn handled(abi: i64) -> u64 {
    sys::READ_ACCESS | writable(abi)
}

pub(super) fn writable(abi: i64) -> u64 {
    let mut access = sys::WRITE_ACCESS;
    if abi >= 2 {
        access |= sys::REFER_ACCESS;
    }
    if abi >= 3 {
        access |= sys::TRUNCATE_ACCESS;
    }
    access
}

pub(super) fn work_dir(policy: &SandboxPolicy, work_dir: &Path, writable: u64) -> u64 {
    if policy
        .allowed_paths
        .iter()
        .any(|path| work_dir.starts_with(path))
    {
        sys::READ_ACCESS | writable
    } else {
        sys::READ_ACCESS
    }
}