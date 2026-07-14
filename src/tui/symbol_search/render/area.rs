use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(percentages(height))
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(percentages(width))
        .split(vertical[1])[1]
}

fn percentages(value: u16) -> [Constraint; 3] {
    let margin = (100 - value) / 2;
    [
        Constraint::Percentage(margin),
        Constraint::Percentage(value),
        Constraint::Percentage(margin),
    ]
}
