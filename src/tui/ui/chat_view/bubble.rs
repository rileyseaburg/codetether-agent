//! Right-aligned user message bubble (`╭─╮` rounded box).
//!
//! Gives user prompts a chat-app feel: content sits in a rounded box
//! pushed to the right edge, visually distinct from the assistant's
//! left-anchored timeline flow (see [`bubble_assistant`]).

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use unicode_width::UnicodeWidthStr;

use crate::tui::ui::gradient::rgb_supported;

#[path = "bubble_mods.rs"]
mod mods;
pub use mods::{assistant_bubble_lines, bubble_assistant, timeline};

const USER_RGB: (u8, u8, u8) = (80, 200, 120);

fn box_color() -> Color {
    if rgb_supported() {
        Color::Rgb(USER_RGB.0, USER_RGB.1, USER_RGB.2)
    } else {
        Color::Green
    }
}

fn dim(color: Color) -> Style {
    Style::default().fg(color).add_modifier(Modifier::DIM)
}

/// Wrap body text rows in a right-aligned rounded user bubble.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::bubble::user_bubble;
/// let lines = user_bubble(&["hello".to_string()], 40);
/// assert_eq!(lines.len(), 3);
/// ```
pub fn user_bubble(body: &[String], panel_width: usize) -> Vec<Line<'static>> {
    let color = box_color();
    let content_w = body.iter().map(|s| s.width()).max().unwrap_or(0);
    let inner = (content_w + 2).min(panel_width.saturating_sub(2)).max(8);
    let mut out = vec![right_pad(border("╭", "╮", inner, color), panel_width)];
    for text in body {
        out.push(right_pad(row(text, inner, color), panel_width));
    }
    out.push(right_pad(border("╰", "╯", inner, color), panel_width));
    out
}

fn border(l: &str, r: &str, inner: usize, color: Color) -> Line<'static> {
    Line::from(Span::styled(
        format!("{l}{}{r}", "─".repeat(inner)),
        dim(color),
    ))
}

fn row(text: &str, inner: usize, color: Color) -> Line<'static> {
    let pad = inner.saturating_sub(text.width() + 1);
    Line::from(vec![
        Span::styled("│", dim(color)),
        Span::raw(format!(" {text}{}", " ".repeat(pad))),
        Span::styled("│", dim(color)),
    ])
}

fn right_pad(line: Line<'static>, width: usize) -> Line<'static> {
    let used: usize = line.spans.iter().map(|s| s.content.width()).sum();
    let mut spans = vec![Span::raw(" ".repeat(width.saturating_sub(used)))];
    spans.extend(line.spans);
    Line::from(spans)
}
