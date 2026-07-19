//! Observable dispatch of requests to discovered A2A peers.

#[path = "request/detached.rs"]
mod detached;
#[path = "request/run.rs"]
mod run;

use super::observation;
use crate::tool::ToolResult;
use anyhow::Result;

pub(super) async fn execute(
    name: &str,
    text: &str,
    context_id: Option<&str>,
    parent_session_id: Option<&str>,
    detach: bool,
) -> Result<ToolResult> {
    let Some(route) = crate::a2a::peer_route::get(name) else {
        return Ok(ToolResult::error(format!("Agent {name} not found")));
    };
    let turn = observation::begin(name, parent_session_id, text);
    if detach {
        return Ok(detached::spawn(name, text, context_id, route, turn));
    }
    run::execute(name, text, context_id, route, turn).await
}

#[cfg(test)]
#[path = "request_tests.rs"]
mod tests;
