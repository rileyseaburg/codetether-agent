//! Convert parsed textual calls into provider tool-call parts with fresh IDs.
use codetether_provider_core::ContentPart;
use uuid::Uuid;
pub(super) fn append(rewritten: &mut Vec<ContentPart>, parsed_calls: Vec<(String, String)>) {
    for (name, arguments) in parsed_calls {
        rewritten.push(ContentPart::ToolCall {
            id: Uuid::new_v4().to_string(),
            name,
            arguments,
            thought_signature: None,
        });
    }
}
