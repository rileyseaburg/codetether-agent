//! Remote subtask runner for Kubernetes-backed swarm execution.

use super::executor::{AgentLoopExit, run_agent_loop};
use super::kubernetes_executor::{
    RemoteBranchProbe, SWARM_SUBTASK_PROBE_PREFIX, SWARM_SUBTASK_RESULT_PREFIX, decode_payload,
};
use super::subtask::SubTaskResult;
use crate::cli::SwarmSubagentArgs;
use crate::provider::ProviderRegistry;
use crate::tool::ToolRegistry;
use anyhow::{Context, Result, anyhow};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::{Duration, Instant};

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

    let tool_registry =
        ToolRegistry::with_provider_arc(Arc::clone(&provider), payload.model.clone());
    // Remote pod is autonomous; interactive question tool is intentionally excluded.
    let tool_defs = tool_registry
        .definitions()
        .into_iter()
        .filter(|t| t.name != "question")
        .collect();

    let specialty = if payload.specialty.is_empty() {
        "generalist".to_string()
    } else {
        payload.specialty.clone()
    };
    let system_prompt = format!(
        "You are a {specialty} specialist sub-agent (ID: {}). \
Use tools to execute the task and summarize concrete outputs.",
        payload.subtask_id
    );
    let user_prompt = if payload.context.trim().is_empty() {
        payload.instruction.clone()
    } else {
        format!(
            "{}\n\nContext from dependencies:\n{}",
            payload.instruction, payload.context
        )
    };

    let done = Arc::new(AtomicBool::new(false));
    let done_for_probe = Arc::clone(&done);
    let subtask_id_for_probe = payload.subtask_id.clone();
    let probe_interval = payload.probe_interval_secs.max(3);
    let probe_thread = thread::spawn(move || {
        emit_probe(&subtask_id_for_probe);
        while !done_for_probe.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(probe_interval));
            if done_for_probe.load(Ordering::Relaxed) {
                break;
            }
            emit_probe(&subtask_id_for_probe);
        }
    });

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
    done.store(true, Ordering::Relaxed);
    let _ = probe_thread.join();

    let result = match run_result {
        Ok((output, steps, tool_calls, exit_reason)) => {
            let (success, error) = match exit_reason {
                AgentLoopExit::Completed => (true, None),
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
                is_retry: false,
                retry_attempts: 0,
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
            is_retry: false,
            retry_attempts: 0,
        },
    };

    println!(
        "{}{}",
        SWARM_SUBTASK_RESULT_PREFIX,
        serde_json::to_string(&result)?
    );
    Ok(())
}

fn emit_probe(subtask_id: &str) {
    let changed_files = collect_changed_files().unwrap_or_default();
    let changed_lines = collect_changed_lines().unwrap_or(0);
    let probe = RemoteBranchProbe {
        subtask_id: subtask_id.to_string(),
        compile_ok: run_cargo_check().unwrap_or(false),
        changed_files: changed_files.into_iter().collect(),
        changed_lines,
    };
    if let Ok(json) = serde_json::to_string(&probe) {
        println!("{SWARM_SUBTASK_PROBE_PREFIX}{json}");
    }
}

fn run_cargo_check() -> Result<bool> {
    let output = Command::new("cargo")
        .args(["check", "--quiet"])
        .output()
        .context("Failed to execute cargo check in remote subtask")?;
    Ok(output.status.success())
}

fn collect_changed_files() -> Result<std::collections::HashSet<String>> {
    let output = Command::new("git")
        .args(["diff", "--name-only"])
        .output()
        .context("Failed to collect changed files in remote subtask")?;
    if !output.status.success() {
        return Ok(Default::default());
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(ToString::to_string)
        .collect())
}

fn collect_changed_lines() -> Result<u32> {
    let output = Command::new("git")
        .args(["diff", "--numstat"])
        .output()
        .context("Failed to collect changed lines in remote subtask")?;
    if !output.status.success() {
        return Ok(0);
    }
    let mut total = 0u32;
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 2 {
            continue;
        }
        total += parts[0].parse::<u32>().unwrap_or(0);
        total += parts[1].parse::<u32>().unwrap_or(0);
    }
    Ok(total)
}
