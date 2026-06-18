//! Block classification: maps a line to a [`Block`] variant.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

#[path = "message_formatter_block_detect_helpers.rs"]
mod helpers;
use helpers::{blockquote, header, header_spans, is_rule, list_item};

#[path = "message_formatter_block_tasks.rs"]
mod tasks;
use tasks::{list_block, task_block, task_item};

#[cfg(test)]
#[path = "message_formatter_block_tasks_tests.rs"]
mod tasks_tests;

/// Outcome of block-level detection for one line.
pub(in crate::tui) enum Block {
    Plain,
    Styled(Vec<Span<'static>>),
    Prefixed {
        prefix: Vec<Span<'static>>,
        rest: String,
    },
}

pub(in crate::tui) fn classify(line: &str) -> Block {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];

    if is_rule(trimmed) {
        return Block::Styled(vec![Span::styled(
            "─".repeat(24),
            Style::default().fg(Color::DarkGray),
        )]);
    }
    if let Some((hashes, text)) = header(trimmed) {
        return Block::Styled(header_spans(hashes, &text));
    }
    if let Some(rest) = blockquote(trimmed) {
        let style = Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC);
        return Block::Prefixed {
            prefix: vec![Span::styled("▏ ", style)],
            rest,
        };
    }
    if let Some((checked, rest)) = task_item(trimmed) {
        return task_block(indent, checked, rest);
    }
    if let Some((marker, rest)) = list_item(trimmed) {
        return list_block(indent, marker, rest);
    }
    Block::Plain
}
