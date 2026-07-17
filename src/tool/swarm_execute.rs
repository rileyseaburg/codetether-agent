//! Swarm Execute Tool - Parallel task execution across multiple agents
//!
//! This tool enables LLM-driven parallel execution of tasks across multiple
//! sub-agents in a swarm pattern, with configurable concurrency and aggregation.

use super::{Tool, ToolResult};
use crate::swarm::executor::run_agent_loop;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::sync::Arc;

mod abort_on_drop;
pub(crate) mod agent_prompt;
pub(crate) mod agent_registry;
mod aggregate;
mod aggregate_response;
mod default_impl;
mod execution_config;
mod input;
mod input_error;
#[cfg(test)]
#[path = "swarm_execute/input_tests.rs"]
mod input_tests;
mod model_request;
mod model_selection;
#[cfg(test)]
#[path = "swarm_execute/model_selection_tests.rs"]
mod model_selection_tests;
mod provider_setup;
mod schema;
pub(crate) mod support;
mod support_task;
mod task_input;
mod task_result;
#[cfg(test)]
#[path = "swarm_execute/task_result_tests.rs"]
mod task_result_tests;
pub(crate) mod tui_bridge;
mod worktree_specs;
mod worktrees;

use execution_config::ExecutionConfig;
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

        let ExecutionConfig {
            concurrency,
            aggregation_strategy,
            model,
            max_steps,
            timeout_secs,
        } = ExecutionConfig::from_params(&params);

        tracing::info!(
            task_count = tasks.len(),
            concurrency = concurrency,
            strategy = %aggregation_strategy,
            "Starting swarm execution"
        );

        let mut observer = tui_bridge::Observer::begin(&tasks, max_steps);

        let selected = match provider_setup::load(model.as_deref(), &mut observer).await? {
            Ok(selected) => selected,
            Err(error) => return Ok(error),
        };
        let provider = selected.provider;
        let selected_model = selected.model;
        tracing::info!(
            requested_model = ?selected_model.requested_model,
            resolved_provider = %selected_model.resolved_provider,
            resolved_model = %selected_model.resolved_model,
            "Using provider for swarm"
        );

        let parent_workspace = support::workspace(&params);
        let shared_results = crate::swarm::result_store::ResultStore::new_arc();

        // Provision one isolated worktree per mutating task so sub-agents
        // never edit the shared checkout (read-only tasks share the dir).
        let specs = worktree_specs::from_tasks(&tasks);
        let worktrees = worktrees::SwarmWorktrees::create(&parent_workspace, &specs).await;

        // Execute tasks concurrently using semaphore for rate limiting
        let semaphore = Arc::new(tokio::sync::Semaphore::new(concurrency));
        let mut join_handles = Vec::new();

        for (idx, task_input) in tasks.clone().into_iter().enumerate() {
            let semaphore = semaphore.clone();
            let provider = provider.clone();
            let read_only = support::is_read_only(
                task_input.intent_name(),
                &task_input.instruction,
                task_input.specialty.as_deref(),
                task_input.needs_worktree,
            );
            let expects_changes = worktrees.expects_changes(idx);
            let verification = worktrees.is_verification(idx);
            let working_dir = worktrees.dir(idx).and_then(|isolated| {
                support::working_directory(isolated, read_only, &parent_workspace)
            });
            let task_id = task_input
                .id
                .clone()
                .unwrap_or_else(|| tui_bridge::task_id(&task_input, idx));
            let model_selection = selected_model.clone();
            let model_name = model_selection.resolved_model.clone();
            let shared_results = Arc::clone(&shared_results);
            let failed_task_id = task_id.clone();
            let failed_task_name = task_input.name.clone();
            let max_steps = max_steps;
            let timeout_secs = timeout_secs;
            let events = observer.sender();

            let handle = tokio::spawn(async move {
                let _permit = semaphore
                    .acquire()
                    .await
                    .expect("swarm semaphore closed unexpectedly");
                let working_dir = working_dir?;
                tui_bridge::started(&events, &task_id, task_input.specialty.as_deref());
                let registry = agent_registry::shared(
                    read_only,
                    verification,
                    &working_dir,
                    shared_results,
                    task_id.clone(),
                    Arc::clone(&provider),
                    model_name.clone(),
                );
                let tools = registry.definitions();
                let system_prompt = agent_prompt::build(
                    &task_id,
                    task_input.specialty.as_deref(),
                    &working_dir,
                    &model_name,
                    &task_input.instruction,
                    read_only,
                    expects_changes,
                );

                let user_prompt = agent_prompt::user(&task_input);

                let (output, steps, tool_calls, exit) = run_agent_loop(
                    provider,
                    &model_name,
                    &system_prompt,
                    &user_prompt,
                    tools,
                    registry,
                    max_steps,
                    timeout_secs,
                    Some(events),
                    task_id.clone(),
                    None,
                    Some(working_dir),
                )
                .await?;

                let error = task_result::failure(exit, &output);
                let success = error.is_none();

                Ok::<TaskResult, anyhow::Error>(TaskResult {
                    task_id,
                    task_name: task_input.name,
                    success,
                    output,
                    error,
                    steps,
                    tool_calls,
                    model: model_selection,
                })
            });

            join_handles.push((
                idx,
                failed_task_id,
                failed_task_name,
                selected_model.clone(),
                abort_on_drop::AbortOnDrop::new(handle),
            ));
        }

        // Wait for all tasks to complete
        let mut results: Vec<TaskResult> = Vec::new();
        let mut failures = 0;

        for (idx, failed_task_id, failed_task_name, model, handle) in join_handles {
            match handle.join().await {
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
                        model,
                    ));
                }
                Err(e) => {
                    failures += 1;
                    tracing::error!(error = %e, "Task join failed");
                    results.push(TaskResult::failed(
                        failed_task_id,
                        failed_task_name,
                        format!("Task join failed: {e}"),
                        model,
                    ));
                }
            }
        }

        observer.complete(&results);
        let response = aggregate_response::build(results, failures, &aggregation_strategy);

        Ok(aggregate::result(response, failures))
    }
}
