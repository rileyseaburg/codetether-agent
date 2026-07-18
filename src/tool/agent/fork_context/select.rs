//! Selection of complete parent turns and interrupted-turn markers.

use super::ForkTurns;
use crate::provider::{Message, Role};

pub(super) fn run(messages: &[Message], scope: ForkTurns) -> Vec<Message> {
    let end = messages
        .iter()
        .rposition(|message| message.role == Role::User)
        .map_or(0, |index| index + 1);
    let start = match scope {
        ForkTurns::None => end,
        ForkTurns::All => 0,
        ForkTurns::Count(count) => messages[..end]
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, message)| message.role == Role::User)
            .nth(count.saturating_sub(1))
            .map_or(0, |(index, _)| index),
    };
    let mut selected = messages[start..end]
        .iter()
        .filter(|message| message.role != Role::System)
        .cloned()
        .collect::<Vec<_>>();
    selected.extend(
        messages[end..]
            .iter()
            .filter(|message| message.role == Role::Developer)
            .cloned(),
    );
    selected
}
