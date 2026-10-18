//! Help images content and presentation.
use crate::blank::blank;
use crate::cmd_row::cmd_row;
use crate::heading::heading;
use crate::separator::separator;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
pub(crate) fn append(lines: &mut Vec<Line<'static>>) {
    // ── Images ──
    lines.push(heading("IMAGES"));
    lines.push(separator());
    if codetether_terminal_clipboard::is_ssh_or_headless() {
        lines.push(Line::from(Span::styled(
            "  SSH/headless session — clipboard unavailable",
            Style::default().fg(Color::Yellow),
        )));
        lines.push(blank());
        lines.push(Line::from(Span::raw(
            "  To paste an image without clipboard access:",
        )));
        lines.push(Line::from(Span::raw(
            "  1. Run `codetether clipboard image` where clipboard works",
        )));
        lines.push(Line::from(Span::raw(
            "  2. It copies a data:image URL to your clipboard",
        )));
        lines.push(Line::from(Span::raw(
            "  3. Paste it here with Ctrl+Shift+V or right-click paste",
        )));
        lines.push(blank());
    }
    lines.push(cmd_row("/image", "", "Attach image file from disk"));
    lines.push(cmd_row("/file", "", "Attach file contents to composer"));
    lines.push(blank());
}
