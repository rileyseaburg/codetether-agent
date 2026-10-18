//! Block classification: maps a line to a [`Block`] variant.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

#[path = "helpers.rs"]
mod helpers;
use helpers::{blockquote, header, header_spans, is_rule, list_item};

#[path = "tasks.rs"]
mod tasks;
use tasks::{list_block, task_block, task_item};

#[cfg(test)]
#[path = "tasks_tests.rs"]
mod tasks_tests;
pub use crate::Block;

/// Classify one Markdown line and style its block-level markers.
/// # Arguments
/// * `line` - Raw Markdown line, including any indentation.
/// # Returns
/// Fully styled spans, a styled prefix with residual text, or a plain marker.
/// # Examples
/// ```
/// use codetether_markdown_block::{classify, Block};
/// assert!(matches!(classify("# Title"), Block::Styled(_)));
/// assert!(matches!(classify("- item"), Block::Prefixed { .. }));
/// ```
pub fn classify(line: &str) -> Block {
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
