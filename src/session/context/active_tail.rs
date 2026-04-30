//! Active-task tail selection for context compaction.

use crate::provider::{ContentPart, Message, Role};

const SHORT_CONTINUATION_CHARS: usize = 80;

pub(crate) fn active_tail_start(messages: &[Message], keep_last: usize) -> usize {
    let recent = messages.len().saturating_sub(keep_last);
    active_user_tail_start(messages, keep_last).unwrap_or(recent)
}

pub(crate) fn active_user_tail_start(messages: &[Message], keep_last: usize) -> Option<usize> {
    let recent = messages.len().saturating_sub(keep_last);
    user_anchor(messages, true)
        .or_else(|| user_anchor(messages, false))
        .map(|idx| idx.min(recent))
}

fn user_anchor(messages: &[Message], substantive: bool) -> Option<usize> {
    messages
        .iter()
        .enumerate()
        .rev()
        .find(|(_, msg)| {
            matches!(msg.role, Role::User) && (!substantive || !is_short_continuation(msg))
        })
        .map(|(idx, _)| idx)
}

fn is_short_continuation(msg: &Message) -> bool {
    let text = msg
        .content
        .iter()
        .find_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .unwrap_or("");
    let normalized = text.trim().to_ascii_lowercase();
    if normalized.len() > SHORT_CONTINUATION_CHARS {
        return false;
    }
    let trimmed = normalized.trim_matches(|c: char| c.is_ascii_punctuation());
    matches!(
        trimmed,
        "continue"
            | "status"
            | "resume"
            | "go on"
            | "keep going"
            | "hey"
            | "ok"
            | "okay"
            | "yes"
            | "y"
    )
}
