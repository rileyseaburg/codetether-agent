//! Git push refspec validation for worker commits.

use anyhow::Result;
use std::{ffi::OsStr, path::Path};

pub(super) fn push_refspec(branch: &str) -> Result<String> {
    let branch = branch.trim();
    if branch.is_empty() || branch == "HEAD" || invalid(branch) {
        anyhow::bail!("Cannot push task changes: invalid branch name {branch:?}");
    }
    Ok(format!("HEAD:refs/heads/{branch}"))
}

fn invalid(branch: &str) -> bool {
    branch.starts_with('-')
        || branch.contains("..")
        || branch.contains('@')
        || branch.contains('\\')
        || branch.contains(' ')
        || branch.contains(':')
        || Path::new(branch).components().any(|component| {
            let value = component.as_os_str();
            value == OsStr::new(".") || value == OsStr::new("..")
        })
}
