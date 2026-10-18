//! Attach dropped-range navigation while installing the compressed prefix.
use codetether_provider_core::Message;
pub(super) fn summary(
    messages: &mut Vec<Message>,
    tail: Vec<Message>,
    summary: String,
    prefix: &[Message],
) {
    let toc = codetether_context_policy::compression::summary::toc::render_toc(prefix, 0);
    codetether_context_policy::compression::summary::install(messages, tail, summary, &toc);
}
