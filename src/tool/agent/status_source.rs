//! Reads the latest sub-agent `TaskState` from the global bus recorder.
//!
//! Sub-agents publish `TaskUpdate` envelopes on session-unique task topics. The
//! parent agent has no live subscription, so this scans the recorder's recent
//! history and keeps the newest update per task id.

use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::a2a::types::TaskState;
use crate::bus::global;

#[path = "status_filter.rs"]
mod status_filter;

/// Latest known status for one sub-agent.
#[derive(Clone)]
pub(super) struct AgentStatus {
    pub state: TaskState,
    pub summary: Option<String>,
    pub at: DateTime<Utc>,
}

/// Return the newest owned `TaskUpdate` per agent name from recent bus history.
///
/// Agents seen only via tool activity (no `TaskUpdate` yet) are backfilled
/// with a `Working` state so they are not misreported as idle (issue #295).
///
/// Returns an empty map when no global bus is installed.
pub(super) fn latest_states(owners: &HashMap<String, String>) -> HashMap<String, AgentStatus> {
    let Some(bus) = global() else {
        return HashMap::new();
    };
    let mut statuses = status_filter::collect(bus.recorder.recent(1024, Some("task.")), owners);
    super::status_tool_activity::backfill(&mut statuses, owners.values());
    statuses
}
