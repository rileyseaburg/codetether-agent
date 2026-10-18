//! Borrow native session history, accounting and access policy.
use super::*;
impl codetether_agent_runtime::SessionView for Native {
    type Session = Session;
    fn directory(session: &Session) -> Option<&Path> {
        session.metadata.directory.as_deref()
    }
    fn messages(session: &Session) -> &[Message] {
        &session.messages
    }
    fn tool_uses(session: &Session) -> &[ToolUse] {
        &session.tool_uses
    }
    fn usage(session: &Session) -> &Usage {
        &session.usage
    }
    fn available(session: &Session, name: &str) -> bool {
        crate::session::helper::runtime::prior_context_tool_available_for_session(session, name)
    }
    fn prior_blocked(session: &Session, name: &str, args: &str) -> Option<ToolResult> {
        crate::session::helper::runtime::block_serialized_prior_context_for_session(
            session, name, args,
        )
    }
}
