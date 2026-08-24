//! Sandboxed Git subprocess execution for trusted orchestration code.

#[path = "process_refs.rs"]
mod refs;
pub(crate) use refs::{output_blocking_refs, output_refs};
#[path = "process_network.rs"]
mod network;
pub(crate) use network::output_networked;
#[path = "process_policy.rs"]
mod process_policy;

use crate::tool::sandbox::sandbox_spawn;
use anyhow::{Context, Result};
use std::path::Path;

pub(crate) fn preflight(cwd: &Path, mutating: bool) -> Result<()> {
    let (policy, cwd) = process_policy::resolve(cwd, mutating)?;
    crate::tool::sandbox::sandbox_spawn_std::preflight(&policy, &cwd)
        .context("git sandbox unavailable")
}

pub(crate) async fn output(
    cwd: &Path,
    args: &[String],
    environment: &[(String, String)],
    mutating: bool,
) -> Result<std::process::Output> {
    let (policy, cwd) = process_policy::resolve(cwd, mutating)?;
    let child = sandbox_spawn::spawn("git", args, &policy, &cwd, false, environment)
        .await
        .context("failed to spawn sandboxed git")?
        .child;
    child
        .wait_with_output()
        .await
        .context("failed to collect sandboxed git output")
}

pub(crate) fn output_blocking(
    cwd: &Path,
    args: &[String],
    environment: &[(String, String)],
    mutating: bool,
) -> Result<std::process::Output> {
    let (policy, cwd) = process_policy::resolve(cwd, mutating)?;
    crate::tool::sandbox::sandbox_spawn_std::spawn_with_env(
        "git",
        args,
        &policy,
        &cwd,
        environment,
    )?
    .wait_with_output()
    .context("failed to collect sandboxed git output")
}
