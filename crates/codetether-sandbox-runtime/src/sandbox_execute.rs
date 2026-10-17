//! execute for sandboxed process execution.
use super::{SandboxPolicy, SandboxResult};
use super::{
    sandbox_command, sandbox_env, sandbox_plan_state, sandbox_process, sandbox_result_builder,
    sandbox_runner,
};
use anyhow::{Result, anyhow};
use std::path::Path;

/// Validate, plan and run one sandboxed command to completion.
///
/// # Arguments
/// Uses the executable, argument list, policy and optional working directory.
/// # Returns
/// Output, exit status, duration and confinement evidence.
/// # Errors
/// Returns validation, preparation, spawning, waiting or timeout errors.
/// # Examples
/// ```rust,no_run
/// # async fn example(policy: &codetether_sandbox_policy::SandboxPolicy) -> anyhow::Result<()> {
/// let _result = codetether_sandbox_runtime::execute_sandboxed("example", &[], policy, None).await?;
/// # Ok(())
/// # }
/// ```
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
    let mut env = sandbox_env::restricted(policy);
    let network_fallbacks = codetether_sandbox_policy::validate_network(policy, command, args)?;
    if !policy.allow_network {
        env.insert("CODETETHER_SANDBOX_NO_NETWORK".to_string(), "1".to_string());
    }
    let work_dir = working_dir
        .map(|p| p.to_path_buf())
        .unwrap_or_else(std::env::temp_dir);
    codetether_sandbox_policy::validate_working_dir(policy, &work_dir).await?;
    codetether_sandbox_policy::validate_command_args(args)?;
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
