use ratatui::{prelude::*, widgets::*};

pub fn block<T: Into<Line<'static>>>(title: T) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::default().fg(Color::DarkGray))
}
