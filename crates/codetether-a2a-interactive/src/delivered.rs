//! Avoid replaying a queued bus reply consumed during an active turn.
use codetether_provider_core::{ContentPart, Message};
use codetether_worker_events::IncomingTask;

/// Check the generated bus-event marker against the session transcript.
/// # Arguments
/// * `task` — Queued remote input, optionally prefixed by a bus-event marker.
/// * `messages` — Messages retained by the active session.
/// # Returns
/// True only when the same marked event is already present in the transcript.
/// # Examples
/// ```
/// let task = codetether_worker_events::IncomingTask {
///     task_id: "id".into(), message: "ordinary peer request".into(), from_agent: None,
/// };
/// assert!(!codetether_a2a_interactive::already_delivered(&task, &[]));
/// ```
pub fn already_delivered(task: &IncomingTask, messages: &[Message]) -> bool {
    let Some(marker) = task
        .message
        .lines()
        .next()
        .filter(|line| line.starts_with("[sub-agent-event:") && line.ends_with(']'))
    else {
        return false;
    };
    messages
        .iter()
        .flat_map(|message| &message.content)
        .any(|part| matches!(part, ContentPart::Text { text } if text.contains(marker)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use codetether_provider_core::Role;

    #[test]
    fn only_the_same_bus_event_is_suppressed_after_model_injection() {
        let task = IncomingTask {
            task_id: "event-1".into(),
            message: "[sub-agent-event:event-1]\npeer replied".into(),
            from_agent: Some("peer".into()),
        };
        assert!(!already_delivered(&task, &[]));
        let mut messages = vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "[sub-agent-event:event-2]\nolder reply".into(),
            }],
        }];
        assert!(!already_delivered(&task, &messages));
        messages[0].content = vec![ContentPart::Text {
            text: task.message.clone(),
        }];
        assert!(already_delivered(&task, &messages));
    }
}
