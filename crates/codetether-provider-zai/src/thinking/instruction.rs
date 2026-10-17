//! instruction for Z.AI thinking requests.
use crate::{ContentPart, Message};
pub(super) fn contains_explicit_short_answer_instruction(message: &Message) -> bool {
    message.content.iter().any(|part| match part {
        ContentPart::Text { text } => {
            let lower = text.to_ascii_lowercase();
            lower.contains("reply with exactly")
                || lower.contains("say only")
                || lower.contains("respond with exactly")
                || lower.contains("return exactly")
        }
        _ => false,
    })
}
