//! User-turn boundary selection and oversized-tail fallback.

use crate::provider::{Message, Role};

pub(super) fn starts(messages: &[Message]) -> Vec<usize> {
    let mut starts = (0..messages.len())
        .filter(|index| messages[*index].role == Role::User)
        .collect::<Vec<_>>();
    starts.push(messages.len());
    starts
}

pub(super) fn latest_user(rendered: &[String], starts: &[usize], limit: usize) -> Vec<String> {
    let index = starts
        .iter()
        .rev()
        .nth(1)
        .copied()
        .unwrap_or(rendered.len() - 1);
    vec![crate::provider::util::truncate_bytes_safe(&rendered[index], limit).to_string()]
}
