//! Swarm Execute Tool - Parallel task execution across multiple agents
//!
//! This tool enables LLM-driven parallel execution of tasks across multiple
//! sub-agents in a swarm pattern, with configurable concurrency and aggregation.

use super::{Tool, ToolResult};
use crate::provider::{ProviderRegistry, parse_model_string};
use crate::swarm::executor::run_agent_loop;
use crate::swarm::orchestrator::{choose_default_provider, default_model_for_provider};
use crate::tool::ToolRegistry;
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::{Value, json};
use std::sync::Arc;

mod default_impl;
mod input;
mod input_error;
#[cfg(test)]
#[path = "swarm_execute/input_tests.rs"]
mod input_tests;
mod schema;
mod support;
mod task_input;
mod task_result;
mod worktrees;

use input::parse_tasks;
use task_result::TaskResult;

pub struct SwarmExecuteTool;

impl SwarmExecuteTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for SwarmExecuteTool {
    fn id(&self) -> &str {
        "swarm_execute"
    }

    fn name(&self) -> &str {
        "Swarm Execute"
    }

    fn description(&self) -> &str {
        "Run independent tasks in parallel. Pass tasks as instruction strings, for example \
         {\"tasks\":[\"Inspect the API\",\"Run focused tests\"]}. Use task objects only when \
         names, IDs, or specialties are needed; orchestration settings are optional."
    }

    fn parameters(&self) -> Value {
        schema::parameters()
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let tasks = match parse_tasks(&params) {
            Ok(tasks) => tasks,
            Err(error) => return Ok(error),
        };

        let concurrency_limit = params
            .get("concurrency_limit")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(5);
        let aggregation_strategy = params
            .get("aggregation_strategy")
            .and_then(|v| v.as_str())
            .unwrap_or("best_effort")
            .to_string();
        let model = params
            .get("model")
            .and_then(|v| v.as_str())
            .map(String::from);
        let max_steps = params
            .get("max_steps")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(50);
        let timeout_secs = params
            .get("timeout_secs")
            .and_then(|v| v.as_u64())
            .unwrap_or(300);

        let concurrency = concurrency_limit.min(20).max(1);

        tracing::info!(
            task_count = tasks.len(),
            concurrency = concurrency,
            strategy = %aggregation_strategy,
            "Starting swarm execution"
        );

        // Get provider registry from Vault
        let providers = ProviderRegistry::from_vault()
            .await
            .context("Failed to load providers")?;
        let provider_list = providers.list();

        if provider_list.is_empty() {
            return Ok(ToolResult::error(
                "No providers available for swarm execution",
            ));
        }

        // Determine provider/model to use
        let (provider_name, model_name) = if let Some(ref model_str) = model {
            let (prov, mod_id) = parse_model_string(model_str);
            let prov = prov.map(|p| if p == "zhipuai" { "zai" } else { p });
            let provider_name = if let Some(explicit_provider) = prov {
                if provider_list.contains(&explicit_provider) {
                    explicit_provider.to_string()
                } else {
                    return Ok(ToolResult::error(format!(
                        "Provider '{}' selected explicitly but is unavailable. Available providers: {}",
                        explicit_provider,
                        provider_list.join(", ")
                    )));
                }
            } else {
                choose_default_provider(provider_list.as_slice())
                    .ok_or_else(|| anyhow::anyhow!("No providers available for swarm execution"))?
                    .to_string()
            };
            let model_name = if mod_id.trim().is_empty() {
                default_model_for_provider(&provider_name)
            } else {
                mod_id.to_string()
            };
            (provider_name, model_name)
        } else {
            let provider_name = choose_default_provider(provider_list.as_slice())
                .ok_or_else(|| anyhow::anyhow!("No providers available for swarm execution"))?
                .to_string();
            let model_name = default_model_for_provider(&provider_name);
            (provider_name, model_name)
        };

        let provider = providers
            .get(&provider_name)
            .context("Failed to get provider")?;

        tracing::info!(provider = %provider_name, model = %model_name, "Using provider for swarm");

        // Get tool definitions (filtered for sub-agents)
        let tools = support::subagent_tools();

        // Provision one isolated worktree per mutating task so sub-agents
        // never edit the shared checkout (read-only tasks share the dir).
        let worktrees = worktrees::SwarmWorktrees::create(
            &tasks
                .iter()
                .map(|t| (t.name.clone(), t.instruction.clone()))
                .collect::<Vec<_>>(),
        )
        .await;

        // System prompt for sub-agents
        let system_prompt = r#"You are a sub-agent in a swarm execution context.
Your role is to execute the given task independently and report your results.
Focus on completing your specific task efficiently.
Use available tools to accomplish your goal.
When done, provide a clear summary of what you accomplished.
Share any intermediate results using the swarm_share tool so other agents can benefit."#;

        // Execute tasks concurrently using semaphore for rate limiting
        let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));
        let mut join_handles = Vec::new();

        for (idx, task_input) in tasks.clone().into_iter().enumerate() {
            let semaphore = semaphore.clone();
            let provider = provider.clone();
            let tools = tools.clone();
            let working_dir = worktrees.dir(idx);
            let system_prompt = system_prompt.to_string();
            let task_id = task_input
                .id
                .clone()
                .unwrap_or_else(|| format!("task_{}", uuid::Uuid::new_v4()));
            let failed_task_id = task_id.clone();
            let failed_task_name = task_input.name.clone();
            let model_name = model_name.clone();
            let max_steps = max_steps;
            let timeout_secs = timeout_secs;

            let handle = tokio::spawn(async move {
                let _permit = semaphore
                    .acquire()
                    .await
                    .expect("swarm semaphore closed unexpectedly");

                let user_prompt = format!(
                    "Task: {}\nSpecialty: {}\n\nInstruction: {}",
                    task_input.name,
                    task_input
                        .specialty
                        .as_deref()
                        .unwrap_or("Generalist execution"),
                    task_input.instruction
                );

                let (output, steps, tool_calls, exit) = run_agent_loop(
                    provider,
                    &model_name,
                    &system_prompt,
                    &user_prompt,
                    tools,
                    Arc::new(ToolRegistry::new()),
                    max_steps,
                    timeout_secs,
                    None,
                    task_id.clone(),
                    None,
                    working_dir,
                )
                .await?;

                let success = matches!(exit, crate::swarm::executor::AgentLoopExit::Completed)
                    || matches!(exit, crate::swarm::executor::AgentLoopExit::MaxStepsReached);

                Ok::<TaskResult, anyhow::Error>(TaskResult {
                    task_id,
                    task_name: task_input.name,
                    success,
                    output,
                    error: if success {
                        None
                    } else {
                        Some(format!("{:?}", exit))
                    },
                    steps,
                    tool_calls,
                })
            });

            join_handles.push((idx, failed_task_id, failed_task_name, handle));
        }

        // Wait for all tasks to complete
        let mut results: Vec<TaskResult> = Vec::new();
        let mut failures = 0;

        for (idx, failed_task_id, failed_task_name, handle) in join_handles {
            match handle.await {
                Ok(Ok(mut result)) => {
                    worktrees.finish(idx, &mut result).await;
                    if !result.success {
                        failures += 1;
                    }
                    results.push(result);
                }
                Ok(Err(e)) => {
                    failures += 1;
                    tracing::error!(error = %e, "Task execution failed");
                    results.push(TaskResult::failed(
                        failed_task_id,
                        failed_task_name,
                        e.to_string(),
                    ));
                }
                Err(e) => {
                    failures += 1;
                    tracing::error!(error = %e, "Task join failed");
                    results.push(TaskResult::failed(
                        failed_task_id,
                        failed_task_name,
                        format!("Task join failed: {e}"),
                    ));
                }
            }
        }

        // Build aggregation response
        let total = results.len();
        let successes = results.iter().filter(|r| r.success).count();

        let response = if failures == 0 {
            json!({
                "status": "success",
                "results": results,
                "summary": {
                    "total": total,
                    "success": successes,
                    "failures": failures
                }
            })
        } else {
            match aggregation_strategy.as_str() {
                "all" => json!({
                    "status": "partial_failure",
                    "results": results,
                    "summary": {
                        "total": total,
                        "success": successes,
                        "failures": failures
                    }
                }),
                "first_error" => json!({
                    "status": "error",
                    "results": results,
                    "summary": {
                        "total": total,
                        "success": successes,
                        "failures": failures
                    }
                }),
                _ => json!({
                    "status": "partial_success",
                    "results": results,
                    "summary": {
                        "total": total,
                        "success": successes,
                        "failures": failures
                    }
                }),
            }
        };

        Ok(ToolResult::success(response.to_string()))
    }
}
