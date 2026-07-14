//! Normalization and pre-execution handling of one tool call.

use super::super::Runner;
use anyhow::Result;
use serde_json::Value;

/// Normalized provider tool call ready for policy checks and execution.
pub(super) struct Call {
    /// Provider-supplied call identifier.
    pub id: String,
    /// Canonical executable tool name.
    pub name: String,
    /// Normalized JSON input passed to the tool.
    pub input: Value,
}

impl Call {
    /// Normalizes a parsed provider tool-call tuple.
    pub fn new(id: String, name: String, input: Value) -> Self {
        let (name, input) =
            super::super::super::edit::normalize_tool_call_for_execution(&name, &input);
        Self { id, name, input }
    }
}

/// Runs one normalized tool call through guards, execution, and publication.
///
/// # Errors
///
/// Returns an error when an execution-stage operation cannot complete.
pub(super) async fn run(runner: &mut Runner<'_>, step: usize, call: Call) -> Result<bool> {
    super::call_guard::publish_start(runner, &call).await;
    if call.name == "list_tools" {
        let content = super::super::super::bootstrap::list_tools_bootstrap_output(
            &runner.model.tools,
            &call.input,
        );
        super::simple::record(runner, &call, content, true).await;
        return Ok(false);
    }
    super::bus::request(runner, step, &call);
    if let Some(reason) = super::call_guard::blocked(runner, &call) {
        super::simple::record(runner, &call, format!("Error: {reason}"), false).await;
        return Ok(false);
    }
    let outcome = super::invoke::execute(runner, &call).await;
    super::publish::complete(runner, step, &call, outcome).await;
    Ok(super::codesearch::guard(runner, step))
}
