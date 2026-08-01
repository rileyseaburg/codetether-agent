//! Runtime discovery and activation of compact-profile tools.

use super::super::Runner;
use super::call::Call;
use crate::provider::ToolDefinition;
use anyhow::Result;

/// Handles the virtual `list_tools` call.
///
/// A request for one exact tool both returns its schema and adds that schema to
/// the next provider request. This keeps the default catalog compact without
/// making the rest of the executable registry unreachable.
pub(super) async fn run(runner: &mut Runner<'_>, call: &Call) -> Result<bool> {
    promote(
        &runner.model.tools,
        &mut runner.model.advertised,
        &call.input,
    );
    let content = super::super::super::bootstrap::list_tools_bootstrap_output(
        &runner.model.tools,
        &call.input,
    );
    super::simple::record(runner, call, content, true).await;
    Ok(false)
}

fn promote(
    tools: &[ToolDefinition],
    advertised: &mut Vec<ToolDefinition>,
    input: &serde_json::Value,
) {
    let Some(name) = input
        .get("tool")
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|name| !name.is_empty())
    else {
        return;
    };
    if advertised
        .iter()
        .any(|tool| tool.name.eq_ignore_ascii_case(name))
    {
        return;
    }
    if let Some(tool) = tools
        .iter()
        .find(|tool| tool.name.eq_ignore_ascii_case(name))
        .cloned()
    {
        advertised.push(tool);
    }
}

#[cfg(test)]
#[path = "discovery_tests.rs"]
mod tests;
