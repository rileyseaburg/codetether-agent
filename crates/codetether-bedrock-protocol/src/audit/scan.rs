//! Tool-use identifier extraction from Converse content blocks.

use serde_json::Value;

/// Ids declared by `pointer` in one message's content blocks.
pub fn ids_at(message: &Value, pointer: &str) -> Vec<String> {
    message
        .get("content")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|part| part.pointer(pointer)?.as_str())
        .map(String::from)
        .collect()
}

/// First unpaired `toolUse` in `messages`, as `(index, missing_ids)`.
///
/// # Arguments
/// * `messages` - Ordered Converse messages.
///
/// # Returns
/// The first offending message index and its unanswered tool identifiers.
///
/// # Examples
/// ```
/// assert!(codetether_bedrock_protocol::audit::unpaired(&[]).is_none());
/// ```
pub fn unpaired(messages: &[Value]) -> Option<(usize, Vec<String>)> {
    for (index, message) in messages.iter().enumerate() {
        let declared = ids_at(message, "/toolUse/toolUseId");
        if declared.is_empty() {
            continue;
        }
        let answered = messages
            .get(index + 1)
            .map(|next| ids_at(next, "/toolResult/toolUseId"))
            .unwrap_or_default();
        let missing: Vec<String> = declared
            .into_iter()
            .filter(|id| !answered.contains(id))
            .collect();
        if !missing.is_empty() {
            return Some((index, missing));
        }
    }
    None
}
