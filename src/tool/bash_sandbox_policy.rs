//! Sandbox construction, preflight, and approval ordering for Bash.

use anyhow::{Context, Result};
use serde_json::Value;
use std::path::{Path, PathBuf};

use crate::tool::sandbox::{SandboxPolicy, SandboxResult};

#[path = "bash_sandbox_paths.rs"]
mod paths;

pub(super) fn authorize(sandboxed: bool, _command: &str, args: &Value) -> Result<()> {
    if !sandboxed {
        if !crate::tool::sandbox::direct_fallback_env_allowed() {
            anyhow::bail!("direct Bash execution requires the explicit unsafe fallback setting");
        }
        if !crate::tool::network_access::allowed_for(args) {
            anyhow::bail!("direct Bash cannot enforce disabled network authority");
        }
        if !crate::runtime_policy::approved_receipt("bash", args) {
            anyhow::bail!("direct Bash execution requires an exact approval receipt");
        }
        crate::approval::use_once::claim("bash", args).context("approval claim failed")?;
    }
    Ok(())
}

pub(super) async fn execute(
    command: &str,
    policy: &SandboxPolicy,
    work_dir: &Path,
    approval_args: &Value,
) -> Result<SandboxResult> {
    let shell = crate::tool::bash_shell::resolve();
    let mut args = shell.prefix_args;
    args.push(command.to_string());
    crate::tool::sandbox::sandbox_preflight::validate(&shell.program, &args, policy, work_dir)
        .await
        .context("sandbox preflight failed")?;
    crate::approval::use_once::claim("bash", approval_args).context("approval claim failed")?;
    crate::tool::sandbox::execute_sandboxed(&shell.program, &args, policy, Some(work_dir)).await
}

pub(super) async fn policy(root: PathBuf, timeout_secs: u64, args: &Value) -> SandboxPolicy {
    SandboxPolicy {
        allowed_paths: paths::writable(root, args).await,
        allow_network: super::bash_sandbox_config::allow_network(args),
        allow_exec: true,
        timeout_secs,
        ..SandboxPolicy::default()
    }
}