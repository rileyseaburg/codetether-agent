//! Application session binding for shared live-agent metadata.
use crate::session::Session;
pub(in crate::tool::agent) type AgentEntry = codetether_agent_registry::AgentEntry<Session>;
impl codetether_agent_registry::Session for Session {
    fn id(&self) -> &str {
        &self.id
    }
}
