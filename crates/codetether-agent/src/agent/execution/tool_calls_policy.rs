//! Compatibility adapter for native session-policy tests.
use crate::{agent::Agent, provider::ContentPart, session::Session, tool::ToolResult};
pub(super) fn blocked(session: &Session, part: &ContentPart) -> Option<ToolResult> {
    codetether_agent_replay::blocked::<Agent>(session, part)
}
