//! Reads the latest sub-agent `TaskState` from the global bus recorder.
//!
//! Sub-agents publish `TaskUpdate` envelopes on `agent.<name>` topics. The
//! parent agent has no live subscription, so this scans the recorder's recent
//! history and keeps the newest update per task id.

use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::a2a::types::TaskState;
use crate::bus::{global, BusMessage};

/// Latest known status for one sub-agent.
#[derive(Clone)]
pub(super) struct AgentStatus {
    pub state: TaskState,
    pub summary: Option<String>,
    pub at: DateTime<Utc>,
}

/// Return the newest `TaskUpdate` per agent name from recent bus history.
///
/// Returns an empty map when no global bus is installed.
pub(super) fn latest_states() -> HashMap<String, AgentStatus> {
    let mut out: HashMap<String, AgentStatus> = HashMap::new();
    let Some(bus) = global() else {
        return out;
    };
    for env in bus.recorder.recent(1024, Some("agent.")) {
        if let BusMessage::TaskUpdate {
            task_id,
            state,
            message,
        } = env.message
        {
            let newer = out.get(&task_id).is_none_or(|p| env.timestamp >= p.at);
            if newer {
                out.insert(
                    task_id,
                    AgentStatus {
                        state,
                        summary: message,
                        at: env.timestamp,
                    },
                );
            }
        }
    }
    out
}
