//! Side-effect-free sandbox validation performed before approval consumption.

use super::{SandboxPolicy, sandbox_network_isolation, sandbox_runner};
use anyhow::{Result, anyhow};
use std::path::Path;

pub(crate) async fn validate(
    command: &str,
    args: &[String],
    policy: &SandboxPolicy,
    working_dir: &Path,
) -> Result<()> {
    if !policy.allow_exec {
        return Err(anyhow!("Sandbox policy denies process execution"));
    }
    super::super::sandbox_network::validate(policy, command, args)?;
    super::super::sandbox_paths::validate_working_dir(policy, working_dir).await?;
    super::super::sandbox_paths::validate_command_args(args)?;
    let plan = sandbox_runner::plan(command, args, policy, working_dir)?;
    sandbox_network_isolation::validate(policy, plan.network_isolated)
}