//! Watchdog notification UI widget.

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};

use crate::tui::app::state::AppState;

/// Render the watchdog notification popup centered in the chat area.
pub fn render_watchdog_notification(f: &mut Frame, area: Rect, state: &AppState) {
    let Some(ref notif) = state.watchdog_notification else {
        return;
    };
    let width = (area.width.min(60)).max(30);
    let height = 5u16;
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    let popup = Rect::new(x, y, width, height);

    let text = vec![
        Line::from(Span::styled(&notif.message, Style::default().fg(Color::Yellow))),
        Line::from(""),
        Line::from(vec![
            Span::styled("[Esc] Dismiss ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled("[Ctrl+X] Cancel", Style::default().fg(Color::Red)),
        ]),
    ];
    let para = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Watchdog"))
        .wrap(Wrap { trim: true });
    f.render_widget(Clear, popup);
    f.render_widget(para, popup);
}
