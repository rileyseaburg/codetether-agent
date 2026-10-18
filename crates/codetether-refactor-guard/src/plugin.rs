//! Optional repository hook discovery, invocation contract, and result policy.
use crate::{GuardReport, GuardViolation};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};
use std::{future::Future, path::Path};

/// Runtime execution supplied by a client that supports refactor-guard plugins.
/// # Implementors
/// Native clients execute the prepared arguments with their rooted plugin tool.
pub trait Runtime {
    /// Execute the hook under the supplied workspace root.
    /// # Errors
    /// Propagates the plugin runtime's execution failures.
    fn execute(root: &Path, args: Value) -> impl Future<Output = Result<ToolResult>> + Send;
}

/// Apply an optional workspace refactor-guard plugin to the current report.
/// # Arguments
/// * `root` — Workspace whose `.codetether/refactor_guard.tether` hook is checked.
/// * `report` — Serializable metrics supplied to the `should_continue` hook.
/// # Returns
/// An optional violation from execution failure or the hook's returned decision.
/// Missing hooks return `None` without invoking the runtime.
/// # Errors
/// Propagates report serialization and plugin-runtime errors.
/// # Examples
/// ```no_run
/// # async fn example<R: codetether_refactor_guard::plugin::Runtime>(root: &std::path::Path, report: &codetether_refactor_guard::GuardReport) -> anyhow::Result<()> {
/// let _violation = codetether_refactor_guard::plugin::apply::<R>(root, report).await?;
/// # Ok(()) }
/// ```
pub async fn apply<R: Runtime>(
    root: &Path,
    report: &GuardReport,
) -> Result<Option<GuardViolation>> {
    let hook = ".codetether/refactor_guard.tether";
    if !root.join(hook).exists() {
        return Ok(None);
    }
    let args = json!({
        "path": hook,
        "hook": "should_continue",
        "args": [serde_json::to_string(report)?],
        "timeout_secs": 2
    });
    let result = R::execute(root, args).await?;
    if !result.success {
        return Ok(Some(GuardViolation::new(hook, result.output)));
    }
    Ok(crate::read_decision(
        &result.metadata.get("value").cloned().unwrap_or(Value::Null),
    ))
}
