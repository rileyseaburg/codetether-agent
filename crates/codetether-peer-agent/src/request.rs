//! Observable dispatch of requests to discovered A2A peers.

mod completion;
#[path = "request/detached.rs"]
mod detached;
#[path = "request/run.rs"]
mod run;

use super::observation;
use anyhow::Result;
use codetether_tool_core::ToolResult;

pub(super) async fn execute(
    name: &str,
    text: &str,
    context_id: Option<&str>,
    parent_session_id: Option<&str>,
    detach: bool,
) -> Result<ToolResult> {
    if codetether_a2a_http::inbound_turn::is_waiting(name) {
        return Ok(ToolResult::error(format!(
            "@{name} is already awaiting your A2A response. Return your answer, clarification \
             question, or blocker in this turn's response; it will be delivered directly. \
             Opening another request to the sender can deadlock both conversations."
        )));
    }
    let Some(route) = codetether_a2a_discovery::routes::get(name) else {
        return Ok(ToolResult::error(format!("Agent {name} not found")));
    };
    let turn = observation::begin(name, parent_session_id, text);
    let conversation = parent_session_id.map(|owner| {
        uuid::Uuid::new_v5(
            &uuid::Uuid::NAMESPACE_URL,
            format!("{owner}\0{name}").as_bytes(),
        )
        .to_string()
    });
    let context_id = context_id.or(conversation.as_deref());
    if detach && parent_session_id.is_some() && codetether_bus_core::global::global().is_some() {
        return Ok(detached::spawn(name, text, context_id, route, turn));
    }
    run::execute(name, text, context_id, route, turn).await
}

#[cfg(test)]
#[path = "request_tests.rs"]
mod tests;
