//! Orphan-toolResult detection for the Bedrock pairing invariant.

use serde_json::Value;

pub(super) fn detect(messages: &[Value]) -> Option<String> {
    for (index, message) in messages.iter().enumerate() {
        let results = super::ids_of(message, "/toolResult/toolUseId");
        if results.is_empty() {
            continue;
        }
        let declared = index
            .checked_sub(1)
            .and_then(|prev| messages.get(prev))
            .map(|prev| super::ids_of(prev, "/toolUse/toolUseId"))
            .unwrap_or_default();
        if let Some(extra) = results.iter().find(|id| !declared.contains(id)) {
            return Some(format!("message {index} has orphan toolResult {extra}"));
        }
    }
    None
}
