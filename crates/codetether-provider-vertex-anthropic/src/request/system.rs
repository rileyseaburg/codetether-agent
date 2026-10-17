//! system content conversion.
use crate::*;
pub(super) fn push(msg: &Message, system_blocks: &mut Vec<Value>, api_messages: &mut Vec<Value>) {
    for part in &msg.content {
        match part {
            ContentPart::Text { text } => {
                system_blocks.push(json!({
                    "type": "text",
                    "text": text,
                }));
            }
            ContentPart::Thinking { text, .. } => {
                system_blocks.push(json!({
                    "type": "thinking",
                    "thinking": text,
                }));
            }
            _ => {}
        }
    }
}
