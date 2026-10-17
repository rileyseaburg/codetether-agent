//! Newest observed tool activity from existing bus history.
use chrono::{DateTime, Utc};
use codetether_bus_core::{BusMessage, global::global};
use std::collections::HashMap;

/// Newest tool-activity timestamp per agent id from recent bus history.
pub(crate) fn tool_activity() -> HashMap<String, DateTime<Utc>> {
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
