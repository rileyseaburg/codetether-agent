//! Native session binding for shared prior-context tool rejection.
use crate::{session::Session, tool::ToolResult};
pub(crate) use codetether_prior_context_access::runtime_context;
#[cfg(test)]
pub(crate) use codetether_prior_context_access::{for_messages, serialized_messages};
pub(crate) fn serialized_session(session: &Session, name: &str, args: &str) -> Option<ToolResult> {
    codetether_prior_context_access::serialized_policy(
        super::session_state::allowed(session),
        name,
        args,
    )
}
