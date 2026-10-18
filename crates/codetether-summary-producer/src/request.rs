//! Preserve the summary request text and transcript projection.
use codetether_provider_core::Message;
pub(super) fn format(slice: &[Message], target_tokens: usize) -> String {
    let context = codetether_context_policy::transcript::messages_to_rlm_context(slice);
    let input = format!(
        "Summarise the following transcript window (~{} messages, target ≤ {} tokens). \
         Preserve key decisions, file paths, error classes, and tool outcomes.\n\n{context}",
        slice.len(),
        target_tokens,
    );
    input
}
