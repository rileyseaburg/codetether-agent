//! Removal of byte-identical repeated high-priority messages.

use crate::provider::{Message, Role};
use std::collections::HashMap;

pub(super) fn system_messages(
    messages: &[Message],
    rendered: Vec<String>,
) -> (Vec<Message>, Vec<String>) {
    let mut latest = HashMap::new();
    for (index, (message, text)) in messages.iter().zip(&rendered).enumerate() {
        if matches!(message.role, Role::System | Role::Developer) {
            latest.insert(text.clone(), index);
        }
    }
    messages
        .iter()
        .cloned()
        .zip(rendered)
        .enumerate()
        .filter(|(index, (message, text))| {
            !matches!(message.role, Role::System | Role::Developer)
                || latest.get(text.as_str()) == Some(index)
        })
        .map(|(_, pair)| pair)
        .unzip()
}
