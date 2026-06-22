//! Surfaces async sub-agent completions into the parent's agent loop.
//!
//! Sub-agents publish `TaskUpdate` envelopes on `agent.<name>` topics, but the
//! parent loop only reasons over its own tool returns. This drains *terminal*
//! sub-agent updates (Completed / Failed) that arrived since the last check and
//! formats them as a notice, so a parent that spawned work asynchronously is
//! told when each sub-agent settles instead of staying blind.

use std::collections::HashSet;

use crate::a2a::types::TaskState;
use crate::bus::{BusMessage, global};
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;

/// Tracks which terminal sub-agent updates the parent has already been shown.
#[derive(Default)]
pub(super) struct SubAgentWatch {
    seen: HashSet<String>,
}

impl SubAgentWatch {
    /// Drain settled sub-agent notices and append them to `session` as a user
    /// message so the parent's next step can act on them. No-op when nothing
    /// new has settled.
    pub(super) fn inject(&mut self, session: &mut Session) {
        if let Some(notice) = self.drain_notice() {
            tracing::info!("Surfacing settled sub-agent updates to parent loop");
            session.add_message(Message {
                role: Role::User,
                content: vec![ContentPart::Text { text: notice }],
            });
        }
    }

    /// Return a notice describing newly-settled sub-agents, or `None`.
    pub(super) fn drain_notice(&mut self) -> Option<String> {
        let bus = global()?;
        let mut lines = Vec::new();
        for env in bus.recorder.recent(1024, Some("agent.")) {
            if let BusMessage::TaskUpdate {
                task_id,
                state,
                message,
            } = env.message
                && state.is_terminal()
                && self.seen.insert(format!("{}|{}", env.id, task_id))
            {
                let verb = match state {
                    TaskState::Completed => "completed",
                    TaskState::Failed => "failed",
                    TaskState::Cancelled => "cancelled",
                    _ => "settled",
                };
                let summary = message.unwrap_or_default();
                lines.push(format!("- @{task_id} {verb}: {summary}"));
            }
        }
        if lines.is_empty() {
            return None;
        }
        Some(format!(
            "Sub-agent status update (you may act on these):\n{}",
            lines.join("\n")
        ))
    }
}
