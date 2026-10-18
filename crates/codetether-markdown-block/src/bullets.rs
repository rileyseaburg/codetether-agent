//! List bullet rendering with depth-aware glyphs.

use ratatui::{
    style::{Color, Style},
    text::Span,
};

use super::super::Block;

/// Render a list item with a depth-aware bullet glyph.
pub(crate) fn list_block(indent: &str, marker: String, rest: String) -> Block {
    let bullet = if marker == "•" {
        bullet_for_depth(indent.len())
    } else {
        marker.as_str()
    };
    Block::Prefixed {
        prefix: vec![Span::styled(
            format!("{indent}{bullet} "),
            Style::default().fg(Color::Yellow),
        )],
        rest,
    }
}

/// Choose a bullet glyph by indent depth (every 2 spaces = one level).
pub(crate) fn bullet_for_depth(spaces: usize) -> &'static str {
    match spaces / 2 {
        0 => "•",
        1 => "◦",
        _ => "▪",
    }
}
