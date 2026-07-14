//! Inject completed, parent-addressed sub-agent results into an agent loop.

use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use std::collections::HashSet;

#[path = "prompt_subagent_notice.rs"]
mod notice;

#[derive(Default)]
pub(in crate::session::helper) struct SubAgentWatch {
    seen: HashSet<String>,
}

impl SubAgentWatch {
    pub(in crate::session::helper) fn inject(&mut self, session: &mut Session) {
        let Some(text) = notice::collect(session, &mut self.seen) else {
            return;
        };
        tracing::info!(session_id = %session.id, "Surfacing sub-agent result to parent loop");
        session.add_delegated_message(Message {
            role: Role::User,
            content: vec![ContentPart::Text { text }],
        });
    }
}
