//! Shared child-result notices bound to the native session.
pub(in crate::session::helper) use codetether_agent_messages::notice::SubAgentWatch;
codetether_agent_messages::impl_session_notices!(crate::session::Session);
