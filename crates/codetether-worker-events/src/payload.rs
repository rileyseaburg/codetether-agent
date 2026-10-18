//! Select bus events and construct their worker-observability payload.
use codetether_bus_core::{BusEnvelope, BusMessage};
pub(crate) fn payload(worker_id: &str, envelope: &BusEnvelope) -> Option<serde_json::Value> {
    Some(match &envelope.message {
        BusMessage::AgentReady {
            agent_id,
            capabilities,
        } => {
            serde_json::json!({
                "type": "agent_ready",
                "worker_id": worker_id,
                "agent_id": agent_id,
                "capabilities": capabilities,
            })
        }
        BusMessage::TaskUpdate {
            task_id,
            state,
            message,
        } => {
            serde_json::json!({
                "type": "task_update",
                "worker_id": worker_id,
                "task_id": task_id,
                "state": format!("{:?}", state),
                "message": message,
            })
        }
        BusMessage::AgentMessage { from, to, parts } => {
            crate::agent_payload::payload(worker_id, from, to, parts)
        }
        // Skip other event types for now
        _ => return None,
    })
}
