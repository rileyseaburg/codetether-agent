//! Shell and workspace resolution for one command invocation.

use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};

use super::input::Input;

pub(super) fn cwd(default: Option<&Path>, requested: Option<&str>) -> Result<PathBuf> {
    let base = default
        .map(Path::to_path_buf)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(std::env::temp_dir);
    let base = base.canonicalize().map_err(|error| anyhow!("invalid workspace: {error}"))?;
    let cwd = requested.map(PathBuf::from).unwrap_or_else(|| base.clone());
    let requested = if cwd.is_absolute() {
        cwd
    } else {
        base.join(&cwd)
    };
    let resolved = requested
        .canonicalize()
        .map_err(|error| anyhow!("invalid workdir {}: {error}", requested.display()))?;
    if !resolved.starts_with(&base) {
        return Err(anyhow!("workdir is outside workspace: {}", resolved.display()));
    }
    Ok(resolved)
}

pub(super) fn invocation(input: &Input) -> (String, Vec<String>) {
    if let Some(program) = &input.shell {
        let flag = if input.login { "-lc" } else { "-c" };
        return (program.clone(), vec![flag.into(), input.cmd.clone()]);
    }
    let shell = crate::tool::bash_shell::resolve();
    let mut args = shell.prefix_args;
    if input.login && args.last().is_some_and(|arg| arg == "-c") {
        *args.last_mut().expect("shell has a command flag") = "-lc".into();
    }
    args.push(input.cmd.clone());
    (shell.program, args)
}