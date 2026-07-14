//! Remote subtask runner for Kubernetes-backed swarm execution.

use super::executor::{AgentLoopExit, run_agent_loop, workspace_registry};
use super::kubernetes_executor::{SWARM_SUBTASK_RESULT_PREFIX, decode_payload};
use super::subtask::SubTaskResult;
use super::tool_policy;
use crate::cli::SwarmSubagentArgs;
use crate::provider::ProviderRegistry;
use anyhow::{Context, Result, anyhow};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

pub async fn run_swarm_subagent(args: SwarmSubagentArgs) -> Result<()> {
    let payload_b64 = if let Some(payload) = args.payload_base64 {
        payload
    } else {
        std::env::var(&args.payload_env)
            .with_context(|| format!("Missing swarm payload env var {}", args.payload_env))?
    };
    let payload = decode_payload(&payload_b64)?;

    let working_dir = payload
        .working_dir
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    std::env::set_current_dir(&working_dir)
        .with_context(|| format!("Failed to set working dir to {}", working_dir.display()))?;

    let registry = ProviderRegistry::from_vault().await?;
    let provider = registry
        .get(&payload.provider)
        .ok_or_else(|| anyhow!("Provider '{}' unavailable in remote pod", payload.provider))?;

    let capability =
        workspace_registry::Capability::from_flags(payload.read_only, payload.verification);
    let tool_registry = Arc::new(workspace_registry::with_provider(
        Arc::clone(&provider),
        payload.model.clone(),
        &working_dir,
        capability,
    ));
    let tool_defs = tool_registry.definitions();

    let specialty = if payload.specialty.is_empty() {
        "generalist".to_string()
    } else {
        payload.specialty.clone()
    };
    let working_dir_display = working_dir.display().to_string();
    let system_prompt = tool_policy::system_prompt(tool_policy::SystemPromptInput {
        specialty: &specialty,
        subtask_id: &payload.subtask_id,
        working_dir: &working_dir_display,
        model: &payload.model,
        instruction: &payload.instruction,
        context: &payload.context,
        line_limit: None,
        read_only: payload.read_only,
        expects_changes: !payload.read_only && !payload.verification,
    });
    let user_prompt = if payload.context.trim().is_empty() {
        payload.instruction.clone()
    } else {
        format!(
            "{}\n\nContext from dependencies:\n{}",
            payload.instruction, payload.context
        )
    };

    let probe = super::remote_probe::start(&payload.subtask_id, payload.probe_interval_secs);

    let started = Instant::now();
    let run_result = run_agent_loop(
        provider,
        &payload.model,
        &system_prompt,
        &user_prompt,
        tool_defs,
        tool_registry,
        payload.max_steps,
        payload.timeout_secs,
        None,
        payload.subtask_id.clone(),
        None,
        Some(working_dir.clone()),
    )
    .await;
    probe.finish();

    let result = match run_result {
        Ok((output, steps, tool_calls, exit_reason)) => {
            let output = tool_policy::verify_output(&payload.instruction, &output);
            let (success, error) = match exit_reason {
                AgentLoopExit::Completed => match tool_policy::deliverable_error(&output) {
                    Some(error) => (false, Some(error)),
                    None => (true, None),
                },
                AgentLoopExit::MaxStepsReached => (
                    false,
                    Some(format!("Sub-agent hit max steps ({})", payload.max_steps)),
                ),
                AgentLoopExit::TimedOut => (
                    false,
                    Some(format!(
                        "Sub-agent timed out after {}s",
                        payload.timeout_secs
                    )),
                ),
            };
            SubTaskResult {
                subtask_id: payload.subtask_id.clone(),
                subagent_id: format!("agent-{}", payload.subtask_id),
                success,
                result: output,
                steps,
                tool_calls,
                execution_time_ms: started.elapsed().as_millis() as u64,
                error,
                artifacts: Vec::new(),

                retry_count: 0,
            }
        }
        Err(error) => SubTaskResult {
            subtask_id: payload.subtask_id.clone(),
            subagent_id: format!("agent-{}", payload.subtask_id),
            success: false,
            result: String::new(),
            steps: 0,
            tool_calls: 0,
            execution_time_ms: started.elapsed().as_millis() as u64,
            error: Some(error.to_string()),
            artifacts: Vec::new(),

            retry_count: 0,
        },
    };

    println!(
        "{}{}",
        SWARM_SUBTASK_RESULT_PREFIX,
        serde_json::to_string(&result)?
    );
    Ok(())
}
