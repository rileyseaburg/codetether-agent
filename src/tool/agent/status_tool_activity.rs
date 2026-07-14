//! Liveness derived from a sub-agent's tool activity.
//!
//! The primary status source ([`super::status_source`]) only reads
//! [`TaskUpdate`](crate::a2a::types::TaskState) envelopes. A working sub-agent
//! that has not yet published a `TaskUpdate` still emits `ToolRequest` /
//! `ToolResponse` envelopes tagged with its `agent_id`. This module surfaces
//! those as a `Working` signal so the agent is not misreported as
//! `no_activity` while it is busy (see issue #295).

use std::collections::HashMap;

use chrono::{DateTime, Utc};

use super::status_source::AgentStatus;
use crate::a2a::types::TaskState;
use crate::bus::{BusMessage, global};

#[path = "status_tool_filter.rs"]
mod status_tool_filter;

/// Newest tool-activity timestamp per agent id from recent bus history.
fn tool_activity() -> HashMap<String, DateTime<Utc>> {
    let mut out: HashMap<String, DateTime<Utc>> = HashMap::new();
    let Some(bus) = global() else {
        return out;
    };
    for env in bus.recorder.recent(1024, Some("agent.")) {
        let agent = match &env.message {
            BusMessage::ToolRequest { agent_id, .. } => agent_id.clone(),
            BusMessage::ToolResponse { agent_id, .. } => agent_id.clone(),
            _ => continue,
        };
        let newer = out.get(&agent).is_none_or(|p| env.timestamp >= *p);
        if newer {
            out.insert(agent, env.timestamp);
        }
    }
    out
}

/// Insert a `Working` status for agents with tool activity but no `TaskUpdate`.
/// Add recent tool activity for children owned by the current parent.
pub(super) fn backfill<'a>(
    out: &mut HashMap<String, AgentStatus>,
    allowed: impl Iterator<Item = &'a String>,
) {
    merge(out, status_tool_filter::owned(tool_activity(), allowed));
}

/// Pure merge: activity overrides `out` only when strictly newer.
fn merge(out: &mut HashMap<String, AgentStatus>, activity: HashMap<String, DateTime<Utc>>) {
    for (agent, at) in activity {
        if out.get(&agent).is_none_or(|p| at > p.at) {
            out.insert(
                agent,
                AgentStatus {
                    state: TaskState::Working,
                    summary: None,
                    at,
                },
            );
        }
    }
}

#[cfg(test)]
#[path = "status_tool_activity_tests.rs"]
mod tests;
