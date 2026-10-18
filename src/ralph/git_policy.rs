//! Native policy type selection and checkout compatibility entry point.
use anyhow::Result;
use std::{path::Path, process::Output};
/// Git policy sharing the application's authoritative invocation check.
pub type Native = codetether_ralph_agent::shell::GitPolicy<crate::tool::registry_lifecycle::Native>;
pub(crate) fn checkout(cwd: &Path, branch: &str) -> Result<Output> {
    codetether_ralph_git::checkout::<Native>(cwd, branch)
}
