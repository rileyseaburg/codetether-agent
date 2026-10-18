//! send assistant output on the correlated agent bus.
use crate::{AgentBus, BusMessage};
use std::sync::Arc;

pub(super) fn send(
    bus: Option<&Arc<AgentBus>>,
    agent: &str,
    turn_id: &str,
    topic: String,
    message: BusMessage,
) {
    let Some(bus) = bus else {
        return;
    };
    bus.handle(agent)
        .send_with_correlation(topic, message, Some(turn_id.to_string()));
}
