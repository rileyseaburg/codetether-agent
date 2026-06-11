use super::{SandboxPolicy, SandboxResult};
use super::{
    sandbox_command, sandbox_env, sandbox_plan_state, sandbox_process, sandbox_result_builder,
    sandbox_runner,
};
use anyhow::{Result, anyhow};
use std::path::Path;

pub async fn execute_sandboxed(
    command: &str,
    args: &[String],
    policy: &SandboxPolicy,
    working_dir: Option<&Path>,
) -> Result<SandboxResult> {
    let started = std::time::Instant::now();
    let mut violations = Vec::new();
    if !policy.allow_exec {
        return Err(anyhow!("Sandbox policy denies process execution"));
    }
    let mut env = sandbox_env::restricted();
    let network_fallbacks = super::super::sandbox_network::validate(policy, command, args)?;
    if !policy.allow_network {
        env.insert("CODETETHER_SANDBOX_NO_NETWORK".to_string(), "1".to_string());
    }
    let work_dir = working_dir
        .map(|p| p.to_path_buf())
        .unwrap_or_else(std::env::temp_dir);
    super::super::sandbox_paths::validate_working_dir(policy, &work_dir).await?;
    super::super::sandbox_paths::validate_command_args(args)?;
    let plan = sandbox_runner::plan(command, args, policy, &work_dir)?;
    let state = sandbox_plan_state::from_plan(plan, network_fallbacks);
    let (cmd, limit_fallbacks) = sandbox_command::build(
        &state.program,
        &state.args,
        &work_dir,
        &env,
        state.landlock,
        policy.max_memory_bytes,
    );
    let mut unsafe_fallbacks = state.unsafe_fallbacks;
    unsafe_fallbacks.extend(limit_fallbacks);
    let output = sandbox_process::wait(cmd, policy.timeout_secs, &mut violations).await?;
    Ok(sandbox_result_builder::from_output(
        command,
        output,
        started,
        violations,
        unsafe_fallbacks,
    ))
}
