//! Admission policy for the formatted-message cache.

use std::mem::size_of;

use ratatui::text::{Line, Span};

use super::Key;
use crate::tui::chat::message::ChatMessage;

pub(super) const BYTE_LIMIT: usize = 2 * 1024 * 1024;

pub(super) fn key(message: &ChatMessage, role: &str, width: usize) -> Key {
    let timestamp = message
        .timestamp
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    (
        timestamp,
        message.content.as_ptr() as usize,
        message.content.len(),
        width,
        role_discrim(role),
    )
}

pub(super) fn weight(lines: &[Line<'static>], line_capacity: usize) -> usize {
    let line_bytes = line_capacity * size_of::<Line<'static>>();
    lines.iter().fold(line_bytes, |total, line| {
        let spans = line.spans.capacity() * size_of::<Span<'static>>();
        let text = line
            .spans
            .iter()
            .map(|span| span.content.len())
            .sum::<usize>();
        total.saturating_add(spans).saturating_add(text)
    })
}

fn role_discrim(role: &str) -> u8 {
    match role {
        "user" => 1,
        "assistant" => 2,
        "system" | "developer" => 3,
        "error" => 4,
        _ => 0,
    }
}

#[cfg(test)]
#[path = "policy_tests.rs"]
mod tests;
