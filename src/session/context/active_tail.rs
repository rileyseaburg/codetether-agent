//! Active-task tail selection for context compaction.

use crate::provider::{Message, Role};

#[path = "active_tail/continuation.rs"]
mod continuation;

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
            matches!(msg.role, Role::User) && (!substantive || !continuation::is_short(msg))
        })
        .map(|(idx, _)| idx)
}
