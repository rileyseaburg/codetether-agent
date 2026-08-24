use super::sys;
#[path = "sandbox_landlock_linux_access.rs"]
mod access;
#[path = "sandbox_landlock_linux_devices.rs"]
mod devices;
#[path = "sandbox_landlock_linux_inactive.rs"]
mod inactive;
#[path = "sandbox_landlock_linux_policy_paths.rs"]
mod policy_paths;
use crate::tool::sandbox::SandboxPolicy;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

#[derive(Debug, Clone)]
pub(crate) struct Rules {
    pub paths: Vec<PathRule>,
}

#[derive(Debug, Clone)]
pub(crate) struct PathRule {
    pub path: CString,
    pub access: u64,
}

#[derive(Debug)]
pub(crate) struct Prepared {
    pub rules: Option<Rules>,
    pub fallback: Option<String>,
}

pub(crate) fn prepare(policy: &SandboxPolicy, work_dir: &Path) -> Prepared {
    if sys::abi_version() <= 0 {
        return inactive::prepared("kernel_unavailable");
    }
    match rules(policy, work_dir) {
        Ok(rules) => Prepared {
            rules: Some(rules),
            fallback: None,
        },
        Err(reason) => inactive::prepared(reason),
    }
}

fn rules(policy: &SandboxPolicy, work_dir: &Path) -> Result<Rules, &'static str> {
    let mut paths = vec![rule(Path::new("/"), sys::READ_ACCESS)?];
    paths.extend(devices::rules()?);
    paths.extend(policy_paths::rules(policy)?);
    paths.push(rule(work_dir, access::work_dir(policy, work_dir))?);
    Ok(Rules { paths })
}

fn rule(path: &Path, access: u64) -> Result<PathRule, &'static str> {
    let path = CString::new(path.as_os_str().as_bytes()).map_err(|_| "invalid_path")?;
    Ok(PathRule { path, access })
}