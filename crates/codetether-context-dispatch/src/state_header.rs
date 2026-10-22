//! Attach persisted and live pinned constraints after derivation.
use codetether_provider_core::Message;

pub fn prepend_state_header<H: crate::Source>(session: &H, messages: &mut Vec<Message>) {
    codetether_context_policy::state_header::prepend_state_header_with_corrections(
        session.constraint_prefix(),
        codetether_incremental_derivation::Source::pages(session),
        codetether_context_legacy::Source::messages(session),
        || session.message_offset(),
        session.constraint_corrections(),
        messages,
    );
}

pub(super) fn reserve(system: &str, headers: &[Message]) -> String {
    let mut prompt = system.to_owned();
    for message in headers {
        for part in &message.content {
            if let codetether_provider_core::ContentPart::Text { text } = part {
                prompt.push('\n');
                prompt.push_str(text);
            }
        }
    }
    prompt
}
