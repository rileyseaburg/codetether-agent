//! proto message to local conversion.
use super::*;

// ═══════════════════════════════════════════════════════════════════════
// Proto → Local
// ═══════════════════════════════════════════════════════════════════════

/// Convert a proto `Message` to a local `Message`.
pub fn proto_message_to_local(msg: &proto::Message) -> local::Message {
    local::Message {
        message_id: msg.message_id.clone(),
        role: match msg.role() {
            proto::Role::User => local::MessageRole::User,
            _ => local::MessageRole::Agent,
        },
        parts: msg.content.iter().filter_map(proto_part_to_local).collect(),
        context_id: if msg.context_id.is_empty() {
            None
        } else {
            Some(msg.context_id.clone())
        },
        task_id: if msg.task_id.is_empty() {
            None
        } else {
            Some(msg.task_id.clone())
        },
        metadata: Default::default(),
        extensions: msg.extensions.clone(),
    }
}
