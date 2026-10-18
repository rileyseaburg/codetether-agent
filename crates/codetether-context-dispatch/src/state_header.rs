//! Attach persisted and live pinned constraints after derivation.
use codetether_provider_core::Message;

pub fn prepend_state_header<H: crate::Source>(session: &H, messages: &mut Vec<Message>) {
    codetether_context_policy::state_header::prepend_state_header(
        session.constraint_prefix(),
        codetether_incremental_derivation::Source::pages(session),
        codetether_context_legacy::Source::messages(session),
        || session.message_offset(),
        messages,
    );
}
