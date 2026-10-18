//! footer lines for the goal-setup card.
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub(super) fn footer_line() -> Line<'static> {
    Line::from(Span::styled(
        "Enter = add  •  empty Enter = done  •  Esc = skip",
        Style::default().fg(Color::DarkGray),
    ))
}
