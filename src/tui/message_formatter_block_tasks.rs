//! Task-list checkboxes (depth-aware bullets live in `bullets`).

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use super::Block;

#[path = "message_formatter_block_bullets.rs"]
mod bullets;
#[cfg(test)]
pub(super) use bullets::bullet_for_depth;
pub(super) use bullets::list_block;

/// Detect a GFM task item `- [ ] text` / `- [x] text`.
///
/// Returns `(checked, rest)` where `checked` is true for `[x]`/`[X]`.
pub(super) fn task_item(t: &str) -> Option<(bool, String)> {
    for m in ["- ", "* ", "+ "] {
        if let Some(after) = t.strip_prefix(m) {
            if let Some(rest) = after.strip_prefix("[ ] ") {
                return Some((false, rest.to_string()));
            }
            for x in ["[x] ", "[X] "] {
                if let Some(rest) = after.strip_prefix(x) {
                    return Some((true, rest.to_string()));
                }
            }
        }
    }
    None
}

/// Render a task item as a styled block (☐ / ☑ with strikethrough when done).
pub(super) fn task_block(indent: &str, checked: bool, rest: String) -> Block {
    let (glyph, color) = if checked {
        ("☑ ", Color::Green)
    } else {
        ("☐ ", Color::Yellow)
    };
    let prefix = vec![Span::styled(
        format!("{indent}{glyph}"),
        Style::default().fg(color),
    )];
    if checked {
        let done = Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::CROSSED_OUT);
        return Block::Styled([prefix, vec![Span::styled(rest, done)]].concat());
    }
    Block::Prefixed { prefix, rest }
}
