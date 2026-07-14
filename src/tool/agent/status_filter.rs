//! Pure filtering of lifecycle events to child sessions owned by a caller.

use std::collections::HashMap;

use super::AgentStatus;
use crate::bus::{BusEnvelope, BusMessage};

pub(super) fn collect(
    envelopes: Vec<BusEnvelope>,
    owners: &HashMap<String, String>,
) -> HashMap<String, AgentStatus> {
    let mut out = HashMap::new();
    for envelope in envelopes {
        let BusMessage::TaskUpdate {
            task_id,
            state,
            message,
        } = envelope.message
        else {
            continue;
        };
        let Some(agent) = owners.get(&task_id) else {
            continue;
        };
        if out
            .get(agent)
            .is_none_or(|status: &AgentStatus| envelope.timestamp >= status.at)
        {
            out.insert(
                agent.clone(),
                AgentStatus {
                    state,
                    summary: message,
                    at: envelope.timestamp,
                },
            );
        }
    }
    out
}

#[cfg(test)]
#[path = "status_filter_tests.rs"]
mod tests;
