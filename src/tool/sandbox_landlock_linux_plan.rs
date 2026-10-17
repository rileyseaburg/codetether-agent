use super::sys;
#[path = "sandbox_landlock_linux_access.rs"]
mod access;
#[path = "sandbox_landlock_linux_devices.rs"]
mod devices;
#[path = "sandbox_landlock_linux_inactive.rs"]
mod inactive;
#[path = "sandbox_landlock_linux_types.rs"]
mod types;
use crate::tool::sandbox::SandboxPolicy;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
pub(crate) use types::{PathRule, Prepared, Rules};

pub(crate) fn prepare(policy: &SandboxPolicy, work_dir: &Path) -> Prepared {
    let abi = sys::abi_version();
    if abi <= 0 {
        return inactive::prepared("kernel_unavailable");
    }
    match rules(policy, work_dir, abi) {
        Ok(rules) => Prepared {
            rules: Some(rules),
            fallback: None,
        },
        Err(reason) => inactive::prepared(reason),
    }
}

fn rules(policy: &SandboxPolicy, work_dir: &Path, abi: i64) -> Result<Rules, &'static str> {
    let writable = access::writable(abi);
    let mut paths = vec![rule(Path::new("/"), sys::READ_ACCESS)?];
    paths.extend(devices::rules(writable)?);
    for path in &policy.allowed_paths {
        paths.push(rule(path, sys::READ_ACCESS | writable)?);
    }
    paths.push(rule(
        work_dir,
        access::work_dir(policy, work_dir, writable),
    )?);
    Ok(Rules {
        paths,
        handled_access: access::handled(abi),
    })
}

fn rule(path: &Path, access: u64) -> Result<PathRule, &'static str> {
    let path = CString::new(path.as_os_str().as_bytes()).map_err(|_| "invalid_path")?;
    Ok(PathRule { path, access })
}

#[cfg(test)]
#[path = "sandbox_landlock_linux_plan_tests.rs"]
mod tests;