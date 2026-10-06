//! A task stream accepts only its canonical topic and matching task payload.

use crate::bus::{BusEnvelope, BusMessage};

pub(super) fn matches_task(envelope: &BusEnvelope, task_id: &str) -> bool {
    if envelope.topic != format!("task.{task_id}") {
        return false;
    }
    match &envelope.message {
        BusMessage::TaskUpdate { task_id: id, .. }
        | BusMessage::ArtifactUpdate { task_id: id, .. } => id == task_id,
        _ => false,
    }
}
