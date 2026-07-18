//! Resident and pending sub-agent thread limit configuration.

use crate::{config::Config, tool::ToolResult};

/// Resolve the configured maximum number of resident or starting children.
pub(in crate::tool::agent) async fn limit() -> Result<usize, ToolResult> {
    Config::load()
        .await
        .map(|config| config.agents.max_threads())
        .map_err(|error| ToolResult::error(format!("cannot load agent thread limit: {error}")))
}

/// Build the structured rejection returned when residency is full.
pub(in crate::tool::agent) fn rejected(usage: usize, limit: usize) -> ToolResult {
    ToolResult::error(format!(
        "cannot open agent: {usage} threads are resident or starting; max is {limit}"
    ))
}
