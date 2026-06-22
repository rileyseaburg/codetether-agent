//! Renders the LSP hover/JSDoc popup over the editor view.
//!
//! Draws a bordered, markdown-formatted box anchored to the bottom of `area`,
//! reusing [`MessageFormatter`] so JSDoc comments appear styled like VS Code.
//! Pure draw logic; the popup content lives in the app's editor LSP state.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

use crate::tui::message_formatter::MessageFormatter;

/// Draws the hover popup if `contents` is non-empty.
pub fn draw_hover(f: &mut Frame, area: Rect, contents: &str) {
    let popup = popup_rect(area);
    let width = popup.width.saturating_sub(2) as usize;
    let lines = MessageFormatter::new(width.max(10)).format_content(contents, "assistant");
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Documentation (Esc to close) ");
    f.render_widget(Clear, popup);
    f.render_widget(
        Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false }),
        popup,
    );
}

/// Computes the popup rectangle: bottom half, full width, inside `area`.
fn popup_rect(area: Rect) -> Rect {
    let height = (area.height / 2).clamp(3, area.height.saturating_sub(1));
    Rect {
        x: area.x,
        y: area.y + area.height.saturating_sub(height),
        width: area.width,
        height,
    }
}
