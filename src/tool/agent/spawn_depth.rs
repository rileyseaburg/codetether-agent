//! Nesting-depth guard for model-spawned sub-agents.

use super::super::{spawn_request::SpawnRequest, store};
use crate::{config::Config, tool::ToolResult};

pub(super) async fn validate(request: &SpawnRequest<'_>) -> Result<(), ToolResult> {
    let (_, depth) = store::lineage_for_session(request.parent_session_id);
    let limit = Config::load()
        .await
        .map(|config| config.agents.max_depth())
        .map_err(|error| ToolResult::error(format!("cannot load agent depth limit: {error}")))?;
    if depth <= limit {
        return Ok(());
    }
    Err(ToolResult::error(format!(
        "cannot spawn @{} at depth {depth}; max depth is {limit}",
        request.name
    )))
}
