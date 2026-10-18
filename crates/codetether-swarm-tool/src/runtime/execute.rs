//! Validate and begin one direct swarm invocation.
use super::{Host, Observer, dispatch};
use crate::{execution_config::ExecutionConfig, input::parse_tasks};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::Value;
/// Execute one direct swarm invocation with the application's native bindings.
/// # Arguments
/// * `params` — Task array and optional execution settings.
/// # Returns
/// Ordered worker results and aggregate status as a tool response.
/// # Errors
/// Propagates provider initialization errors; individual worker failures become results.
/// # Examples
/// ```no_run
/// # async fn invoke<H: codetether_swarm_tool::runtime::Host>() -> anyhow::Result<()> {
/// let _result = codetether_swarm_tool::runtime::execute::<H>(serde_json::json!({"tasks":["Review"]})).await?;
/// # Ok(()) }
/// ```
pub async fn execute<H: Host>(params: Value) -> Result<ToolResult> {
    let tasks = match parse_tasks(&params) {
        Ok(tasks) => tasks,
        Err(error) => return Ok(error),
    };
    let config = ExecutionConfig::from_params(&params);
    tracing::info!(task_count = tasks.len(), concurrency = config.concurrency,
        strategy = %config.aggregation_strategy, "Starting swarm execution");
    let mut observer = H::Observer::begin(&tasks, config.max_steps);
    let selected = match H::load(config.model.as_deref(), &mut observer).await? {
        Ok(selected) => selected,
        Err(error) => return Ok(error),
    };
    tracing::info!(requested_model = ?selected.model.requested_model,
        resolved_provider = %selected.model.resolved_provider,
        resolved_model = %selected.model.resolved_model, "Using provider for swarm");
    dispatch::execute::<H>(params, tasks, config, observer, selected).await
}
