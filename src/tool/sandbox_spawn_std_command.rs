//! Restricted environment and stdio configuration for synchronous children.

use super::super::{sandbox_landlock, sandbox_seccomp};
use anyhow::{Context, Result};
use std::path::Path;
use std::process::{Child, Command, Stdio};

pub(super) fn spawn(
    program: &str,
    args: &[String],
    cwd: &Path,
    has_stdin: bool,
    environment: &[(String, String)],
    landlock: Option<sandbox_landlock::Rules>,
    seccomp: Option<&sandbox_seccomp::Program>,
    apply_seccomp: bool,
) -> Result<Child> {
    let mut command = Command::new(program);
    command
        .args(args)
        .current_dir(cwd)
        .env_clear()
        .envs(crate::tool::sandbox::restricted_env())
        .envs(environment.iter().cloned())
        .stdin(if has_stdin {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    sandbox_landlock::apply_std(&mut command, landlock);
    if apply_seccomp && let Some(program) = seccomp {
        sandbox_seccomp::apply_std(&mut command, program);
    }
    command.spawn().context("failed to spawn sandboxed process")
}