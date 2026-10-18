//! Initialize the durable baseline only for user-role messages.
use codetether_provider_core::{Message, Role};
use codetether_session_metadata::SessionMetadata;

pub(super) fn prepare(
    metadata: &mut SessionMetadata,
    messages: &[Message],
    message: &Message,
) -> bool {
    if message.role != Role::User {
        return false;
    }
    if metadata.prior_context_allowed.is_none() {
        metadata.prior_context_allowed =
            Some(codetether_prior_context_policy::latest_persistent(messages).unwrap_or(true));
    }
    true
}
