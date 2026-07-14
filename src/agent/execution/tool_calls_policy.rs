//! Prior-context policy checks for agent tool-call batches.

use crate::provider::ContentPart;
use crate::session::Session;
use crate::tool::ToolResult;
use crate::tool::readonly::is_read_only;

/// Return a structured denial when a call violates session access policy.
pub(super) fn blocked(session: &Session, part: &ContentPart) -> Option<ToolResult> {
    let call = part.as_tool_call()?;
    crate::session::helper::runtime::block_serialized_prior_context_for_session(
        session,
        call.name,
        call.arguments,
    )
}

/// Allow parallel dispatch only when every call is read-only and permitted.
pub(super) fn can_parallelize(session: &Session, calls: &[ContentPart]) -> bool {
    calls.len() > 1
        && calls.iter().all(|part| {
            part.as_tool_call()
                .is_some_and(|call| is_read_only(call.name) && blocked(session, part).is_none())
        })
}
