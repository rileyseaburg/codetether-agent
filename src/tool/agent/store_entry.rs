//! Application session binding for shared live-agent metadata.
use crate::session::Session;
pub(in crate::tool::agent) type AgentEntry = codetether_agent_registry::AgentEntry<Session>;
codetether_agent_registry::impl_session_identity!(Session);
