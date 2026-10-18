//! Bind application server state to shared task-message publication.
use super::*;

pub(super) fn emit_a2a_inbound<H: Application>(
    server: &A2AServer<H>,
    task_id: &str,
    message: &Message,
) {
    codetether_bus_core::a2a_messages::inbound(
        server.bus.as_deref(),
        &server.agent_card.name,
        task_id,
        message,
    );
}

pub(super) fn emit_a2a_outbound<H: Application>(
    server: &A2AServer<H>,
    task_id: &str,
    message: &Message,
) {
    codetether_bus_core::a2a_messages::outbound(
        server.bus.as_deref(),
        &server.agent_card.name,
        task_id,
        message,
    );
}
