//! Runtime settings parsed from a `swarm_execute` tool invocation.

use super::model_request;
use serde_json::Value;

/// Effective execution settings with invocation defaults applied.
/// # Examples
/// ```
/// let config = codetether_swarm_tool::execution_config::ExecutionConfig::from_params(&serde_json::json!({}));
/// assert_eq!(config.concurrency, 5);
/// ```
pub struct ExecutionConfig {
    /// Opt into isolated worktrees instead of one shared writable workspace.
    pub isolate_worktrees: bool,
    /// Concurrent worker limit, clamped to 1 through 20.
    pub concurrency: usize,
    /// Requested failure aggregation strategy.
    pub aggregation_strategy: String,
    /// Explicit or inherited model reference.
    pub model: Option<String>,
    /// Per-worker step budget.
    pub max_steps: usize,
    /// Per-worker timeout in seconds.
    pub timeout_secs: u64,
}

impl ExecutionConfig {
    /// Apply defaults and clamp concurrency while preserving requested budgets.
    pub fn from_params(params: &Value) -> Self {
        Self {
            isolate_worktrees: params
                .get("isolate_worktrees")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            concurrency: usize_value(params, "concurrency_limit", 5).clamp(1, 20),
            aggregation_strategy: params
                .get("aggregation_strategy")
                .and_then(Value::as_str)
                .unwrap_or("best_effort")
                .to_string(),
            model: model_request::requested(params).map(String::from),
            max_steps: usize_value(params, "max_steps", 50),
            timeout_secs: u64_value(params, "timeout_secs", 300),
        }
    }
}

fn usize_value(params: &Value, key: &str, default: usize) -> usize {
    u64_value(params, key, default as u64)
        .try_into()
        .unwrap_or(usize::MAX)
}

fn u64_value(params: &Value, key: &str, default: u64) -> u64 {
    params.get(key).and_then(Value::as_u64).unwrap_or(default)
}
