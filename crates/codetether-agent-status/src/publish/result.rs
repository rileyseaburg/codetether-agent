//! Parent-directed publication of a completed child response.
use crate::PublishHost;
use codetether_a2a_core::Part;
use codetether_bus_core::global::global;

/// Deliver the completed child response directly to its owning parent session.
/// # Arguments
/// * `agent_id` — Child live-registry key and sender ID.
/// * `response` — Completed response text.
/// * `error` — Optional terminal error.
/// # Returns
/// Unit; absent bus, child or owner produces no publication.
/// # Examples
/// Use an application [PublishHost] implementation.
pub fn announce_result<H: PublishHost>(agent_id: &str, response: &str, error: Option<&str>) {
    let (Some(bus), Some(entry)) = (global(), H::get(agent_id)) else {
        return;
    };
    let Some(parent) = entry.owner_session_id.as_deref() else {
        return;
    };
    let text = crate::result_message(&entry.name, response, error);
    bus.handle(agent_id)
        .send_to_agent(parent, vec![Part::Text { text }]);
}
