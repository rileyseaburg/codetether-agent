//! Text content extracted without separators for assistant messages.
use crate::{ContentPart, Message};

pub(super) fn text(msg: &Message) -> String {
    let text: String = msg
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("");

    text
}
