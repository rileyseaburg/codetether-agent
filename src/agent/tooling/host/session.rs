//! Bind native session directory, prior-context enrichment and history.
use super::*;
impl codetether_agent_tools::SessionView for Agent {
    type Session = Session;
    fn directory(session: &Session) -> Option<&Path> {
        session.metadata.directory.as_deref()
    }
    fn enrich_input(input: &Value, cwd: &Path, session: &Session) -> Value {
        crate::session::helper::runtime::enrich_tool_input_for_session(input, cwd, session)
    }
    fn messages(session: &Session) -> &[Message] {
        &session.messages
    }
}
