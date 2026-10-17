//! Merge tool activity only when strictly newer than the known task state.
use crate::status_source::AgentStatus;
use chrono::{DateTime, Utc};
use codetether_a2a_core::TaskState;
use std::collections::HashMap;

/// Pure merge: activity overrides `out` only when strictly newer.
pub(crate) fn merge(
    out: &mut HashMap<String, AgentStatus>,
    activity: HashMap<String, DateTime<Utc>>,
) {
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
