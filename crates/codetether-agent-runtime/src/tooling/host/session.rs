//! Bind native session directory, prior-context enrichment and history.
use super::*;
impl<H: Host> codetether_agent_tools::SessionView for Agent<H> {
    type Session = H::Session;
    fn directory(session: &H::Session) -> Option<&Path> {
        H::directory(session)
    }
    fn enrich_input(input: &Value, cwd: &Path, session: &H::Session) -> Value {
        H::enrich(input, cwd, session)
    }
    fn messages(session: &H::Session) -> &[Message] {
        H::messages(session)
    }
}
