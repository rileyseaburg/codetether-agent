//! Spawn form panel contents.
//! Form state and terminal presentation require no application runtime.

use crate::{SpawnField, SpawnFormState, row::field_row};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub(crate) fn build_widget(form: &SpawnFormState) -> Paragraph<'static> {
    let area_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1); 4])
        .split(Rect::ZERO);
    let _ = area_split; // layout used only for constraint count documentation

    let lines = vec![
        Line::from(Span::styled(
            "  Tab: next field  ·  Enter: spawn  ·  Esc: cancel",
            Style::default().fg(Color::DarkGray),
        )),
        field_row("Name", &form.name, form.active == SpawnField::Name),
        field_row(
            "Instructions",
            &form.instructions,
            form.active == SpawnField::Instructions,
        ),
        Line::from(""),
    ];
    Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Spawn Agent ")
                .title_alignment(Alignment::Center)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false })
}
