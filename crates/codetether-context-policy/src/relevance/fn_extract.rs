//! extract policy.
use super::*;

/// Extract [`RelevanceMeta`] for a single chat-history entry.
///
/// Pure and fast: no LLM calls, no IO. Safe to call from
/// `Session::add_message` in the application.
pub fn extract(msg: &Message) -> RelevanceMeta {
    let mut meta = RelevanceMeta::default();
    for part in &msg.content {
        match part {
            ContentPart::Text { text } => {
                append_files(text, &mut meta.files);
                append_error_classes(text, &mut meta.error_classes);
            }
            ContentPart::ToolCall { name, .. } => {
                if !meta.tools.contains(name) {
                    meta.tools.push(name.clone());
                }
            }
            ContentPart::ToolResult { content, .. } => {
                append_error_classes(content, &mut meta.error_classes);
            }
            _ => {}
        }
    }
    dedupe_preserving_order(&mut meta.files);
    dedupe_preserving_order(&mut meta.error_classes);
    meta
}
