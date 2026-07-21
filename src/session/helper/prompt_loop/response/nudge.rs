//! Internal corrective messages for the prompt loop.

use super::super::Runner;
use crate::provider::{ContentPart, Message, Role};

/// Adds developer guidance without attributing it to the end user.
pub(in crate::session::helper::prompt_loop) fn add(runner: &mut Runner<'_>, text: &str) {
    runner.session.add_message(message(text));
}

fn message(text: &str) -> Message {
    Message {
        role: Role::Developer,
        content: vec![ContentPart::Text {
            text: text.to_string(),
        }],
    }
}

#[cfg(test)]
#[path = "nudge_tests.rs"]
mod tests;
