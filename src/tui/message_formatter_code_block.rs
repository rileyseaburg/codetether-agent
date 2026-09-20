//! Frame a fenced code block, dispatching mermaid blocks to the diagram renderer.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::tui::chat::mermaid;

use super::message_formatter_code_render::render_code_body;

/// Render a fenced block: a mermaid diagram when possible, else a code box.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::message_formatter::message_formatter_code_block::render;
/// assert!(render(&["let x = 1;".to_string()], "rust", 40).len() >= 3);
/// ```
pub fn render(lines: &[String], language: &str, width: usize) -> Vec<Line<'static>> {
    if mermaid::is_mermaid(language)
        && let Some(diagram) = mermaid::render_block(&lines.join("\n"), width)
    {
        return diagram;
    }
    let mut result = vec![Line::from(Span::styled(
        header(language, width),
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    ))];
    result.extend(render_code_body(lines, language));
    result.push(Line::from(Span::styled(
        format!("└{}", "─".repeat(width.saturating_sub(1))),
        Style::default().fg(Color::DarkGray),
    )));
    result
}

fn header(language: &str, width: usize) -> String {
    let title = if language.is_empty() {
        "┌─ Code ─".to_string()
    } else {
        format!("┌─ {language} Code ─")
    };
    let pad = width.saturating_sub(title.chars().count());
    format!("{title}{}", "─".repeat(pad))
}
