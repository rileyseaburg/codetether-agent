//! Inject completed, parent-addressed sub-agent results into an agent loop.

use super::{Host, collect as notice};
use codetether_provider_core::{ContentPart, Message, Role};
use std::collections::HashSet;

/// Tracks child-result envelopes already inspected by one parent prompt loop.
/// # Examples
/// ```
/// let _watch = codetether_agent_messages::notice::SubAgentWatch::default();
/// ```
#[derive(Default)]
pub struct SubAgentWatch {
    seen: HashSet<String>,
}

impl SubAgentWatch {
    /// Insert unseen parent-addressed results through delegated-message handling.
    pub fn inject(&mut self, session: &mut impl Host) {
        let Some(text) = notice::collect(session, &mut self.seen) else {
            return;
        };
        tracing::info!(session_id = %session.session_id(), "Surfacing sub-agent result to parent loop");
        session.add_delegated_message(Message {
            role: Role::User,
            content: vec![ContentPart::Text { text }],
        });
    }
}
